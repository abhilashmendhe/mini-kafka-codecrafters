use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::{net::TcpListener, sync::{mpsc, Mutex}};

use crate::{kafka_client_handler::{kread_handler, kwrite_handler}, kafka_errors::KafkaErrors};

pub type SharedConnectionMapT = Arc<Mutex<HashMap<u16, mpsc::UnboundedSender<(SocketAddr, Vec<u8>)>>>>;

pub async fn run_kafka_broker() -> Result<(), KafkaErrors> {

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    let connections: SharedConnectionMapT  = Arc::new(Mutex::new(HashMap::new()));
    // let (tx, rx) = mpsc::unbounded_channel();

     // Create ctrl_c future only once
    let mut shutdown_signal = Box::pin(tokio::signal::ctrl_c());
    loop 
    {
        tokio::select! {
            listener_result = listener.accept() => {
                match listener_result {
                    Ok((stream, sock_addr)) => {
                        
                        // Channel to send/recv commands
                        let (tx, rx) = mpsc::unbounded_channel();

                        // insert k-broker clients into the map
                        {
                            let mut connection_gaurd = connections.lock().await;
                            connection_gaurd.insert(sock_addr.port(),tx);
                        }

                        // Split the strea into reader and writer
                        let (reader, writer) = stream.into_split();

                        // Read data from kafka client
                        let connections1 = Arc::clone(&connections);
                        tokio::spawn(async move {
                            kread_handler(
                            reader,
                            sock_addr,
                            connections1).await
                        });

                        // Write response back to kafka client
                        let connections2 = Arc::clone(&connections);
                        tokio::spawn(async move {
                            kwrite_handler(
                                writer, 
                                rx, 
                                connections2).await
                        });
                    },
                    Err(e) => {
                        eprintln!("Accept error: {}", e)
                    },
                }
            },
            _ = &mut shutdown_signal => {
                println!("\nCtrl-c command received!");
                break;
            },
        }   
    }
    Ok(())
}
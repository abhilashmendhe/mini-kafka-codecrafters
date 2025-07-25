use std::net::SocketAddr;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc::{ UnboundedReceiver}};

use crate::{kafka_errors::KafkaErrors, kafka_wire_proto::kbroker_req_res::broker_req, run_kbroker::SharedConnectionMapT};

pub async fn kread_handler(
    mut reader: tokio::net::tcp::OwnedReadHalf,
    sock_addr: SocketAddr,
    connections: SharedConnectionMapT
) -> Result<(), KafkaErrors> {

    let mut read_buf = [0u8; 1024];
    loop {
        match reader.read(&mut read_buf).await {
            Ok(0) => {
                
                println!("Diconnecting: {}",sock_addr);
                connections.lock().await.remove(&sock_addr.port());
                println!("Removed connection : {}\n",sock_addr);

                return Ok(());
            },
            Ok(n) => {

                // broker_req(reader).await;
                println!("Read {} bytes kafka client.",n);
                println!("{:?}", &read_buf[..n]);
                println!("{}", String::from_utf8_lossy(&read_buf[..n]));
                
                let resp_bytes = broker_req(&read_buf[..n]).await;
                // println!("{}",resp_bytes.len());
                let mut write_buf = Vec::new();
                write_buf.extend_from_slice(&[0,0,0,19]);
                write_buf.extend_from_slice(&resp_bytes);
                respond_to_client(sock_addr, connections.clone(), &write_buf).await?;

            },
            Err(e) => {
                eprintln!("Read error: {}", e);
                connections.lock().await.remove(&sock_addr.port());
                return Err(KafkaErrors::IOError(e));
            },
        }
    }
}

pub async fn respond_to_client(
    sock_addr: SocketAddr,
    connections: SharedConnectionMapT,
    resp_bytes: &[u8]
) -> Result<(), KafkaErrors> {
    if let Some(kclient_tx) = connections.lock().await.get(&sock_addr.port()) {
        kclient_tx.send((sock_addr, resp_bytes.to_vec()))?;   
    }
    Ok(())
}

pub async fn kwrite_handler(
    mut writer: tokio::net::tcp::OwnedWriteHalf,
    mut rx: UnboundedReceiver<(SocketAddr, Vec<u8>)>,
    connections: SharedConnectionMapT
) {

    if let Some((_sock_addr, recv_bytes)) = rx.recv().await {
        match writer.write(&recv_bytes).await {
            Ok(0) => {
                return;
            }
            Ok(_) => {
                println!("Writing data back to client: {}", _sock_addr);
            },
            Err(e) => {
                println!("Error in write_handler: {e}");
                if e.kind() == std::io::ErrorKind::BrokenPipe {
                    connections.lock().await.remove(&_sock_addr.port());
                }
                // break;
                return;
            },
        }
    }
}
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

use crate::kafka_errors::KafkaErrors;


pub async fn run_kafka_broker() -> Result<(), KafkaErrors> {

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    
    let (stream, _sock_addr) = listener.accept().await?;

    let (mut reader, mut writer) = stream.into_split();

    let mut buffer = [0u8; 1024];

    let size = reader.read(&mut buffer).await?;

    println!("buffer: {:?}", &buffer[..size]);

    writer.write(&[0u8,0,0,0,0,0,0,07]).await?;
    Ok(())
}
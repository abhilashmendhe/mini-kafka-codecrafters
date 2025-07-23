use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

use crate::kafka_errors::KafkaErrors;


pub async fn run_kafka_broker() -> Result<(), KafkaErrors> {

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    loop {
        let (stream, _sock_addr) = listener.accept().await?;

        let (mut reader, mut writer) = stream.into_split();

        // let mut buffer = [0u8; 1024];

        let mut msg_buf = [0u8; 4];
        let _msg_n = reader.read_exact(&mut msg_buf).await?;

        let mut req_api_key = [0u8; 2];
        let _req_api_key_n = reader.read_exact(&mut req_api_key).await?;

        let mut req_api_ver = [0u8; 2];
        let _req_api_ver_n = reader.read_exact(&mut req_api_ver).await?;

        let mut corr_id = [0u8; 4];
        let _corr_id_n = reader.read_exact(&mut corr_id).await?;
        
        println!("corr_id(u8): {:?}", &corr_id);

        let mut write_buf = Vec::new();
        write_buf.extend_from_slice(&[0,0,0,0]);
        write_buf.extend_from_slice(&corr_id);
        writer.write(&write_buf).await?;
    }
    // Ok(())
}
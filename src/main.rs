use codecrafters_kafka::{kafka_errors::KafkaErrors, run_kbroker};

#[tokio::main]
async fn main() -> Result<(), KafkaErrors> {
    
    run_kbroker::run_kafka_broker().await?;
    Ok(())
}

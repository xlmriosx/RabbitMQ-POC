use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargar variables de entorno desde .env
    dotenvy::dotenv().ok();

    // Obtener credenciales desde variables de entorno
    let user = std::env::var("RABBITMQ_USER").unwrap_or_else(|_| "admin".to_string());
    let pass = std::env::var("RABBITMQ_PASS").unwrap_or_else(|_| "admin123".to_string());
    let host = std::env::var("RABBITMQ_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("RABBITMQ_PORT").unwrap_or_else(|_| "5672".to_string());

    // Construir la URL de conexión con vhost / (URL-encoded como %2F)
    let addr = format!("amqp://{}:{}@{}:{}/%2F", user, pass, host, port);

    // Conectar a RabbitMQ
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    // Declarar una cola
    let queue = "hello_queue";
    channel
        .queue_declare(
            queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Crear un consumidor
    let mut consumer = channel
        .basic_consume(
            queue,
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Esperando mensajes...");

    // Procesar mensajes
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        let payload = String::from_utf8_lossy(&delivery.data);
        println!("Recibido: {}", payload);

        // Confirmar el mensaje (ACK)
        channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await?;
    }

    Ok(())
}
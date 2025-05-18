use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, BasicProperties};
use serde_json::json;
use tokio::time::{sleep, Duration};

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

    // Enviar mensajes
    let mut count = 0;
    loop {
        let payload = json!({
            "message": format!("Hello from Rust #{}", count),
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        })
        .to_string();

        channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                payload.as_bytes(),
                BasicProperties::default(),
            )
        .await?;

        println!("Enviado: {}", payload);
        count += 1;
        sleep(Duration::from_secs(1)).await;
    }
}
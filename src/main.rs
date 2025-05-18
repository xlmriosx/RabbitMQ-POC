use axum::{routing::post, Json, Router};
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, BasicProperties};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    channel: Arc<Mutex<lapin::Channel>>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

async fn send_message(
    state: axum::extract::State<AppState>,
    Json(message): Json<Message>,
) -> Result<String, String> {
    let channel = state.channel.lock().await;
    let queue = "hello_queue";

    channel
        .basic_publish(
            "",
            queue,
            BasicPublishOptions::default(),
            message.content.as_bytes(),
            BasicProperties::default(),
        )
        .await
        .map_err(|e| format!("Error al enviar mensaje: {}", e))?;

    Ok(format!("Mensaje enviado: {}", message.content))
}

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

    // Configurar estado de la aplicación
    let app_state = AppState {
        channel: Arc::new(Mutex::new(channel)),
    };

    // Crear el router de Axum
    let app = Router::new()
        .route("/send", post(send_message))
        .with_state(app_state);

    // Iniciar el servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("API escuchando en http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
# RustRabbitPOC

A Proof of Concept (POC) demonstrating a message queue system using **RabbitMQ** with a **Rust**-based API, producer, and consumer. This project showcases asynchronous message processing with RabbitMQ, integrated with a RESTful API built using Axum in Rust.

## Table of Contents
- [Introduction to Message Queues](#introduction-to-message-queues)
- [Project Overview](#project-overview)
- [Prerequisites](#prerequisites)
- [Setup Instructions](#setup-instructions)
- [Running the Application](#running-the-application)
- [Testing the Application](#testing-the-application)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Introduction to Message Queues

A **Message Queue (MQ)** is a system that enables asynchronous communication between applications by allowing them to send and receive messages via a queue. This decouples producers (senders) from consumers (receivers), improving scalability, reliability, and fault tolerance in distributed systems.

### Key Concepts
- **Message**: A piece of data (e.g., JSON, text) sent from a producer to a consumer.
- **Queue**: A buffer that stores messages until they are processed by a consumer.
- **Producer**: An application or component that sends messages to a queue.
- **Consumer**: An application or component that retrieves and processes messages from a queue.
- **Broker**: The server (e.g., RabbitMQ) that manages queues, routes messages, and ensures delivery.
- **Exchange**: A RabbitMQ component that routes messages to queues based on rules (e.g., direct, fanout, topic).
- **AMQP**: The Advanced Message Queuing Protocol used by RabbitMQ for communication.

### Benefits of Message Queues
- **Asynchronous Processing**: Producers and consumers operate independently, allowing tasks to be processed later.
- **Scalability**: Multiple consumers can process messages in parallel.
- **Reliability**: Messages are stored until successfully processed, reducing data loss.
- **Decoupling**: Producers and consumers don’t need to know each other’s details.

In this project, **RabbitMQ** is used as the message broker, leveraging its robust support for AMQP to handle message queuing.

## Project Overview

**RustRabbitPOC** demonstrates a simple message queue system with three components:
1. **API**: A RESTful API built with Axum that sends messages to a RabbitMQ queue (`hello_queue`) via a POST endpoint.
2. **Producer**: A Rust program that periodically sends JSON messages to `hello_queue`.
3. **Consumer**: A Rust program that listens for messages in `hello_queue` and prints them.

The project uses:
- **Rust** for the API, producer, and consumer.
- **RabbitMQ** as the message broker.
- **Axum** for the HTTP server.
- **Lapin** (a Rust AMQP client) to interact with RabbitMQ.
- **Tokio** for asynchronous runtime.
- **dotenvy** to load RabbitMQ credentials from environment variables.

## Prerequisites

To run this project, you need:
- **Rust**: Install via [rustup](https://rustup.rs/) (run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` on Unix-like systems or follow instructions for Windows).
- **Docker**: For running RabbitMQ (install [Docker Desktop](https://www.docker.com/products/docker-desktop/) on Windows/macOS or `docker` on Linux).
- **Git**: To clone the repository (optional).
- **curl**: For testing the API (available by default on most systems or install via `apt`, `brew`, or similar).

Verify installations:
```bash
rustc --version  # Should show 1.80 or later
docker --version # Should show Docker is installed
curl --version   # Should show curl is available
```

## Setup Instructions

1. **Clone the Repository** (if not already done):
   ```bash
   git clone https://github.com/xlmriosx/RabbitMQ-POC.git
   cd RabbitMQ-POC
   ```
   On Windows, if working from `C:\git\RabbitMQ-POC`, navigate to:
   ```bash
   cd C:\git\RabbitMQ-POC
   ```

2. **Run RabbitMQ with Docker**:
   Start a RabbitMQ container with the management interface:
   ```bash
   docker run -d --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management
   ```

3. **Configure RabbitMQ User**:
   Create an `admin` user with password `admin123` (or your preferred credentials):
   ```bash
   docker exec -it rabbitmq bash
   rabbitmqctl add_user admin admin123
   rabbitmqctl set_user_tags admin administrator
   rabbitmqctl set_permissions -p / admin ".*" ".*" ".*"
   exit
   ```
   Verify RabbitMQ is accessible at `http://localhost:15672` (login with your credentials).

4. **Configure Environment Variables**:
   Create a `.env` file in the project root based on `.env.example`:
   ```bash
   cp .env.example .env
   ```
   Edit `.env` to set your RabbitMQ credentials (default values shown):
   ```env
   RABBITMQ_USER=admin
   RABBITMQ_PASS=admin123
   RABBITMQ_HOST=localhost
   RABBITMQ_PORT=5672
   ```
   Alternatively, set these variables in your environment (e.g., via `export` on Unix or `set` on Windows).

5. **Install Rust Dependencies**:
   Compile the project dependencies:
   ```bash
   cargo build
   ```

## Running the Application

The project includes three components: the API, producer, and consumer. Each runs as a separate binary.

1. **Run the Consumer**:
   In a terminal, start the consumer to listen for messages in `hello_queue`:
   ```bash
   cargo run --bin consumer
   ```
   Output: `Esperando mensajes...`

2. **Run the API**:
   In another terminal, start the Axum-based API:
   ```bash
   cargo run --bin rabbitmq-rust-api
   ```
   Output: `API escuchando en http://localhost:3000`

3. **Run the Producer** (Optional):
   In a third terminal, start the producer to send periodic messages:
   ```bash
   cargo run --bin producer
   ```
   Output: `Enviado: {"message":"Hello from Rust #0","timestamp":...}` (every second)

## Testing the Application

1. **Test the API**:
   Send a message to the `hello_queue` via the API:
   ```bash
   curl -X POST http://localhost:3000/send -H "Content-Type: application/json" -d '{"content":"Hola desde la API"}'
   ```
   - Expected API response: `Mensaje enviado: Hola desde la API`
   - Check the consumer terminal; it should print: `Recibido: Hola desde la API`

2. **Test the Producer**:
   If the producer is running, the consumer should print messages like:
   ```bash
   Recibido: {"message":"Hello from Rust #0","timestamp":1745097600}
   Recibido: {"message":"Hello from Rust #1","timestamp":1745097601}
   ```

3. **Monitor RabbitMQ**:
   - Open `http://localhost:15672` (login with your RabbitMQ credentials).
   - Go to the **Queues** tab and select `hello_queue` to see message counts and activity.
   - Check the **Connections** tab to verify the Rust application is connected.

## Troubleshooting

- **Consumer Error: `ACCESS_REFUSED`**:
  - Ensure the credentials in `.env` (or environment variables) match those set in RabbitMQ.
  - Verify RabbitMQ is running: `docker ps`.
  - Check logs: `docker logs rabbitmq`.

- **API or Producer Fails to Connect**:
  - Confirm the RabbitMQ credentials and connection details in `.env`.
  - Ensure ports `5672` and `15672` are not blocked (use `netstat -a` on Windows).

- **Compilation Errors**:
  - Run `cargo clean` and `cargo build` to clear cached artifacts.
  - Verify Rust is up-to-date: `rustup update`.

- **Windows-Specific Issues**:
  - Ensure Docker Desktop is running.
  - Use PowerShell or CMD for commands.
  - If paths cause issues, navigate using `cd C:\git\RabbitMQ-POC`.

- **No Messages Received**:
  - Confirm the queue name (`hello_queue`) is consistent across API, producer, and consumer.
  - Check RabbitMQ’s management UI for queue activity.

For further help, check the [RabbitMQ documentation](https://www.rabbitmq.com/docs) or open an issue in this repository.

## Contributing

Contributions are welcome! To contribute:
1. Fork the repository.
2. Create a branch: `git checkout -b feature/your-feature`.
3. Commit changes: `git commit -m "Add your feature"`.
4. Push to your fork: `git push origin feature/your-feature`.
5. Open a pull request.

Please include tests and update documentation as needed.

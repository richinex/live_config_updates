
# Dynamic Configuration Management Server

This server is designed to manage and update configurations dynamically for applications, using Actix Web, Tokio, and a structured logging mechanism. It allows for real-time configuration updates and fetching the current configuration state through a RESTful API. A key feature of this server is the use of Tokio channels for efficient, non-blocking communication between different parts of the application.

## Features

- **Dynamic Configuration Updates**: Partially update the application's configuration in real-time using Tokio channels.
- **Real-Time Configuration Fetching**: Retrieve the current configuration state through a simple API call.
- **Robust Logging**: Utilizes both console and JSON logging for comprehensive monitoring.
- **Asynchronous Communication**: Leverages Tokio channels to asynchronously communicate configuration updates across the application.

## Using Tokio Channels

Tokio channels are used to facilitate non-blocking communication between the HTTP server handling API requests and the internal components that manage the application's configuration state. When a configuration update is received via the `/update-config` endpoint, the update is sent through a Tokio channel to the component responsible for applying these updates. This approach allows the application to remain responsive and efficiently process incoming configuration changes.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine.
- Basic knowledge of Rust and asynchronous programming with Tokio.

### Setup

Clone the repository to your local machine:

```bash
git clone https://github.com/richinex/live_config_updates.git
cd live_config_updates
```

### Running the Server

To start the server, run:

```bash
cargo run
```

The server will start listening on `127.0.0.1:8080`, ready to accept requests.

## API Endpoints

### Update Configuration

- **POST** `/update-config`

  Accepts partial configuration updates in JSON format. Example request body:

  ```json
  {
    "ball_color": "red",
    "ball_size": 20,
    "ball_speed": 10,
    "number_of_balls": 30
  }
  ```

### Fetch Current Configuration

- **GET** `/config`

  Returns the current configuration in JSON format.

## Logging

The server uses dual logging mechanisms, outputting both to the console and as JSON. The logging level and specific details can be adjusted in the `configure_logging` function.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for any improvements or feature requests.


# RCP - Rust CORS Proxy

RCP (Rust CORS Proxy) is a lightweight HTTP proxy server implemented in Rust that enables Cross-Origin Resource Sharing (CORS) for web applications. It allows you to make cross-origin requests to APIs or resources that would otherwise be blocked due to browser security policies.

## Features

- Enables CORS for web applications by acting as a proxy server.
- Supports HTTP GET, POST, PUT, DELETE, and OPTIONS methods.
- Automatically adds appropriate CORS headers to the proxied responses.
- Provides a simple and straightforward implementation.

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) on your machine.
2. Clone this repository.
```bash
git clone https://github.com/f-str/rcp.git
```
3. Build the project.
```bash
cd rcp
cargo build --release
```

## Usage

1. Start the RCP server.
```bash
cargo run --release
```

2. RCP will start listening on `0.0.0.0:8080` by default.

3. Make requests to the RCP server by replacing the original URL with the RCP URL.
```bash 
http://localhost:8080/original-url
```
For example, to proxy `https://api.example.com/data`, you would make a request to `http://localhost:8080/https://api.example.com/data`.

4. RCP will forward the request to the original URL and return the response with the appropriate CORS headers.

## Configuration

RCP can be configured using environment variables:

- `LOGGING_ENABLED`: Set to `"true"` to enable logging (default: `false`).
- `PORT`: Set the port that RCP listens on (default: `8080`).
- `ADDRESS`: Set the address that RCP listens on (default: `0.0.0.0`). 

## Contributing

Contributions to RCP are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request. Make sure to follow the existing code style and provide clear commit messages.

## License

This project is licensed under the MIT License.

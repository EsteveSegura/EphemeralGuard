# Secret db client

EphemeralGuard CLI is a command-line interface tool for managing secrets securely with expiration capabilities. This tool allows you to create, read, and delete secrets via a ZeroMQ server, making it ideal for scenarios where ephemeral or temporary secrets are required.

## Features

- **Create Secrets:** Add a new secret with a specified expiration time.
- **Read Secrets:** Retrieve a stored secret using its ID and associated credentials.
- **Delete Secrets:** Remove a secret by its ID.

## Requirements

- Rust (latest stable version)
- JSON processing (`serde_json` crate)

## Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/EsteveSegura/EphemeralGuard.git
   cd ./EphemeralGuard/secret_db_client
   ```

2. **Build the project:**

   ```bash
   cargo build --release
   ```

## Usage

### General Command Structure

```bash
cargo run -- <command> [options]
```

### Commands

#### 1. Add a Secret

Creates a new secret that will expire after a specified number of seconds.

```bash
cargo run -- add_secret -p "SECRET_PAYLOAD" -t "EXPIRATION_TIME_IN_SECONDS"
```

- **-p, --payload**: The secret data to be stored.
- **-t, --time**: Time in seconds until the secret expires.

**Example:**

```bash
cargo run -- add_secret -p "my_secret_data" -t 20
```

#### 2. Read a Secret

Reads a secret using its ID and the associated credentials.

```bash
cargo run -- read_secret -i "SECRET_ID" -c "CREDENTIALS"
```

- **-i, --id**: The ID of the secret.
- **-c, --credentials**: The credentials required to access the secret.

**Example:**

```bash
cargo run -- read_secret -i "SECRET_ID" -c "my_credentials"
```

#### 3. Delete a Secret

Deletes a secret using its ID.

```bash
cargo run -- delete_secret -i "SECRET_ID"
```

- **-i, --id**: The ID of the secret to be deleted.

**Example:**

```bash
cargo run -- delete_secret -i "12345"
```

## Example Responses

- **Create Secret:**

  ```json
  {
    "id": "generated_id",
    "credentials": "generated_credentials"
  }
  ```

- **Read Secret:**

  ```json
  {
    "payload": "my_secret_data"
  }
  ```

- **Delete Secret:**

  ```json
  {
    "status": true
  }
  ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

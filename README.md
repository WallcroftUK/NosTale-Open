# NosTale-Open

## Overview

NosTale-Open is an open-source implementation of the game **NosTale**, aiming to recreate the gameplay and server functionalities of the original game. This project is structured as a Rust workspace containing multiple components that work together to form the complete server architecture.

## Project Structure

The repository consists of the following main components:

- **database_server**: (To be implemented) Manages data persistence for user accounts, game states, and logs.
- **login_server**: Handles user authentication and session management.
- **master_server**: Coordinates between various game servers.
- **world_server**: (To be implemented) Manages game world logic, player interactions, and in-game events.

### Workspace Configuration

The project is configured as a Rust workspace, which allows for easier management of multiple interdependent packages. The `Cargo.toml` at the root level defines the workspace:

```toml
[workspace]
members = [
    "database_server",
    "login_server",
    "master_server",
    # "world_server",
]
```

## Requirements

- Rust programming language (version 1.58 or later)
- PostgreSQL (for database support)
- MongoDB (for log storage)
- Docker ( for containerizing the database)

## Getting Started

### Cloning the Repository

```bash
git clone https://github.com/yourusername/NosTale-Open.git
cd NosTale-Open
```

### Setting Up the Environment

1. **Install Rust**: Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2. **Set Up Docker**: If using Docker for your database services, ensure Docker is installed and running on your machine.

### Database Configuration

If you decide to include the database server, configure the PostgreSQL and MongoDB instances. You can use Docker to run these databases:

```bash
docker run --name postgres -e POSTGRES_USER=user -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres
docker run --name mongodb -p 27017:27017 -d mongo
```

### Building the Workspace

To build the entire workspace, run the following command from the root of the repository:

```bash
cargo build
```

### Running the Servers

To run the servers, execute the following commands in separate terminal windows:

#### Login Server

```bash
cd login_server
cargo run
```

#### Master Server

```bash
cd master_server
cargo run
```

#### Database Server

```bash
cd database_server
cargo run
```

### Note on World Server

The world server is not currently implemented. This section can be uncommented and developed in the future as needed.

## Contributing

Contributions are welcome! Please read our [CONTRIBUTING](CONTRIBUTING) for details on the code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For questions or support, please open an issue in the repository or contact the maintainers.

---

**Enjoy your journey in NosTale-Open!**

# Lightning Nodes API

A Rust API server that fetches Lightning Network node data from mempool.space and serves it via HTTP endpoints.

## Build tools & versions used

- Rust 1.87.0
- Axum 0.8.4 (web framework)
- SQLx 0.8.6 (database client)
- PostgreSQL 15 (database)
- Docker (for database)
- Tokio 1.45.0 (async runtime)
- Reqwest 0.12.15 (HTTP client)
- Serde 1.0.219 (serialization)

## Prerequisites

Before running the application, make sure you have the following installed:

1. **Rust** (latest stable version)

2. **Docker** (for PostgreSQL)

3. **SQLx CLI** (for database migrations):

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## Steps to run the app

1. **Clone the repository**:

```bash
git clone https://github.com/marinhotg/lightning-nodes-api
cd lightning_nodes_api
```

2. **Set database URL**:

```bash
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lightning_nodes"
```

3. **Install Rust dependencies**:

```bash
cargo build
```

4. **Start PostgreSQL database**:

```bash
chmod +x start_db.sh
./start_db.sh
```

5. **Run database migrations**:

```bash
sqlx migrate run
```

6. **Start the server**:

```bash
cargo run
```

The server will be available at `http://localhost:3000` with the following endpoints:

- `GET /fetch` - Test API connection
- `GET /save` - Import nodes from mempool.space to database
- `GET /nodes` - Return formatted node data from database

## What was the reason for your focus? What problems were you trying to solve?

The focus was to build a simple and functional solution that ensures the integrity of the nodes. In the client request, the top 100 nodes with the most associated channels were received and saved to the local database. However, nodes that dropped out of this ranking still had their records preserved in the database. So, I focused on creating a straightforward solution to address this issue. I implemented an upsert mechanism combined with a cleanup process based on the last update timestamp. This way, the database always reflects the top 100 ranked nodes with their most up-to-date information according to the Lightning Network.

## How long did you spend on this project?

Approximately 10 hours. Most of the time was spent researching libraries and studying their implementations to solve specific issues in the code.

## Did you make any trade-offs for this project? What would you have done differently with more time?

The main trade-off was in error handling. Due to time constraints, I implemented a minimal solution focused on maintaining service execution integrity. With more time, I would have implemented a more robust error-handling system, using more detailed error types and better distinguishing between different kinds of failures. This would make the code easier to understand, debug, and maintain.

## What do you think is the weakest part of your project?

The weakest part was probably the error handling. I went with a basic solution that worked, but with more experience or time, I would have built something more structured and detailed. It’s definitely an area I’d like to keep improving as I learn more about best practices.

## Is there any other information you'd like us to know?

I'm still a beginner in Rust, so I spent more time reading documentation than actually coding. Because of that, I chose to use the most popular libraries with strong community support and good documentation to help me complete this project effectively.

Even though I'm just starting out, I really enjoy the language and its core principles, and I'm very motivated to keep learning and improving my skills in Rust.

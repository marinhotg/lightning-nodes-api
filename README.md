# Lightning Nodes API

A Rust API server that fetches Lightning Network node data from mempool.space and serves it via HTTP endpoints.

## Build tools & versions used

- Rust 1.87.
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

2. **Install Rust dependencies**:

```bash
cargo build
```

3. **Start PostgreSQL database**:

```bash
chmod +x start_db.sh
./start_db.sh
```

4. **Run database migrations**:

```bash
sqlx migrate run
```

5. **Start the server**:

```bash
cargo run
```

The server will be available at `http://localhost:3000` with the following endpoints:

- `GET /fetch` - Test API connection
- `GET /save` - Import nodes from mempool.space to database
- `GET /nodes` - Return formatted node data from database

## Environment Variables

Create a `.env` file in the project root with:

```
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/lightning_nodes
```

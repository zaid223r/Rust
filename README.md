# Rust Backend API

A modern, secure REST API built with Rust using Axum web framework, featuring user authentication with JWT tokens and CRUD operations for posts.

## 🚀 Features

- **User Authentication**: JWT-based authentication with secure password hashing using bcrypt
- **User Management**: User registration and login with email validation
- **Post Management**: Full CRUD operations for user posts
- **Database Integration**: PostgreSQL with SQLx for type-safe database operations
- **Security**: CORS enabled, password hashing, JWT token validation
- **Structured Logging**: Comprehensive logging with tracing
- **Database Migrations**: SQLx migrations for database schema management

## 🛠️ Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Modern async web framework
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) for async database operations
- **Authentication**: JWT tokens with [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- **Password Hashing**: [bcrypt](https://github.com/Keats/rust-bcrypt)
- **Serialization**: [Serde](https://github.com/serde-rs/serde) for JSON serialization/deserialization
- **Logging**: [Tracing](https://github.com/tokio-rs/tracing) for structured logging
- **UUID**: [uuid](https://github.com/uuid-rs/uuid) for unique identifiers
- **Date/Time**: [Chrono](https://github.com/chronotope/chrono) for date and time handling

## 📁 Project Structure

```
rust_backend_api/
├── src/
│   ├── config/          # Configuration management
│   ├── database/        # Database connection and setup
│   ├── handlers/        # Request handlers (auth, posts)
│   ├── middleware/      # Custom middleware (JWT auth)
│   ├── models/          # Data models and DTOs
│   └── main.rs          # Application entry point
├── migrations/          # Database migrations
├── api_docs/           # API documentation (Postman collection)
└── Cargo.toml          # Dependencies and project configuration
```

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- PostgreSQL 12+
- SQLx CLI (`cargo install sqlx-cli`)

### Installation

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd rust_backend_api
   ```

2. **Set up environment variables**
   Create a `.env` file in the project root:
   ```env
   DATABASE_URL=postgresql://username:password@localhost/database_name
   JWT_SECRET=your-super-secret-jwt-key-here
   SERVER_PORT=3000
   ```

3. **Set up the database**
   ```bash
   # Create the database
   createdb your_database_name
   
   # Run migrations
   sqlx migrate run
   ```

4. **Install dependencies and run**
   ```bash
   cargo run
   ```

The server will start on `http://localhost:3000` (or your configured port).

### Using Postman

Import the provided Postman collection from `api_docs/Rust.postman_collection.json` for a complete set of pre-configured requests.

## 🚀 Development

### Running in Development Mode

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run with specific log level
RUST_LOG=info cargo run
```

### Database Migrations

```bash
# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Building for Production

```bash
# Build optimized release
cargo build --release

# Run the release binary
./target/release/rust_backend_api
```

## 📝 Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DATABASE_URL` | PostgreSQL connection string | - | Yes |
| `JWT_SECRET` | Secret key for JWT token signing | - | Yes |
| `SERVER_PORT` | Port to run the server on | 3000 | No |

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Troubleshooting

### Common Issues

1. **Database Connection Errors**
   - Ensure PostgreSQL is running
   - Check your `DATABASE_URL` in the `.env` file
   - Verify database exists and migrations are applied

2. **JWT Token Issues**
   - Ensure `JWT_SECRET` is set in environment variables
   - Check token expiration (30 days default)
   - Verify Authorization header format: `Bearer <token>`

3. **Port Already in Use**
   - Change `SERVER_PORT` in `.env` file
   - Or kill the process using the port: `lsof -ti:3000 | xargs kill -9`

### Debug Mode

Run with debug logging to see detailed error information:
```bash
RUST_LOG=debug cargo run
```

---

<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos + Actix + Diesel + PostgreSQL + Tailwind Starter

A modern, full-stack web application starter template built with Rust, featuring server-side rendering, type-safe database access, and a beautiful Tailwind CSS UI.

## Tech Stack

- **[Leptos](https://github.com/leptos-rs/leptos)** - Reactive web framework for Rust with SSR and hydration
- **[Actix Web](https://actix.rs/)** - Fast, ergonomic web framework for the server
- **[Diesel](https://diesel.rs/)** - Type-safe ORM and query builder for PostgreSQL
- **[PostgreSQL](https://www.postgresql.org/)** - Powerful, open-source relational database
- **[Tailwind CSS](https://tailwindcss.com/)** - Utility-first CSS framework

## Features

- Server-side rendering (SSR) with hydration
- Type-safe database operations with Diesel ORM
- PostgreSQL integration with migrations
- Tailwind CSS for styling
- Hot-reload development experience
- Production-optimized WASM builds

## Prerequisites

Make sure you have the following installed:

- [Rust](https://rustup.rs/) (nightly toolchain)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Node.js](https://nodejs.org/) and npm (for Tailwind CSS)
- [Diesel CLI](https://diesel.rs/guides/getting-started) with PostgreSQL support

### Installation Commands

```bash
# Install Rust nightly
rustup toolchain install nightly --allow-downgrade

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install cargo-leptos
cargo install cargo-leptos --locked

# Install Diesel CLI with PostgreSQL support
cargo install diesel_cli --no-default-features --features postgres

# Install Node dependencies (Tailwind CSS)
npm install
```

## Getting Started

### 1. Setup PostgreSQL Database

Create a PostgreSQL database and update the `.env` file:

```bash
# .env
DATABASE_URL=postgres://postgres:postgres@localhost/postgres
```

### 2. Run Migrations

```bash
diesel migration run
```

This will set up the database schema, including the `users` table.

### 3. Build Tailwind CSS

```bash
# Build CSS once
npm run build:css

# Or watch for changes
npm run watch:css
```

### 4. Run the Development Server

```bash
cargo leptos watch
```

The application will be available at `http://localhost:3000`

## Project Structure

```
.
├── src/
│   ├── app.rs       # Main application component and routes
│   ├── lib.rs       # Library entry point
│   ├── main.rs      # Server entry point
│   ├── db.rs        # Database connection setup
│   ├── models.rs    # Diesel models
│   └── schema.rs    # Auto-generated database schema
├── migrations/      # Diesel database migrations
├── assets/          # Static assets
├── style/           # Tailwind CSS source files
├── end2end/         # Playwright tests
├── .env             # Environment variables
└── Cargo.toml       # Rust dependencies and configuration
```

## Available Scripts

```bash
# Development with hot-reload
cargo leptos watch

# Build for production
cargo leptos build --release

# Run tests
cargo test

# Run end-to-end tests
cargo leptos end-to-end

# Build Tailwind CSS
npm run build:css

# Watch Tailwind CSS for changes
npm run watch:css

# Create a new Diesel migration
diesel migration generate <migration_name>

# Run pending migrations
diesel migration run

# Revert the last migration
diesel migration revert
```

## Database

This template includes a basic `users` table schema:

```sql
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

You can create additional migrations using:

```bash
diesel migration generate create_your_table
```

## Production Deployment

After running `cargo leptos build --release`, you'll need:

1. The server binary from `target/server/release/leptos-actix-diesel-postgres-tailwind`
2. The `target/site` directory with all static assets

Set these environment variables on your server:

```bash
export LEPTOS_OUTPUT_NAME="leptos-actix-diesel-postgres-tailwind"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
export DATABASE_URL="postgres://user:password@localhost/database"
```

Then run the binary and ensure PostgreSQL is accessible.

## Development Tips

- Keep `npm run watch:css` running alongside `cargo leptos watch` during development
- Run `diesel migration run` after pulling new migrations from git
- The app uses SSR by default; check `src/app.rs` to customize routes
- Database models are in `src/models.rs` and auto-sync with `src/schema.rs`

## CSR Mode (Client-Side Rendering Only)

For static site generation or tools like Tauri:

```bash
trunk serve --open --features csr
```

Note: This bypasses the server and database features.

## License

This project is released under the MIT License. Feel free to use it as a starting point for your own applications.
# Starter-projek-with-rust
# Restaurant_Ordering_Sistem

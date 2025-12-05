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

## Architecture Overview

This application follows a clean, modular architecture with clear separation of concerns:

### Frontend (Leptos)
- **Components**: Reusable UI components (`src/components/`)
  - `Navigation` - Site navigation with cart count and auth state
  - `ProductCard` - Product display component
  - `LoginForm` - Authentication form
  - `ToastContainer` - Notification system
- **Contexts**: Global state management (`src/contexts/`)
  - `AuthContext` - User authentication state
  - `CartContext` - Shopping cart state
  - `ToastContext` - Notification state
- **Services**: API communication layer (`src/services/`)
  - `AuthService` - Authentication operations
  - `ProductService` - Product and category management
  - `CartService` - Shopping cart operations
  - `ApiClient` - HTTP client with error handling

### Backend (Actix Web + Diesel)
- **Handlers**: API endpoint implementations (`src/handlers/`)
  - `auth.rs` - User registration and login
  - `products.rs` - Product and category CRUD operations
  - `cart.rs` - Shopping cart management
  - `orders.rs` - Order processing
  - `middleware.rs` - Authentication middleware
- **Types**: Shared type definitions (`src/types/`)
  - `auth.rs` - Authentication types (Claims, User, AuthResponse)
  - `products.rs` - Product and category types
  - `cart.rs` - Shopping cart types
  - `orders.rs` - Order management types

### Database Layer
- **Models**: Diesel ORM models (`src/models.rs`)
- **Schema**: Auto-generated database schema (`src/schema.rs`)
- **Migrations**: Database schema evolution (`migrations/`)

## Project Structure

```
.
├── src/
│   ├── app.rs           # Main application component and routes
│   ├── lib.rs           # Library entry point
│   ├── main.rs          # Server entry point
│   ├── db.rs            # Database connection setup
│   ├── models.rs        # Diesel ORM models
│   ├── schema.rs        # Auto-generated database schema
│   ├── components/      # Reusable UI components
│   │   ├── mod.rs
│   │   ├── navigation.rs
│   │   ├── product_card.rs
│   │   ├── login_form.rs
│   │   └── toast_container.rs
│   ├── contexts/        # Global state management
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── cart.rs
│   │   └── toast.rs
│   ├── services/        # API communication layer
│   │   ├── mod.rs
│   │   ├── api.rs
│   │   ├── auth.rs
│   │   ├── products.rs
│   │   └── cart.rs
│   ├── types/           # Shared type definitions
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── products.rs
│   │   ├── cart.rs
│   │   └── orders.rs
│   └── handlers/        # API endpoint implementations
│       ├── mod.rs
│       ├── auth.rs
│       ├── products.rs
│       ├── cart.rs
│       ├── orders.rs
│       └── middleware.rs
├── migrations/          # Diesel database migrations
├── assets/              # Static assets
├── style/               # Tailwind CSS source files
├── end2end/             # Playwright end-to-end tests
├── tests/               # Integration tests
├── .env                 # Environment variables
└── Cargo.toml           # Rust dependencies and configuration
```

## Testing

The application includes comprehensive testing:

### Unit Tests
```bash
# Run all unit tests (type serialization tests)
cargo test

# Run specific test module
cargo test types::tests
```

### Integration Tests
```bash
# Run integration tests (API endpoint tests)
cargo test --test integration_tests
```

### End-to-End Tests
```bash
# Run Playwright end-to-end tests
cargo leptos end-to-end
```

## Available Scripts

```bash
# Development with hot-reload
cargo leptos watch

# Build for production
cargo leptos build --release

# Run all tests
cargo test

# Run integration tests only
cargo test --test integration_tests

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

## API Endpoints

The application provides RESTful API endpoints:

### Authentication
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login

### Products & Categories
- `GET /api/products` - List products (with optional filtering)
- `GET /api/products/{id}` - Get specific product
- `POST /api/products` - Create product (admin)
- `GET /api/categories` - List categories
- `POST /api/categories` - Create category (admin)

### Shopping Cart (Authenticated)
- `GET /api/cart` - Get user's cart
- `POST /api/cart` - Add item to cart
- `PUT /api/cart/{id}` - Update cart item quantity
- `DELETE /api/cart/{id}` - Remove item from cart
- `DELETE /api/cart` - Clear entire cart

### Orders (Authenticated)
- `POST /api/orders` - Create order from cart
- `GET /api/orders` - Get user's orders

## Database Schema

The application includes the following tables:

- **users**: User accounts with authentication
- **categories**: Product categories
- **products**: Product catalog
- **cart_items**: Shopping cart items
- **orders**: Order records
- **order_items**: Order line items

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
# Online_Store
# Online_Store
# Online_Store

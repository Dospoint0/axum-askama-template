# Axum + Askama Website Template

This is a feature-rich template for jumpstarting website development with Rust, using the Axum web framework, Askama for templating, and SQLx for database interaction.

## Features

-   [Axum](https://github.com/tokio-rs/axum) for robust and modular web services.
-   [Askama](https://github.com/djc/askama) for type-safe, Jinja-like templates.
-   [SQLx](https://github.com/launchbadge/sqlx) for asynchronous, compile-time checked SQL queries.
-   [SQLite](https://www.sqlite.org/index.html) for a simple, file-based database (easy to swap for Postgres or MySQL).
-   [Tokio](https://tokio.rs/) for an efficient, asynchronous runtime.
-   [Tower-http](https://github.com/tower-rs/tower-http) for serving static files and request tracing.
-   [Tracing](https://github.com/tokio-rs/tracing) for application-level logging.
-   [dotenvy](https://github.com/allan2/dotenvy) for environment variable management.
-   [validator](https://github.com/Keats/validator) for struct-level data validation.
-   [bcrypt](https://github.com/Keats/bcrypt) for secure password hashing.
-   A ready-to-use user signup flow with server-side and client-side validation.
-   A clean, modular project structure to build upon.

## Getting Started

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) toolchain
-   (Optional) [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) for managing database migrations manually.
    ```bash
    cargo install sqlx-cli
    ```

### Running the Application

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/naifl/axum-askama-template.git
    cd axum-askama-template
    ```

2.  **Set up environment variables:**
    Copy the example environment file. This file contains the default configuration for the server address and database connection.
    ```bash
    cp .env.example .env
    ```

3.  **Run the application:**
    ```bash
    cargo run
    ```
    The server will start, automatically create the `database/users.db` file if it doesn't exist, and run the necessary database migrations.

4.  **Open in your browser:**
    Navigate to `http://127.0.0.1:8000`. You should see the home page. Try navigating to `/signup` to test the user registration form.

## Configuration

The application is configured via environment variables defined in the `.env` file.

-   `LISTEN_ADDR`: The IP address and port the server listens on. Default: `127.0.0.1:8000`.
-   `DATABASE_URL`: The connection string for the database. Default: `sqlite:database/users.db`.

## Project Structure

```
.
├── Cargo.toml
├── static/                # Static assets (CSS, JS, images)
│   └── css/
│       └── global.css
├── src/
│   ├── handlers/          # Request handlers
│   │   ├── mod.rs
│   │   └── public.rs
│   ├── models/            # Data models and templates
│   │   ├── mod.rs
│   │   └── templates.rs
│   ├── state.rs           # Shared application state
│   ├── util.rs            # Utility functions/structs
│   ├── routes.rs          # Route definitions
│   ├── lib.rs             # Library root
│   └── main.rs            # Binary entrypoint
└── templates/             # Askama templates
    ├── base.html          # Base layout template
    └── pages/
        ├── home.html
        ├── notfound.html
        └── server-error.html
```

## Getting Started

1.  Clone this repository.
2.  Run `cargo run` to start the development server.
3.  Open `http://127.0.0.1:8000` in your browser.

## Customization

-   **Add new routes**: Add a new handler in `src/handlers/` and register it in `src/routes.rs`.
-   **Add new templates**: Create a new `.html` file in `templates/pages/` and a corresponding struct in `src/models/templates.rs`.
-   **Shared State**: Modify `src/state.rs` to add any shared state your application needs. The `view_count` is provided as an example.

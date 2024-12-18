# Rust Axum Starter


A simple RESTful API built with Rust using the **Axum** framework. This project serves as a starter template for building scalable and maintainable backend applications in Rust. It includes versioned APIs (`v1` and `v2`) for basic user management operations using a JSON file as a datastore.



## Features

- **Axum Framework:** Utilizes the ergonomic and powerful Axum web framework for building APIs.
- **Versioned APIs:** Supports API versioning (`v1` and `v2`) to manage different iterations of the API.
- **JSON File Datastore:** Implements a simple JSON file as a datastore for user data.
- **User Model:** Basic `User` model with `id`, `name`, and `email` fields.
- **CRUD Operations:** Provides `create` and `get` endpoints for managing users.
- **Professional File Architecture:** Organized project structure for scalability and maintainability.
- **Error Handling:** Robust error handling using the `anyhow` crate.
- **Middleware:** Incorporates logging middleware with `tower-http`'s `TraceLayer`.

## Technology Stack

- **Language:** Rust
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Asynchronous Runtime:** [Tokio](https://tokio.rs/)
- **Serialization:** [Serde](https://serde.rs/)
- **Unique Identifiers:** [UUID](https://docs.rs/uuid/)
- **Middleware:** [Tower](https://docs.rs/tower/) and [Tower-HTTP](https://docs.rs/tower-http/)
- **Error Handling:** [Anyhow](https://crates.io/crates/anyhow)
- **Concurrency:** [Lazy Static](https://docs.rs/lazy_static/)

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust:** Install from [rustup.rs](https://rustup.rs/)
- **Cargo:** Comes with Rust installation
- **Git:** For version control (optional but recommended)

### Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/your-username/rust-axum-starter.git
   cd rust-axum-starter
   ```

2. **Install Dependencies:**

   Cargo will automatically handle dependency installation. Ensure your `Cargo.toml` includes the necessary dependencies as outlined in the [Cargo.toml](#cargo.toml) section.

### Running the Application

1. **Initialize the JSON Database File:**

   Ensure the `db` directory and `users.json` file exist. If not, create them manually:

   ```bash
   mkdir -p db
   echo "[]" > db/users.json
   ```

2. **Run the Server:**

   ```bash
   cargo run
   ```

   You should see output similar to:

   ```
   🚀 Server running at http://127.0.0.1:3000
   ```

## API Endpoints

The API provides endpoints for creating and retrieving users. It is versioned into `v1` and `v2` to allow for future enhancements and changes without disrupting existing clients.

### Version 1 (v1)

#### Create User

- **Endpoint:** `POST /v1/users`
- **Description:** Creates a new user.
- **Request Body:**

  ```json
  {
      "name": "John Doe",
      "email": "john.doe@example.com"
  }
  ```

- **Response:**

  - **Status:** `201 Created`
  - **Body:**

    ```json
    {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "John Doe",
        "email": "john.doe@example.com"
    }
    ```

#### Get Users

- **Endpoint:** `GET /v1/users`
- **Description:** Retrieves a list of all users.
- **Response:**

  - **Status:** `200 OK`
  - **Body:**

    ```json
    [
        {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "John Doe",
            "email": "john.doe@example.com"
        }
    ]
    ```

### Version 2 (v2)

*Note: Currently, `v2` mirrors `v1`. Future enhancements can be made independently within this version.*

#### Create User

- **Endpoint:** `POST /v2/users`
- **Description:** Creates a new user with potential future enhancements.
- **Request Body:**

  ```json
  {
      "name": "Jane Smith",
      "email": "jane.smith@example.com"
  }
  ```

- **Response:**

  - **Status:** `201 Created`
  - **Body:**

    ```json
    {
        "id": "660e8400-e29b-41d4-a716-446655440111",
        "name": "Jane Smith",
        "email": "jane.smith@example.com"
    }
    ```

#### Get Users

- **Endpoint:** `GET /v2/users`
- **Description:** Retrieves a list of all users with potential future modifications.
- **Response:**

  - **Status:** `200 OK`
  - **Body:**

    ```json
    [
        {
            "id": "660e8400-e29b-41d4-a716-446655440111",
            "name": "Jane Smith",
            "email": "jane.smith@example.com"
        }
    ]
    ```

## Project Structure

A professional and scalable file architecture is essential for maintaining and expanding the project. Below is the recommended structure:

```
rust-axum-starter/
├── Cargo.toml
├── README.md
└── src
    ├── main.rs
    ├── models
    │   └── user.rs
    ├── db
    │   ├── error.rs
    │   └── mod.rs
    ├── handlers
    │   ├── mod.rs
    │   ├── v1.rs
    │   └── v2.rs
    └── routes
        ├── mod.rs
        ├── v1.rs
        └── v2.rs
```

- **main.rs:** Entry point of the application.
- **models/user.rs:** Defines the `User` data model.
- **db/mod.rs:** Handles database operations using a JSON file.
- **db/error.rs:** Custom error definitions for the `db` module.
- **handlers/mod.rs:** Organizes route handlers.
- **handlers/v1.rs & handlers/v2.rs:** Define handlers for respective API versions.
- **routes/mod.rs:** Combines all routes.
- **routes/v1.rs & routes/v2.rs:** Define routes for respective API versions.

## Contributing

Contributions are welcome! Please follow these steps:

1. **Fork the Repository**
2. **Create a Feature Branch**

   ```bash
   git checkout -b feature/YourFeature
   ```

3. **Commit Your Changes**

   ```bash
   git commit -m "Add YourFeature"
   ```

4. **Push to the Branch**

   ```bash
   git push origin feature/YourFeature
   ```

5. **Open a Pull Request**

Please ensure your code follows the project's coding standards and includes appropriate tests.

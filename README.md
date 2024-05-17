# User CRUD API in Rust with Actix

This is a simple CRUD (Create, Read, Update, Delete) application for managing users, built with Rust and the Actix web framework. This project is intended for learning purposes and demonstrates basic operations for user management.

## Features

- Create a new user
- Retrieve a list of users
- Retrieve a specific user by ID
- Update an existing user
- Delete a user

## Technologies Used

- Rust
- Actix-web
- SQLx (for database interactions)
- PostgreSQL (as the database)

## API Endpoints

- **Create a new user**

    ```http
    POST /users
    ```

    **Request body:**

    ```json
    {
        "name": "John Doe",
        "email": "john.doe@example.com",
        "password": "securepassword"
    }
    ```

- **Retrieve a list of users**

    ```http
    GET /users
    ```

- **Retrieve a specific user by ID**

    ```http
    GET /users/{id}
    ```

- **Update an existing user**

    ```http
    PUT /users/{id}
    ```

    **Request body:**

    ```json
    {
        "name": "Jane Doe",
        "email": "jane.doe@example.com",
        "password": "newsecurepassword"
    }
    ```

- **Delete a user**

    ```http
    DELETE /users/{id}
    ```

## Acknowledgements

- [Actix Web Framework](https://actix.rs/)
- [SQLx](https://github.com/launchbadge/sqlx)

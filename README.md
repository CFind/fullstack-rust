# Project Overview

This project is a full-stack web application built with Rust. It aims to showcase the power and versatility of Rust in web development. The application includes both a backend server and a frontend user interface.

Big thank you to [Francesco Ciulla](https://www.youtube.com/@francescociulla) for the great tutorial.

## Features

- Backend server built with Rust's Rocket framework
- Frontend user interface using HTML, CSS, and JavaScript
- Database integration with PostgreSQL
- RESTful API endpoints for data manipulation

## Getting Started

To get started with this project, follow these steps:

1. Clone the repository to your local machine:
    ```
    git clone https://github.com/CFind/fullstack-rust.git
    ```

2. Install Rust programming language by following the official installation guide: [Rust Installation Guide](https://www.rust-lang.org/tools/install)

3. Install Docker and Docker Compose on your local machine. You can find installation instructions for Docker [here](https://docs.docker.com/get-docker/) and for Docker Compose [here](https://docs.docker.com/compose/install/).

4. Clone the repository to your local machine:
    ```
    git clone https://github.com/CFind/fullstack-rust.git
    ```

5. Navigate to the project directory:
    ```
    cd fullstack-rust
    ```

6. Open the `compose.yaml` file in your workspace and modify any necessary configurations, such as database credentials or port mappings.

7. Start the application using Docker Compose:
    ```
    docker-compose up
    ```

8. Open your web browser and visit `http://localhost:` to access the application.

## Frontend

To start the frontend of the application using `create-next-app@latest` and add it to Docker Compose, follow these steps:

1. Install `create-next-app` globally by running the following command:
    ```
    npm install -g create-next-app
    ```

2. Create a new Next.js project by running the following command:
    ```
    npx create-next-app@latest
    ```

3. Navigate to the project directory:
    ```
    cd your-project-directory
    ```

4. Open the `compose.yaml` file in your workspace and add the following service definition under the `services` section:
    ```yaml
    frontend:
      build:
         context: .
         dockerfile: Dockerfile
      ports:
         - 3000:3000
      depends_on:
         - backend
    ```

5. Save the `compose.yaml` file.

6. Build and start the application using Docker Compose:
    ```
    docker-compose up --build
    ```

7. Open your web browser and visit `http://localhost:3000` to access the frontend of the application.

Make sure to replace `your-project-directory` with the actual directory name of your Next.js project.



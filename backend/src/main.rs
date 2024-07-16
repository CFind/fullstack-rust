// REFERENCE OPEN-SOURCE REPO: https://github.com/FrancescoXX/fullstack-rust-nextjs/tree/main/backend

use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// ################## SERVER ##################

// Constants
const OK_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin:*\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERAL_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

// ##################### Deserialize ###################
// get id from request
fn get_id(request: &str) -> &str {
    request.split("/").nth(4).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//Deserialize user from request body without id
fn deserialize_req_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

// create client
fn handle_client(mut tcp_stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request: String = String::new();

    match tcp_stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("OPTIONS") => (OK_RESPONSE.to_string(), "".to_string()),
                r if r.starts_with("POST /api/rust/users/") => handle_post_request(r),
                r if r.starts_with("GET /api/rust/users/") => handle_get_request(r),
                r if r.starts_with("GET /api/rust/users") => handle_get_all_request(r),
                r if r.starts_with("PUT /api/rust/users/") => handle_put_request(r),
                r if r.starts_with("DELETE /api/rust/users/") => handle_delete_request(r),
                _ => (NOT_FOUND_RESPONSE.to_string(), "404 not found".to_string()),
            };

            tcp_stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Error reading from stream {}", e);
        }
    }
}

//################# REST #######################

//handle get all request
fn handle_get_all_request(_request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();
            for row in client
                .query("SELECT id, name, email FROM users", &[])
                .expect("SHOULD return all users") {
                users.push(User {
                    id: Some(row.get(0)),
                    name: row.get(1),
                    email: row.get(2),
                });
            }
            (
                OK_RESPONSE.to_string(),
                serde_json::to_string_pretty(&users).expect("SHOULD return the users"),
            )
        }
        _ => (INTERAL_ERROR.to_string(), "Internal error, get_all_users".to_string()),
    }
}

// Handle GET request
fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: Some(row.get(0)),
                        name: row.get(1),
                        email: row.get(2),
                    };
                    (
                        OK_RESPONSE.to_string(),
                        serde_json::to_string_pretty(&user).expect("SHOULD return the user"),
                    )
                }
                _ => (NOT_FOUND_RESPONSE.to_string(), "User {}, not found".to_string()),
            }
        _ => (INTERAL_ERROR.to_string(), "Failed to retrieve user".to_string()),
    }
}

// Handle POST request
fn handle_post_request(request: &str) -> (String, String) {
    match (deserialize_req_body(request), Client::connect(DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            let row = client
                .query_one(
                    "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
                    &[&user.name, &user.email]
                )
                .expect("SHOULD return exactly one row.");

            let user_id: i32 = row.get(0);

            // Fetch the created user
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&user_id]) {
                Ok(row) => {
                    let user = User {
                        id: Some(row.get(0)),
                        name: row.get(1),
                        email: row.get(2),
                    };
                    (
                        OK_RESPONSE.to_string(),
                        serde_json::to_string_pretty(&user).expect("SHOULD return the new user"),
                    )
                }
                Err(e) =>
                    (
                        INTERAL_ERROR.to_string(),
                        format!("Failed to retrieve created user{}", e.to_string()),
                    ),
            }
        }
        _ => {
            (
                INTERAL_ERROR.to_string(),
                "SOME STUFF WENT DOWN IN HANDLE POST REQUEST. REFACTOR MATCH. DO NOT MATCH TUPLES YOU CRAZY ITALIAN".to_string(),
            )
        }
    }
}

// Handle PUT request
fn handle_put_request(request: &str) -> (String, String) {
    match
        (
            get_id(&request).parse::<i32>(),
            deserialize_req_body(&request),
            Client::connect(DB_URL, NoTls),
        )
    {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2, WHERE id = $3",
                    &[&user.name, &user.email, &id]
                )
                .expect("SHOULD update exactly one row.");
            (OK_RESPONSE.to_string(), format!("User {} updated", id))
        }
        _ => (INTERAL_ERROR.to_string(), "Failed to update user".to_string()),
    }
}

// Handle DELETE request
fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_altered: u64 = client
                .execute("DELETE FROM users WHERE id = $1", &[&id])
                .expect("SHOULD delete exactly one row.");
            if rows_altered <= 0 {
                (NOT_FOUND_RESPONSE.to_string(), format!("User {} not found", id))
            } else {
                (OK_RESPONSE.to_string(), format!("User {} deleted", id))
            }
        }
        _ => (INTERAL_ERROR.to_string(), "Failed to delete user".to_string()),
    }
}


// Database URL
const DB_URL: &str = env!("DATABASE_URL");

// Database setup
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    ",
    )?;
    Ok(())
}

//main function
fn main() {

    match set_database() {
        Err(e) => {
            eprintln!("Failed to setup database: {}", e);
            return;
        },
        Ok(()) => println!("Database setup complete")
    }

    let tcp_listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

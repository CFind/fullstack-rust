// REFERENCE OPEN-SOURCE REPO: https://github.com/FrancescoXX/fullstack-rust-nextjs/tree/main/backend

use postgres::{Client, NoTls};
use postgres::Error as PostegressError;
use std::{error::Error, net::{SocketAddr, TcpListener, TcpStream}};
use std::io::{Read, Write};
use std::env;
use std::thread;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// Database URL
// TODO!
//const DB_URL: &str = env!("DATABASE_URL");


// Constants
const OK_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin:*\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERAL_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

//main function
fn main() {
    //TODO!
    // Set Database
    // if set_database().is_err() {
    //     panic!("set_databasie");  //exit with error
    // }
    


    if let tcp_listener = TcpListener::bind("0.0.0.0:8085") 
        
    }
    





}

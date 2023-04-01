use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::__private::from_utf8_lossy;
use serde::{Serialize};

#[derive(Serialize)]
pub enum Jobs {
    PrintMessageJob {
        data: HashMap<String, String>,
    },
    PrintSumJob {
        data: HashMap<String, String>,
    },
}

#[tokio::main]
async fn main() {
    for i in 0..4 {
        add_to_print_message_job(i);
        add_to_print_sum_job(i);
    }
}

fn add_to_print_message_job(i: usize) {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            let mut data = HashMap::new();
            data.insert("message".to_string(), format!("My {} job", i).to_string());

            let entry = Jobs::PrintMessageJob {data};

            let value = serde_json::to_value(&entry).unwrap();

            stream.write(value.to_string().as_bytes()).unwrap();

            let mut buffer = [0 as u8; 1024];

            match stream.read_exact(&mut buffer) {
                Ok(_) => {
                    let _text = from_utf8_lossy(&buffer);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }

        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn add_to_print_sum_job(i: usize) {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            let mut data = HashMap::new();
            data.insert("number_1".to_string(), i.to_string());
            data.insert("number_2".to_string(), i.to_string());

            let entry = Jobs::PrintSumJob {data};

            let value = serde_json::to_value(&entry).unwrap();

            stream.write(value.to_string().as_bytes()).unwrap();

            let mut buffer = [0 as u8; 1024];

            match stream.read_exact(&mut buffer) {
                Ok(_) => {
                    let _text = from_utf8_lossy(&buffer);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }

        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

//! Basic usage example for the protocol
//!
//! Run with: cargo run --example basic

use protocol_name::{decode, encode, Message, MessageType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Protocol Name - Basic Example\n");

    // Create a request message
    println!("1. Creating a request message...");
    let request = Message::request(1, b"Hello, Protocol!");
    println!("   Type: {:?}", request.message_type());
    println!("   ID: {}", request.id);
    println!("   Payload: {:?}", String::from_utf8_lossy(&request.payload));

    // Encode the message
    println!("\n2. Encoding to bytes...");
    let bytes = encode(&request)?;
    println!("   Bytes ({} total): {:02x?}", bytes.len(), &bytes[..std::cmp::min(20, bytes.len())]);

    // Decode the message
    println!("\n3. Decoding from bytes...");
    let decoded = decode(&bytes)?;
    println!("   Type: {:?}", decoded.message_type());
    println!("   ID: {}", decoded.id);
    println!("   Payload: {:?}", String::from_utf8_lossy(&decoded.payload));

    // Create a response
    println!("\n4. Creating a response...");
    let response = Message::response(1, 0, b"Message received!");
    let response_bytes = encode(&response)?;
    println!("   Status: {} (success)", response.status.unwrap());
    println!("   Is success: {}", response.is_success());
    println!("   Bytes: {} total", response_bytes.len());

    // Create an error
    println!("\n5. Creating an error message...");
    let error = Message::error(1, 0x03, "Payload too large");
    let error_bytes = encode(&error)?;
    println!("   Error code: {}", error.status.unwrap());
    println!("   Message: {:?}", String::from_utf8_lossy(&error.payload));
    println!("   Bytes: {} total", error_bytes.len());

    println!("\n✓ All examples completed successfully!");
    Ok(())
}

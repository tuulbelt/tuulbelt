//! Protocol Name CLI
//!
//! Command-line interface for testing and debugging the protocol.
//!
//! ## Usage
//!
//! ```bash
//! # Encode a message to hex
//! protocol-name encode --type request --id 1 --payload "hello"
//!
//! # Decode a hex message
//! protocol-name decode 545501010000000100000005hello
//!
//! # Validate a message
//! protocol-name validate 545501010000000100000005hello
//! ```

use std::env;
use std::process;

use protocol_name::{decode, encode, Message, MessageType, MAGIC, VERSION};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let result = match args[1].as_str() {
        "encode" => cmd_encode(&args[2..]),
        "decode" => cmd_decode(&args[2..]),
        "validate" => cmd_validate(&args[2..]),
        "version" => {
            println!("Protocol Name v{}", env!("CARGO_PKG_VERSION"));
            println!("Protocol Version: {}", VERSION);
            println!("Magic: {:02X}{:02X}", MAGIC[0], MAGIC[1]);
            Ok(())
        }
        "-h" | "--help" | "help" => {
            print_usage();
            Ok(())
        }
        cmd => {
            eprintln!("Unknown command: {}", cmd);
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn print_usage() {
    eprintln!("Protocol Name - Reference Implementation CLI");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    protocol-name <COMMAND> [OPTIONS]");
    eprintln!();
    eprintln!("COMMANDS:");
    eprintln!("    encode      Encode a message to hex");
    eprintln!("    decode      Decode a hex message");
    eprintln!("    validate    Validate a hex message");
    eprintln!("    version     Show version info");
    eprintln!("    help        Show this message");
    eprintln!();
    eprintln!("EXAMPLES:");
    eprintln!("    protocol-name encode --type request --id 1 --payload hello");
    eprintln!("    protocol-name decode 545501010000000100000005hello");
    eprintln!("    protocol-name validate 545501010000000100000005hello");
}

fn cmd_encode(args: &[String]) -> Result<(), String> {
    let mut msg_type = MessageType::Request;
    let mut id: u32 = 1;
    let mut payload = Vec::new();
    let mut status: u8 = 0;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--type" | "-t" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value for --type".to_string());
                }
                msg_type = match args[i].as_str() {
                    "request" => MessageType::Request,
                    "response" => MessageType::Response,
                    "error" => MessageType::Error,
                    t => return Err(format!("Unknown type: {}", t)),
                };
            }
            "--id" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value for --id".to_string());
                }
                id = args[i].parse().map_err(|_| "Invalid ID")?;
            }
            "--payload" | "-p" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value for --payload".to_string());
                }
                payload = args[i].as_bytes().to_vec();
            }
            "--status" | "-s" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value for --status".to_string());
                }
                status = args[i].parse().map_err(|_| "Invalid status")?;
            }
            arg => {
                return Err(format!("Unknown argument: {}", arg));
            }
        }
        i += 1;
    }

    let message = match msg_type {
        MessageType::Request => Message::request(id, &payload),
        MessageType::Response => Message::response(id, status, &payload),
        MessageType::Error => Message::error(id, status, std::str::from_utf8(&payload).unwrap_or("")),
    };

    let bytes = encode(&message).map_err(|e| e.to_string())?;

    // Print as hex
    for byte in &bytes {
        print!("{:02x}", byte);
    }
    println!();

    Ok(())
}

fn cmd_decode(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Missing hex input".to_string());
    }

    let hex = &args[0];
    let bytes = hex_to_bytes(hex)?;
    let message = decode(&bytes).map_err(|e| e.to_string())?;

    println!("Version: {}", message.version);
    println!("Type: {:?}", message.message_type);
    println!("ID: {}", message.id);
    if let Some(status) = message.status {
        println!("Status: {}", status);
    }
    println!("Payload ({} bytes): {:?}", message.payload.len(), String::from_utf8_lossy(&message.payload));

    Ok(())
}

fn cmd_validate(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Missing hex input".to_string());
    }

    let hex = &args[0];
    let bytes = hex_to_bytes(hex)?;

    match decode(&bytes) {
        Ok(_) => {
            println!("Valid message");
            Ok(())
        }
        Err(e) => {
            println!("Invalid message: {}", e);
            Err(e.to_string())
        }
    }
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|_| format!("Invalid hex at position {}", i))
        })
        .collect()
}

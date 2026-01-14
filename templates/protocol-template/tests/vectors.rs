//! Protocol compliance test vectors
//!
//! These tests verify the implementation against the specification.
//! Each test corresponds to a vector in SPEC.md Section 7.

use protocol_name::{decode, encode, Message, MessageType, ProtocolError};

// ============================================================================
// Valid Message Vectors
// ============================================================================

#[test]
fn vector_minimal_request() {
    // From SPEC.md Section 7.1
    // Minimal request with empty payload
    let bytes: Vec<u8> = vec![
        0x54, 0x55, // Magic: "TU"
        0x01,       // Version: 1
        0x01,       // Type: Request
        0x00, 0x00, 0x00, 0x01, // ID: 1
        0x00, 0x00, 0x00, 0x00, // Payload length: 0
    ];

    let message = decode(&bytes).expect("Should decode minimal request");

    assert_eq!(message.version, 1);
    assert_eq!(message.message_type, MessageType::Request);
    assert_eq!(message.id, 1);
    assert!(message.payload.is_empty());
}

#[test]
fn vector_request_with_payload() {
    // From SPEC.md Section 7.1
    // Request with "hello" payload
    let bytes: Vec<u8> = vec![
        0x54, 0x55, // Magic: "TU"
        0x01,       // Version: 1
        0x01,       // Type: Request
        0x00, 0x00, 0x00, 0x02, // ID: 2
        0x00, 0x00, 0x00, 0x05, // Payload length: 5
        0x68, 0x65, 0x6C, 0x6C, 0x6F, // Payload: "hello"
    ];

    let message = decode(&bytes).expect("Should decode request with payload");

    assert_eq!(message.version, 1);
    assert_eq!(message.message_type, MessageType::Request);
    assert_eq!(message.id, 2);
    assert_eq!(message.payload, b"hello");
}

#[test]
fn vector_success_response() {
    // Response with status 0 (success)
    let bytes: Vec<u8> = vec![
        0x54, 0x55, // Magic: "TU"
        0x01,       // Version: 1
        0x02,       // Type: Response
        0x00, 0x00, 0x00, 0x03, // ID: 3
        0x00,       // Status: 0 (success)
        0x00, 0x00, 0x00, 0x02, // Payload length: 2
        0x6F, 0x6B, // Payload: "ok"
    ];

    let message = decode(&bytes).expect("Should decode success response");

    assert_eq!(message.message_type, MessageType::Response);
    assert_eq!(message.id, 3);
    assert_eq!(message.status, Some(0));
    assert!(message.is_success());
    assert_eq!(message.payload, b"ok");
}

#[test]
fn vector_error_response() {
    // Error message
    let bytes: Vec<u8> = vec![
        0x54, 0x55, // Magic: "TU"
        0x01,       // Version: 1
        0xFF,       // Type: Error
        0x00, 0x00, 0x00, 0x04, // ID: 4
        0x01,       // Error code: 1
        0x00, 0x00, 0x00, 0x05, // Payload length: 5
        0x65, 0x72, 0x72, 0x6F, 0x72, // Payload: "error"
    ];

    let message = decode(&bytes).expect("Should decode error");

    assert_eq!(message.message_type, MessageType::Error);
    assert_eq!(message.id, 4);
    assert_eq!(message.status, Some(1));
    assert_eq!(message.payload, b"error");
}

// ============================================================================
// Invalid Message Vectors
// ============================================================================

#[test]
fn vector_invalid_magic() {
    // From SPEC.md Section 7.2
    let bytes: Vec<u8> = vec![
        0x00, 0x00, // Invalid magic
        0x01,       // Version: 1
        0x01,       // Type: Request
        0x00, 0x00, 0x00, 0x01, // ID: 1
        0x00, 0x00, 0x00, 0x00, // Payload length: 0
    ];

    let result = decode(&bytes);
    assert!(matches!(result, Err(ProtocolError::InvalidMagic([0x00, 0x00]))));
}

#[test]
fn vector_unknown_type() {
    // From SPEC.md Section 7.2
    let bytes: Vec<u8> = vec![
        0x54, 0x55, // Magic: "TU"
        0x01,       // Version: 1
        0x99,       // Type: Unknown (0x99)
        0x00, 0x00, 0x00, 0x01, // ID: 1
        0x00, 0x00, 0x00, 0x00, // Payload length: 0
    ];

    let result = decode(&bytes);
    assert!(matches!(result, Err(ProtocolError::UnknownType(0x99))));
}

#[test]
fn vector_unsupported_version() {
    let bytes: Vec<u8> = vec![
        0x54, 0x55, // Magic: "TU"
        0x99,       // Version: 153 (unsupported)
        0x01,       // Type: Request
        0x00, 0x00, 0x00, 0x01, // ID: 1
        0x00, 0x00, 0x00, 0x00, // Payload length: 0
    ];

    let result = decode(&bytes);
    assert!(matches!(result, Err(ProtocolError::UnsupportedVersion(0x99))));
}

#[test]
fn vector_incomplete_message() {
    // Too short to be valid
    let bytes: Vec<u8> = vec![0x54, 0x55, 0x01];

    let result = decode(&bytes);
    assert!(matches!(result, Err(ProtocolError::IncompleteMessage)));
}

// ============================================================================
// Round-trip Tests
// ============================================================================

#[test]
fn roundtrip_request() {
    let original = Message::request(42, b"test payload");
    let bytes = encode(&original).expect("Should encode");
    let decoded = decode(&bytes).expect("Should decode");

    assert_eq!(original.id, decoded.id);
    assert_eq!(original.message_type, decoded.message_type);
    assert_eq!(original.payload, decoded.payload);
}

#[test]
fn roundtrip_response() {
    let original = Message::response(123, 0, b"response data");
    let bytes = encode(&original).expect("Should encode");
    let decoded = decode(&bytes).expect("Should decode");

    assert_eq!(original.id, decoded.id);
    assert_eq!(original.message_type, decoded.message_type);
    assert_eq!(original.status, decoded.status);
    assert_eq!(original.payload, decoded.payload);
}

#[test]
fn roundtrip_error() {
    let original = Message::error(456, 0x03, "something went wrong");
    let bytes = encode(&original).expect("Should encode");
    let decoded = decode(&bytes).expect("Should decode");

    assert_eq!(original.id, decoded.id);
    assert_eq!(original.message_type, decoded.message_type);
    assert_eq!(original.status, decoded.status);
    assert_eq!(original.payload, decoded.payload);
}

#[test]
fn roundtrip_empty_payload() {
    let original = Message::request(0, b"");
    let bytes = encode(&original).expect("Should encode");
    let decoded = decode(&bytes).expect("Should decode");

    assert!(decoded.payload.is_empty());
}

#[test]
fn roundtrip_large_payload() {
    let payload = vec![0xAB; 10000]; // 10KB payload
    let original = Message::request(999, &payload);
    let bytes = encode(&original).expect("Should encode");
    let decoded = decode(&bytes).expect("Should decode");

    assert_eq!(decoded.payload.len(), 10000);
    assert_eq!(decoded.payload, payload);
}

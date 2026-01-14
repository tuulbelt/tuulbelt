//! # Protocol Name
//!
//! Reference implementation of the Protocol Name specification.
//!
//! See [SPEC.md](../SPEC.md) for the full protocol specification.
//!
//! ## Example
//!
//! ```rust
//! use protocol_name::{Message, MessageType, encode, decode};
//!
//! // Create a request
//! let request = Message::request(1, b"hello");
//!
//! // Encode to bytes
//! let bytes = encode(&request).unwrap();
//!
//! // Decode from bytes
//! let decoded = decode(&bytes).unwrap();
//! assert_eq!(decoded.message_type(), MessageType::Request);
//! ```

use std::io::{self, Read, Write};

// ============================================================================
// Constants
// ============================================================================

/// Protocol magic bytes (0x54 0x55 = "TU")
pub const MAGIC: [u8; 2] = [0x54, 0x55];

/// Current protocol version
pub const VERSION: u8 = 1;

/// Maximum payload size (1 MB default)
pub const MAX_PAYLOAD_SIZE: usize = 1024 * 1024;

// ============================================================================
// Types
// ============================================================================

/// Message type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    /// Client request
    Request = 0x01,
    /// Server response
    Response = 0x02,
    /// Error message
    Error = 0xFF,
}

impl TryFrom<u8> for MessageType {
    type Error = ProtocolError;

    fn try_from(value: u8) -> Result<Self, ProtocolError> {
        match value {
            0x01 => Ok(MessageType::Request),
            0x02 => Ok(MessageType::Response),
            0xFF => Ok(Self::Error),
            _ => Err(ProtocolError::UnknownType(value)),
        }
    }
}

/// Protocol error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    /// Invalid magic bytes
    InvalidMagic([u8; 2]),
    /// Unsupported protocol version
    UnsupportedVersion(u8),
    /// Unknown message type
    UnknownType(u8),
    /// Payload exceeds maximum size
    PayloadTooLarge(usize),
    /// Incomplete message (not enough bytes)
    IncompleteMessage,
    /// I/O error
    Io(String),
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMagic(magic) => write!(f, "invalid magic: {:02x}{:02x}", magic[0], magic[1]),
            Self::UnsupportedVersion(v) => write!(f, "unsupported version: {}", v),
            Self::UnknownType(t) => write!(f, "unknown message type: {:02x}", t),
            Self::PayloadTooLarge(size) => write!(f, "payload too large: {} bytes", size),
            Self::IncompleteMessage => write!(f, "incomplete message"),
            Self::Io(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for ProtocolError {}

impl From<io::Error> for ProtocolError {
    fn from(err: io::Error) -> Self {
        ProtocolError::Io(err.to_string())
    }
}

/// A protocol message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// Protocol version
    pub version: u8,
    /// Message type
    pub message_type: MessageType,
    /// Request/response ID
    pub id: u32,
    /// Response status (only for Response type)
    pub status: Option<u8>,
    /// Message payload
    pub payload: Vec<u8>,
}

impl Message {
    /// Create a new request message
    pub fn request(id: u32, payload: &[u8]) -> Self {
        Self {
            version: VERSION,
            message_type: MessageType::Request,
            id,
            status: None,
            payload: payload.to_vec(),
        }
    }

    /// Create a new response message
    pub fn response(id: u32, status: u8, payload: &[u8]) -> Self {
        Self {
            version: VERSION,
            message_type: MessageType::Response,
            id,
            status: Some(status),
            payload: payload.to_vec(),
        }
    }

    /// Create an error message
    pub fn error(id: u32, error_code: u8, message: &str) -> Self {
        Self {
            version: VERSION,
            message_type: MessageType::Error,
            id,
            status: Some(error_code),
            payload: message.as_bytes().to_vec(),
        }
    }

    /// Get the message type
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }

    /// Check if this is a successful response
    pub fn is_success(&self) -> bool {
        self.message_type == MessageType::Response && self.status == Some(0)
    }
}

// ============================================================================
// Encoding
// ============================================================================

/// Encode a message to bytes
pub fn encode(message: &Message) -> Result<Vec<u8>, ProtocolError> {
    let payload_len = message.payload.len();
    if payload_len > MAX_PAYLOAD_SIZE {
        return Err(ProtocolError::PayloadTooLarge(payload_len));
    }

    let mut buf = Vec::with_capacity(11 + payload_len);

    // Header: magic (2) + version (1)
    buf.extend_from_slice(&MAGIC);
    buf.push(message.version);

    // Type (1)
    buf.push(message.message_type as u8);

    // ID (4, big-endian)
    buf.extend_from_slice(&message.id.to_be_bytes());

    // Status (1, only for Response/Error)
    if let Some(status) = message.status {
        buf.push(status);
    }

    // Payload length (4, big-endian) + payload
    buf.extend_from_slice(&(payload_len as u32).to_be_bytes());
    buf.extend_from_slice(&message.payload);

    Ok(buf)
}

/// Write a message to a writer
pub fn write_message<W: Write>(writer: &mut W, message: &Message) -> Result<(), ProtocolError> {
    let bytes = encode(message)?;
    writer.write_all(&bytes)?;
    Ok(())
}

// ============================================================================
// Decoding
// ============================================================================

/// Decode a message from bytes
pub fn decode(bytes: &[u8]) -> Result<Message, ProtocolError> {
    if bytes.len() < 8 {
        return Err(ProtocolError::IncompleteMessage);
    }

    // Header: magic (2)
    let magic = [bytes[0], bytes[1]];
    if magic != MAGIC {
        return Err(ProtocolError::InvalidMagic(magic));
    }

    // Version (1)
    let version = bytes[2];
    if version != VERSION {
        return Err(ProtocolError::UnsupportedVersion(version));
    }

    // Type (1)
    let message_type = MessageType::try_from(bytes[3])?;

    // ID (4, big-endian)
    let id = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

    // Parse based on message type
    let (status, payload_offset) = match message_type {
        MessageType::Request => (None, 8),
        MessageType::Response | MessageType::Error => {
            if bytes.len() < 9 {
                return Err(ProtocolError::IncompleteMessage);
            }
            (Some(bytes[8]), 9)
        }
    };

    // Payload length (4) + payload
    if bytes.len() < payload_offset + 4 {
        return Err(ProtocolError::IncompleteMessage);
    }

    let payload_len = u32::from_be_bytes([
        bytes[payload_offset],
        bytes[payload_offset + 1],
        bytes[payload_offset + 2],
        bytes[payload_offset + 3],
    ]) as usize;

    if payload_len > MAX_PAYLOAD_SIZE {
        return Err(ProtocolError::PayloadTooLarge(payload_len));
    }

    let payload_start = payload_offset + 4;
    if bytes.len() < payload_start + payload_len {
        return Err(ProtocolError::IncompleteMessage);
    }

    let payload = bytes[payload_start..payload_start + payload_len].to_vec();

    Ok(Message {
        version,
        message_type,
        id,
        status,
        payload,
    })
}

/// Read a message from a reader
pub fn read_message<R: Read>(reader: &mut R) -> Result<Message, ProtocolError> {
    // Read header
    let mut header = [0u8; 8];
    reader.read_exact(&mut header)?;

    // Validate and parse header
    let magic = [header[0], header[1]];
    if magic != MAGIC {
        return Err(ProtocolError::InvalidMagic(magic));
    }

    let version = header[2];
    if version != VERSION {
        return Err(ProtocolError::UnsupportedVersion(version));
    }

    let message_type = MessageType::try_from(header[3])?;
    let id = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);

    // Read status if needed
    let status = match message_type {
        MessageType::Request => None,
        MessageType::Response | MessageType::Error => {
            let mut status_buf = [0u8; 1];
            reader.read_exact(&mut status_buf)?;
            Some(status_buf[0])
        }
    };

    // Read payload length
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let payload_len = u32::from_be_bytes(len_buf) as usize;

    if payload_len > MAX_PAYLOAD_SIZE {
        return Err(ProtocolError::PayloadTooLarge(payload_len));
    }

    // Read payload
    let mut payload = vec![0u8; payload_len];
    reader.read_exact(&mut payload)?;

    Ok(Message {
        version,
        message_type,
        id,
        status,
        payload,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_request() {
        let request = Message::request(1, b"hello");
        let bytes = encode(&request).unwrap();
        let decoded = decode(&bytes).unwrap();

        assert_eq!(decoded.message_type, MessageType::Request);
        assert_eq!(decoded.id, 1);
        assert_eq!(decoded.payload, b"hello");
    }

    #[test]
    fn test_encode_decode_response() {
        let response = Message::response(2, 0, b"world");
        let bytes = encode(&response).unwrap();
        let decoded = decode(&bytes).unwrap();

        assert_eq!(decoded.message_type, MessageType::Response);
        assert_eq!(decoded.id, 2);
        assert_eq!(decoded.status, Some(0));
        assert_eq!(decoded.payload, b"world");
        assert!(decoded.is_success());
    }

    #[test]
    fn test_invalid_magic() {
        let bytes = [0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let result = decode(&bytes);
        assert!(matches!(result, Err(ProtocolError::InvalidMagic(_))));
    }

    #[test]
    fn test_unknown_type() {
        let bytes = [0x54, 0x55, 0x01, 0x99, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let result = decode(&bytes);
        assert!(matches!(result, Err(ProtocolError::UnknownType(0x99))));
    }
}

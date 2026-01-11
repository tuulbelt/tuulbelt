# Protocol Name Specification

**Version:** 0.1.0
**Status:** Draft
**Authors:** [Author Name]
**Date:** YYYY-MM-DD

---

## Abstract

One paragraph summary of what this protocol does and why it exists.

---

## 1. Introduction

### 1.1 Purpose

Explain the problem this protocol solves.

### 1.2 Scope

Define what is and isn't covered by this specification.

### 1.3 Terminology

| Term | Definition |
|------|------------|
| MUST | Requirement is absolute |
| SHOULD | Recommended but not required |
| MAY | Optional behavior |
| Message | A single unit of data in this protocol |

---

## 2. Wire Format

### 2.1 Overview

Describe the high-level structure of messages.

```
+--------+--------+--------+--------+
| Header |  Type  | Length | Payload|
+--------+--------+--------+--------+
```

### 2.2 Header

| Field | Size | Description |
|-------|------|-------------|
| Magic | 2 bytes | Protocol identifier (0xTUUL) |
| Version | 1 byte | Protocol version |

### 2.3 Message Types

| Type | Value | Description |
|------|-------|-------------|
| Request | 0x01 | Client request |
| Response | 0x02 | Server response |
| Error | 0xFF | Error message |

### 2.4 Encoding

- All integers are big-endian
- Strings are UTF-8 encoded
- Length fields are 4-byte unsigned integers

---

## 3. Message Semantics

### 3.1 Request

```
Request = Header Type(0x01) RequestId Payload

RequestId = 4-byte unsigned integer
Payload = Length Data
```

### 3.2 Response

```
Response = Header Type(0x02) RequestId Status Payload

Status = 1-byte (0x00 = success, 0x01 = error)
```

### 3.3 Error Handling

| Error Code | Meaning |
|------------|---------|
| 0x01 | Invalid message format |
| 0x02 | Unknown message type |
| 0x03 | Payload too large |

---

## 4. Protocol Behavior

### 4.1 Connection Lifecycle

1. Client establishes connection
2. Client sends Request
3. Server sends Response
4. Repeat or close

### 4.2 Ordering Guarantees

- Responses MUST be sent in request order
- Request IDs MUST be unique per connection

### 4.3 Timeouts

- Implementations SHOULD timeout after 30 seconds
- Implementations MAY support configurable timeouts

---

## 5. Security Considerations

### 5.1 Authentication

This protocol does not define authentication. Implementations SHOULD:
- Use TLS for transport security
- Implement application-level authentication

### 5.2 Input Validation

Implementations MUST:
- Validate message length before reading payload
- Reject messages exceeding maximum size (configurable, default 1MB)
- Handle malformed messages gracefully

---

## 6. Extensibility

### 6.1 Version Negotiation

Future versions MAY add new message types. Unknown types SHOULD be ignored.

### 6.2 Reserved Fields

The following values are reserved for future use:
- Message types 0xF0-0xFE
- Error codes 0xF0-0xFE

---

## 7. Test Vectors

See `tests/vectors/` for compliance test data.

### 7.1 Valid Messages

```
# Minimal request
Input:  54 55 01 01 00 00 00 01 00 00 00 00
Parsed: Header(TUUL, v1) Request(id=1) Payload(empty)

# Request with payload
Input:  54 55 01 01 00 00 00 02 00 00 00 05 68 65 6C 6C 6F
Parsed: Header(TUUL, v1) Request(id=2) Payload("hello")
```

### 7.2 Invalid Messages

```
# Invalid magic
Input:  00 00 01 01 00 00 00 01 00 00 00 00
Error:  InvalidMagic

# Unknown type
Input:  54 55 01 99 00 00 00 01 00 00 00 00
Error:  UnknownType
```

---

## Appendix A: Reference Implementation

See `src/lib.rs` for the reference implementation.

## Appendix B: Changelog

### v0.1.0 (YYYY-MM-DD)
- Initial draft specification

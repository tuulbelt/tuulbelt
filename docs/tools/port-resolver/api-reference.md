# API Reference

Complete API documentation for the port-resolver library.

## Types

### `Result<T, E>`

All methods return a Result type for explicit error handling:

```typescript
type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };
```

### `PortConfig`

Configuration for port allocation:

```typescript
interface PortConfig {
  minPort?: number;   // Minimum port (default: 10000)
  maxPort?: number;   // Maximum port (default: 65535)
  tag?: string;       // Optional identifier
}
```

### `PortEntry`

Registry entry for an allocated port:

```typescript
interface PortEntry {
  port: number;         // The allocated port number
  pid: number;          // Process ID that owns this port
  tag?: string;         // Optional tag for grouping
  allocatedAt: string;  // ISO 8601 timestamp
}
```

### `PortAllocation`

Result of a successful port allocation:

```typescript
interface PortAllocation {
  port: number;      // The allocated port
  entry: PortEntry;  // Full registry entry
}
```

### `RegistryStatus`

Statistics about the port registry:

```typescript
interface RegistryStatus {
  totalEntries: number;    // All entries in registry
  activeEntries: number;   // Entries with live processes
  staleEntries: number;    // Entries from dead processes
  registryPath: string;    // Path to registry file
}
```

### `PortRegistry`

Internal registry structure:

```typescript
interface PortRegistry {
  version: number;       // Registry format version
  entries: PortEntry[];  // All port entries
}
```

## Module-Level Functions (v0.2.0)

Convenience functions for common port allocation patterns.

### `getPort(options?)`

Convenience wrapper for single port allocation without class instantiation.

```typescript
async function getPort(options?: {
  tag?: string;
  config?: PortResolverConfig;
}): Promise<Result<PortAllocation>>
```

**Parameters:**
- `tag` - Optional tag for identification
- `config` - Optional PortResolver configuration

**Returns:** `Result<PortAllocation>` containing the allocated port

**Example:**
```typescript
import { getPort } from '@tuulbelt/port-resolver';

const result = await getPort({ tag: 'api-server' });
if (result.ok) {
  console.log(`Port: ${result.value.port}`);
}
```

---

### `getPorts(count, options?)`

Batch allocation with individual tags or shared tag.

```typescript
async function getPorts(
  count: number,
  options?: {
    tag?: string;
    tags?: string[];
    config?: PortResolverConfig;
  }
): Promise<Result<PortAllocation[]>>
```

**Parameters:**
- `count` - Number of ports to allocate
- `tag` - Optional shared tag for all ports
- `tags` - Optional array of individual tags (must match count)
- `config` - Optional PortResolver configuration

**Returns:** `Result<PortAllocation[]>` with all allocated ports

**All-or-nothing semantics:** If any allocation fails, all allocated ports are rolled back.

**Example:**
```typescript
import { getPorts } from '@tuulbelt/port-resolver';

// With individual tags
const services = await getPorts(3, {
  tags: ['http-server', 'grpc-server', 'metrics-server'],
});

// With shared tag
const workers = await getPorts(5, { tag: 'worker' });
```

---

## Class: `PortManager` (v0.2.0)

Port lifecycle management with tag-based tracking.

### Constructor

```typescript
new PortManager(config?: PortResolverConfig)
```

**Parameters:**
- `config` - Optional PortResolver configuration (same as PortResolver)

**Example:**
```typescript
import { PortManager } from '@tuulbelt/port-resolver';

const manager = new PortManager({
  minPort: 8000,
  maxPort: 9000,
});
```

### Methods

#### `allocate(tag?)`

Allocate a port and track by tag. **Prevents duplicate tags** within this instance.

```typescript
async allocate(tag?: string): Promise<Result<PortAllocation>>
```

**Parameters:**
- `tag` - Optional tag for identification

**Returns:** `Result<PortAllocation>` containing the allocated port

**Note:** Calling this twice with the same tag will fail to prevent losing track of allocations.

**Example:**
```typescript
const result = await manager.allocate('api-server');
if (result.ok) {
  console.log(`Allocated: ${result.value.port}`);
}

// This will fail:
await manager.allocate('api-server'); // Error: tag already in use
```

---

#### `allocateMultiple(count, tag?)`

Allocate multiple ports atomically. All share same tag if provided.

```typescript
async allocateMultiple(
  count: number,
  tag?: string
): Promise<Result<PortAllocation[]>>
```

**Parameters:**
- `count` - Number of ports to allocate
- `tag` - Optional shared tag for all ports

**Returns:** `Result<PortAllocation[]>` with all allocated ports

**Example:**
```typescript
const result = await manager.allocateMultiple(3, 'workers');
if (result.ok) {
  console.log(`Allocated ${result.value.length} ports`);
}
```

---

#### `release(tagOrPort)`

Release by tag or port number. **Idempotent** - succeeds even if already released.

```typescript
async release(tagOrPort: string | number): Promise<Result<void>>
```

**Parameters:**
- `tagOrPort` - Tag string or port number to release

**Returns:** `Result<void>` indicating success

**Note:** Returns success even if tag/port was not allocated (idempotent behavior).

**Example:**
```typescript
// Release by tag
await manager.release('api-server');

// Release by port number
await manager.release(54321);

// Calling again succeeds (idempotent)
await manager.release('api-server'); // Still succeeds
```

---

#### `releaseAll()`

Release all ports managed by this PortManager instance.

```typescript
async releaseAll(): Promise<Result<number>>
```

**Returns:** `Result<number>` with count of ports released

**Example:**
```typescript
const result = await manager.releaseAll();
if (result.ok) {
  console.log(`Released ${result.value} ports`);
}
```

---

#### `getAllocations()`

Get all tracked allocations as array.

```typescript
getAllocations(): PortAllocation[]
```

**Returns:** Array of all port allocations tracked by this instance

**Example:**
```typescript
const allocations = manager.getAllocations();
for (const alloc of allocations) {
  console.log(`${alloc.tag}: ${alloc.port}`);
}
```

---

#### `get(tag)`

Get specific allocation by tag.

```typescript
get(tag: string): PortAllocation | undefined
```

**Parameters:**
- `tag` - Tag to lookup

**Returns:** `PortAllocation` if found, `undefined` otherwise

**Example:**
```typescript
const frontend = manager.get('frontend');
if (frontend) {
  console.log(`Frontend port: ${frontend.port}`);
}
```

---

## Class: `PortResolver`

Main class for port allocation and management.

### Constructor

```typescript
new PortResolver(options?: {
  registryPath?: string;
  minPort?: number;
  maxPort?: number;
})
```

**Parameters:**
- `registryPath` - Path to the registry file (default: `/tmp/tuulbelt-port-registry.json`)
- `minPort` - Default minimum port for allocations (default: `10000`)
- `maxPort` - Default maximum port for allocations (default: `65535`)

**Example:**
```typescript
const resolver = new PortResolver({
  registryPath: '/tmp/my-test-ports.json',
  minPort: 8000,
  maxPort: 9000,
});
```

### Methods

#### `get(config?)`

Allocate a single port.

```typescript
async get(config?: PortConfig): Promise<Result<PortAllocation>>
```

**Parameters:**
- `config.minPort` - Override minimum port
- `config.maxPort` - Override maximum port
- `config.tag` - Tag for grouping/identification

**Returns:** `Result<PortAllocation>` containing the allocated port and entry

**Example:**
```typescript
const result = await resolver.get({ tag: 'api-server' });
if (result.ok) {
  console.log(`Port: ${result.value.port}`);
}
```

---

#### `getMultiple(count, config?)`

Allocate multiple ports at once.

```typescript
async getMultiple(
  count: number,
  config?: PortConfig
): Promise<Result<PortAllocation[]>>
```

**Parameters:**
- `count` - Number of ports to allocate (max: 100)
- `config` - Same as `get()`

**Returns:** `Result<PortAllocation[]>` with all allocated ports

**Example:**
```typescript
const result = await resolver.getMultiple(3, { tag: 'services' });
if (result.ok) {
  const [apiPort, dbPort, cachePort] = result.value.map(a => a.port);
}
```

---

#### `reserveRange(options)` (v0.2.0)

Reserve a contiguous range of ports.

```typescript
async reserveRange(options: {
  start: number;
  count: number;
  tag?: string;
}): Promise<Result<PortAllocation[]>>
```

**Parameters:**
- `start` - Starting port number
- `count` - Number of contiguous ports to reserve
- `tag` - Optional shared tag for all ports

**Returns:** `Result<PortAllocation[]>` with all allocated ports in range

**Use case:** Microservices clusters requiring adjacent ports.

**Example:**
```typescript
const cluster = await resolver.reserveRange({
  start: 50000,
  count: 5,
  tag: 'backend-cluster',
});

if (cluster.ok) {
  console.log('Cluster ports:', cluster.value.map(p => p.port).join(', '));
  // Output: "50000, 50001, 50002, 50003, 50004"
}
```

---

#### `getPortInRange(options)` (v0.2.0)

Get any available port within specific bounds.

```typescript
async getPortInRange(options: {
  min: number;
  max: number;
  tag?: string;
}): Promise<Result<PortAllocation>>
```

**Parameters:**
- `min` - Minimum port number (inclusive)
- `max` - Maximum port number (inclusive)
- `tag` - Optional tag for identification

**Returns:** `Result<PortAllocation>` with allocated port in range

**Use case:** Firewall rules or compliance requirements restricting port ranges.

**Example:**
```typescript
const apiPort = await resolver.getPortInRange({
  min: 8000,
  max: 9000,
  tag: 'public-api',
});

if (apiPort.ok) {
  console.log(`Public API port (8000-9000): ${apiPort.value.port}`);
}
```

---

#### `release(port)`

Release a single allocated port.

```typescript
async release(port: number): Promise<Result<void>>
```

**Parameters:**
- `port` - Port number to release

**Returns:** `Result<void>` indicating success or failure

**Note:** Only the process that allocated the port can release it.

**Example:**
```typescript
const released = await resolver.release(54321);
if (!released.ok) {
  console.error('Could not release port');
}
```

---

#### `releaseAll(tag?)`

Release multiple ports.

```typescript
async releaseAll(tag?: string): Promise<Result<number>>
```

**Parameters:**
- `tag` - Optional: only release ports with this tag

**Returns:** `Result<number>` with count of released ports

**Example:**
```typescript
// Release all ports owned by this process
await resolver.releaseAll();

// Release only ports with specific tag
await resolver.releaseAll('api-server');
```

---

#### `list()`

List all allocated ports in the registry.

```typescript
async list(): Promise<Result<PortEntry[]>>
```

**Returns:** `Result<PortEntry[]>` with all registry entries

**Example:**
```typescript
const result = await resolver.list();
if (result.ok) {
  for (const entry of result.value) {
    console.log(`${entry.port}: ${entry.tag} (PID ${entry.pid})`);
  }
}
```

---

#### `clean()`

Remove stale entries from crashed processes.

```typescript
async clean(): Promise<Result<number>>
```

**Returns:** `Result<number>` with count of removed entries

**Example:**
```typescript
const result = await resolver.clean();
if (result.ok) {
  console.log(`Removed ${result.value} stale entries`);
}
```

---

#### `status()`

Get registry statistics.

```typescript
async status(): Promise<Result<RegistryStatus>>
```

**Returns:** `Result<RegistryStatus>` with registry information

**Example:**
```typescript
const result = await resolver.status();
if (result.ok) {
  console.log(`Active: ${result.value.activeEntries}`);
  console.log(`Stale: ${result.value.staleEntries}`);
}
```

---

#### `clear()`

Remove all entries from the registry.

```typescript
async clear(): Promise<Result<void>>
```

**Returns:** `Result<void>`

**Warning:** This removes ALL entries, not just yours.

**Example:**
```typescript
await resolver.clear();
```

## Standalone Functions

### `isPortAvailable(port)`

Check if a specific port is available for binding.

```typescript
async function isPortAvailable(port: number): Promise<boolean>
```

**Parameters:**
- `port` - Port number to check

**Returns:** `true` if port can be bound, `false` otherwise

**Example:**
```typescript
import { isPortAvailable } from '@tuulbelt/port-resolver';

if (await isPortAvailable(3000)) {
  console.log('Port 3000 is free');
}
```

### `findAvailablePort(min, max)`

Find any available port in a range.

```typescript
async function findAvailablePort(
  min: number,
  max: number
): Promise<Result<number>>
```

**Parameters:**
- `min` - Minimum port number
- `max` - Maximum port number

**Returns:** `Result<number>` with an available port

**Algorithm:**
1. Try 10 random ports in range
2. Fall back to sequential scan if all random attempts fail

**Example:**
```typescript
import { findAvailablePort } from '@tuulbelt/port-resolver';

const result = await findAvailablePort(8000, 9000);
if (result.ok) {
  console.log(`Found port: ${result.value}`);
}
```

## Constants

```typescript
const DEFAULT_REGISTRY_PATH = '/tmp/tuulbelt-port-registry.json';
const DEFAULT_MIN_PORT = 10000;
const DEFAULT_MAX_PORT = 65535;
const REGISTRY_VERSION = 1;
const MAX_PORTS_PER_REQUEST = 100;
const MAX_REGISTRY_ENTRIES = 10000;
```

## Error Handling

Common error scenarios:

```typescript
// No ports available in range
{ ok: false, error: Error('No available port found in range 8000-8010') }

// Port not found for release
{ ok: false, error: Error('Port 54321 not found in registry') }

// Port owned by different process
{ ok: false, error: Error('Port 54321 is owned by PID 12345, not 67890') }

// Registry too large
{ ok: false, error: Error('Registry has too many entries (10001 > 10000)') }

// Too many ports requested
{ ok: false, error: Error('Cannot allocate more than 100 ports at once') }
```

## Thread Safety

The PortResolver uses [file-based-semaphore-ts](/tools/file-based-semaphore-ts/) (`semats`) for atomic registry access. This ensures that concurrent port allocations from multiple processes never corrupt the registry file.

```typescript
// Semaphore is used internally - no configuration needed
const resolver = new PortResolver();
// All registry operations are protected by semats
```

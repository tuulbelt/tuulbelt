# Getting Started

## Installation

Clone the repository and install dependencies:

```bash
git clone https://github.com/tuulbelt/port-resolver.git
cd port-resolver
npm install
```

## Global CLI Setup

To use `portres` from anywhere:

```bash
npm link
```

Now you can run:

```bash
portres --help
```

## Quick Test

Verify the installation:

```bash
# Allocate a port
portres get --tag "test"

# List allocated ports
portres list

# Clean up
portres release-all
```

## Usage Patterns

### CLI for Scripts

```bash
# In your test setup script
PORT=$(portres get --tag "integration-test")
export TEST_PORT=$PORT

# Run tests
npm test

# Cleanup
portres release --port $PORT
```

### Library for Node.js Tests

```typescript
import { PortResolver } from '@tuulbelt/port-resolver';

const resolver = new PortResolver();

beforeAll(async () => {
  const result = await resolver.get({ tag: 'my-test' });
  if (result.ok) {
    process.env.TEST_PORT = String(result.value.port);
  }
});

afterAll(async () => {
  await resolver.releaseAll('my-test');
});
```

## Configuration

The resolver uses sensible defaults:

| Option | Default | Description |
|--------|---------|-------------|
| `registryPath` | `/tmp/tuulbelt-port-registry.json` | Location of port registry |
| `minPort` | `10000` | Minimum port number |
| `maxPort` | `65535` | Maximum port number |
| `tag` | `undefined` | Optional identifier for grouping |

## Verify Tests Pass

```bash
npm test
```

Expected: All 56 tests pass.

## Next Steps

- [CLI Usage](./cli-usage) - Complete CLI reference
- [Library Usage](./library-usage) - API patterns
- [Examples](./examples) - Real-world scenarios

# Examples

Real-world examples for common test port scenarios.

## Integration Test Server

```typescript
import { createServer } from 'node:http';
import { PortResolver } from '@tuulbelt/test-port-resolver';

async function startTestServer() {
  const resolver = new PortResolver();
  const result = await resolver.get({ tag: 'integration-server' });

  if (!result.ok) {
    throw new Error(`Failed to get port: ${result.error.message}`);
  }

  const port = result.value.port;

  const server = createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ status: 'ok' }));
  });

  return new Promise<{ server: typeof server; port: number; cleanup: () => Promise<void> }>((resolve) => {
    server.listen(port, () => {
      console.log(`Test server running on port ${port}`);
      resolve({
        server,
        port,
        cleanup: async () => {
          server.close();
          await resolver.release(port);
        }
      });
    });
  });
}

// Usage
const { server, port, cleanup } = await startTestServer();
// Run tests against http://localhost:${port}
await cleanup();
```

## Parallel Test Workers

```typescript
// test-setup.ts
import { PortResolver } from '@tuulbelt/test-port-resolver';

const resolver = new PortResolver();

export async function allocateTestPorts(count: number) {
  const result = await resolver.getMultiple(count, {
    tag: `worker-${process.pid}`,
  });

  if (!result.ok) {
    throw result.error;
  }

  return result.value.map(a => a.port);
}

export async function releaseTestPorts() {
  await resolver.releaseAll(`worker-${process.pid}`);
}
```

```typescript
// test-file.test.ts
import { allocateTestPorts, releaseTestPorts } from './test-setup';

let ports: number[];

beforeAll(async () => {
  ports = await allocateTestPorts(3);
  process.env.API_PORT = String(ports[0]);
  process.env.DB_PORT = String(ports[1]);
  process.env.CACHE_PORT = String(ports[2]);
});

afterAll(async () => {
  await releaseTestPorts();
});

test('services communicate correctly', () => {
  // Tests can run in parallel without port conflicts
});
```

## Docker Compose Override

```typescript
// generate-docker-ports.ts
import { writeFileSync } from 'node:fs';
import { PortResolver } from '@tuulbelt/test-port-resolver';

async function generateDockerPorts() {
  const resolver = new PortResolver();

  const services = ['api', 'postgres', 'redis'];
  const ports: Record<string, number> = {};

  for (const service of services) {
    const result = await resolver.get({ tag: `docker-${service}` });
    if (!result.ok) throw result.error;
    ports[service] = result.value.port;
  }

  // Generate docker-compose.override.yml
  const override = `
version: '3.8'
services:
  api:
    ports:
      - "${ports.api}:3000"
  postgres:
    ports:
      - "${ports.postgres}:5432"
  redis:
    ports:
      - "${ports.redis}:6379"
`;

  writeFileSync('docker-compose.override.yml', override);
  console.log('Generated docker-compose.override.yml with dynamic ports');

  return ports;
}

generateDockerPorts();
```

## Express Server Testing

```typescript
import express from 'express';
import { PortResolver } from '@tuulbelt/test-port-resolver';

class TestableServer {
  private resolver = new PortResolver();
  private app = express();
  private server?: ReturnType<typeof express.prototype.listen>;
  private port?: number;

  constructor() {
    this.app.get('/health', (req, res) => {
      res.json({ status: 'healthy' });
    });
  }

  async start(): Promise<string> {
    const result = await this.resolver.get({ tag: 'express-test' });
    if (!result.ok) throw result.error;

    this.port = result.value.port;

    return new Promise((resolve) => {
      this.server = this.app.listen(this.port, () => {
        resolve(`http://localhost:${this.port}`);
      });
    });
  }

  async stop(): Promise<void> {
    if (this.server) {
      this.server.close();
    }
    if (this.port) {
      await this.resolver.release(this.port);
    }
  }
}

// In tests
const server = new TestableServer();
const baseUrl = await server.start();

// Test against baseUrl
const response = await fetch(`${baseUrl}/health`);

await server.stop();
```

## CI Script

```bash
#!/bin/bash
# ci-test.sh - Run tests with dynamic port allocation

set -e

# Allocate ports for all services
API_PORT=$(portres get --tag "ci-api")
DB_PORT=$(portres get --tag "ci-db")
REDIS_PORT=$(portres get --tag "ci-redis")

# Export for test processes
export API_PORT DB_PORT REDIS_PORT

echo "Running tests with ports:"
echo "  API: $API_PORT"
echo "  DB: $DB_PORT"
echo "  Redis: $REDIS_PORT"

# Cleanup function
cleanup() {
  echo "Cleaning up allocated ports..."
  portres release-all --tag "ci-api"
  portres release-all --tag "ci-db"
  portres release-all --tag "ci-redis"
}

# Register cleanup on exit
trap cleanup EXIT

# Run tests
npm test

echo "Tests completed successfully!"
```

## WebSocket Server

```typescript
import { WebSocketServer } from 'ws';
import { PortResolver } from '@tuulbelt/test-port-resolver';

async function createTestWebSocket() {
  const resolver = new PortResolver();
  const result = await resolver.get({ tag: 'websocket-test' });

  if (!result.ok) throw result.error;

  const port = result.value.port;
  const wss = new WebSocketServer({ port });

  wss.on('connection', (ws) => {
    ws.on('message', (message) => {
      ws.send(`echo: ${message}`);
    });
  });

  return {
    url: `ws://localhost:${port}`,
    close: async () => {
      wss.close();
      await resolver.release(port);
    }
  };
}
```

## Multiple Services Test Suite

```typescript
// multi-service-test.ts
import { PortResolver } from '@tuulbelt/test-port-resolver';

interface ServicePorts {
  api: number;
  auth: number;
  notification: number;
  analytics: number;
}

class TestEnvironment {
  private resolver = new PortResolver();
  private ports: ServicePorts | null = null;

  async setup(): Promise<ServicePorts> {
    const services = ['api', 'auth', 'notification', 'analytics'] as const;
    const ports: Partial<ServicePorts> = {};

    for (const service of services) {
      const result = await this.resolver.get({
        tag: `multi-service-${service}`,
        minPort: 8000,
        maxPort: 9000,
      });

      if (!result.ok) {
        // Cleanup any allocated ports on failure
        await this.teardown();
        throw result.error;
      }

      ports[service] = result.value.port;
    }

    this.ports = ports as ServicePorts;
    return this.ports;
  }

  async teardown(): Promise<void> {
    const services = ['api', 'auth', 'notification', 'analytics'];
    for (const service of services) {
      await this.resolver.releaseAll(`multi-service-${service}`);
    }
  }

  getUrl(service: keyof ServicePorts): string {
    if (!this.ports) throw new Error('Environment not set up');
    return `http://localhost:${this.ports[service]}`;
  }
}

// Usage
const env = new TestEnvironment();
const ports = await env.setup();

console.log('Service URLs:');
console.log(`  API: ${env.getUrl('api')}`);
console.log(`  Auth: ${env.getUrl('auth')}`);
console.log(`  Notification: ${env.getUrl('notification')}`);
console.log(`  Analytics: ${env.getUrl('analytics')}`);

// Run tests...

await env.teardown();
```

## Resource Pool Pattern

```typescript
import { PortResolver } from '@tuulbelt/test-port-resolver';

class PortPool {
  private resolver = new PortResolver();
  private available: number[] = [];
  private inUse = new Set<number>();

  constructor(private poolSize: number = 10) {}

  async initialize(): Promise<void> {
    const result = await this.resolver.getMultiple(this.poolSize, {
      tag: 'port-pool',
    });

    if (!result.ok) throw result.error;

    this.available = result.value.map(a => a.port);
  }

  acquire(): number {
    const port = this.available.pop();
    if (!port) throw new Error('No ports available in pool');
    this.inUse.add(port);
    return port;
  }

  release(port: number): void {
    if (this.inUse.has(port)) {
      this.inUse.delete(port);
      this.available.push(port);
    }
  }

  async destroy(): Promise<void> {
    await this.resolver.releaseAll('port-pool');
    this.available = [];
    this.inUse.clear();
  }
}

// Usage
const pool = new PortPool(5);
await pool.initialize();

const port1 = pool.acquire();
const port2 = pool.acquire();

// Use ports...

pool.release(port1);
pool.release(port2);

await pool.destroy();
```

# Examples

Real-world examples and patterns for using Config File Merger.

## CLI Examples

### Basic File Merge

```bash
# Load from a single config file
cfgmerge --file config.json
```

### Override with CLI Arguments

```bash
# Override port and enable debug
cfgmerge --file config.json --args "port=3000,debug=true"
```

### Environment Variables

```bash
# Include all env vars
cfgmerge --env

# Filter by prefix
cfgmerge --env --prefix APP_

# Keep prefix and case
cfgmerge --env --prefix APP_ --no-strip-prefix --no-lowercase
```

### Full Stack Configuration

```bash
# Combine all sources
cfgmerge \
  --defaults defaults.json \
  --file config.json \
  --env --prefix APP_ \
  --args "debug=true" \
  --track-sources
```

### Docker/Container Configuration

```bash
# In Dockerfile or docker-compose
CMD ["cfgmerge", "--defaults", "defaults.json", "--env", "--prefix", "APP_"]

# Local development
cfgmerge --defaults defaults.json --file local.json
```

### CI/CD Pipeline

```bash
# Set environment-specific config
export APP_DATABASE_URL="postgres://prod.db.com/app"
export APP_LOG_LEVEL="info"
export APP_DEBUG="false"

# Merge and save
cfgmerge \
  --defaults defaults.json \
  --env --prefix APP_ \
  > /etc/app/config.json
```

---

## Library Examples

### Basic Merge

```typescript
import { mergeConfig, getValue } from './src/index.js';

const result = mergeConfig({
  defaults: {
    port: 8080,
    host: 'localhost',
    debug: false,
  },
  cli: {
    port: 3000,
    debug: true,
  },
});

if (result.ok) {
  console.log('Merged config:', result.config);
  // { port: 3000, host: 'localhost', debug: true }
}
```

### Environment Configuration

```typescript
import { mergeConfig, getValue } from './src/index.js';

// Simulated environment
const mockEnv = {
  APP_HOST: 'prod.example.com',
  APP_PORT: '443',
  APP_DEBUG: 'false',
  OTHER_VAR: 'ignored',
};

const result = mergeConfig({
  defaults: { port: 8080, host: 'localhost', timeout: 5000 },
  env: mockEnv,
  envPrefix: 'APP_',
  lowercaseEnv: true,
});

if (result.ok) {
  const port = getValue<string>(result.config, 'port');
  const host = getValue<string>(result.config, 'host');
  console.log(`Connecting to ${host}:${port}`);
}
```

### Source Tracking

```typescript
import { mergeConfig } from './src/index.js';

const result = mergeConfig({
  defaults: { timeout: 30000 },
  file: { host: 'api.example.com' },
  cli: { port: 443 },
  trackSources: true,
});

if (result.ok) {
  console.log('Config with sources:');
  console.log(JSON.stringify(result.config, null, 2));
  // {
  //   "timeout": { "value": 30000, "source": "default" },
  //   "host": { "value": "api.example.com", "source": "file" },
  //   "port": { "value": 443, "source": "cli" }
  // }
}
```

### Complete Application Configuration

```typescript
import {
  mergeConfig,
  getValue,
  parseJsonFile,
  parseCliArgs,
} from './src/index.js';

// Configuration schema
interface AppConfig {
  port: number;
  host: string;
  database: {
    url: string;
    pool: number;
  };
  logging: {
    level: string;
    format: string;
  };
  debug: boolean;
}

async function loadConfiguration(): Promise<AppConfig> {
  // Load defaults
  const defaultsResult = parseJsonFile('config/defaults.json');
  if (!defaultsResult.ok) {
    throw new Error(`Failed to load defaults: ${defaultsResult.error}`);
  }

  // Load environment-specific config
  const env = process.env.NODE_ENV || 'development';
  const envConfigResult = parseJsonFile(`config/${env}.json`);
  const envConfig = envConfigResult.ok ? envConfigResult.data : {};

  // Parse CLI args from argv
  const cliArgs = process.argv
    .slice(2)
    .filter(arg => arg.includes('='))
    .join(',');

  const cliConfig: Record<string, unknown> = {};
  if (cliArgs) {
    const parsed = parseCliArgs(cliArgs);
    for (const [key, entry] of Object.entries(parsed)) {
      cliConfig[key] = entry.value;
    }
  }

  // Merge all sources
  const result = mergeConfig({
    defaults: defaultsResult.data,
    file: envConfig,
    env: process.env,
    envPrefix: 'APP_',
    cli: cliConfig,
    trackSources: process.env.DEBUG_CONFIG === 'true',
  });

  if (!result.ok) {
    throw new Error('Configuration merge failed');
  }

  // Extract typed configuration
  const config = result.config;

  return {
    port: getValue<number>(config, 'port', 8080),
    host: getValue<string>(config, 'host', 'localhost'),
    database: {
      url: getValue<string>(config, 'database_url', 'postgres://localhost/app'),
      pool: getValue<number>(config, 'database_pool', 10),
    },
    logging: {
      level: getValue<string>(config, 'log_level', 'info'),
      format: getValue<string>(config, 'log_format', 'json'),
    },
    debug: getValue<boolean>(config, 'debug', false),
  };
}

// Usage
const config = await loadConfiguration();
console.log(`Starting server on ${config.host}:${config.port}`);
```

### Twelve-Factor App Pattern

```typescript
import { mergeConfig, getValue, parseJsonFile } from './src/index.js';

// Twelve-factor: Config in environment
// https://12factor.net/config

function loadTwelveFactorConfig() {
  // Default config for development
  const defaults = {
    port: 3000,
    database_url: 'postgres://localhost/dev',
    redis_url: 'redis://localhost:6379',
    log_level: 'debug',
    secret_key: 'dev-secret-change-me',
  };

  // In production, all config comes from environment
  const result = mergeConfig({
    defaults,
    env: process.env,
    envPrefix: 'APP_',
  });

  if (!result.ok) {
    throw new Error('Configuration error');
  }

  const config = result.config;

  // Validate required production config
  if (process.env.NODE_ENV === 'production') {
    const secretKey = getValue<string>(config, 'secret_key');
    if (secretKey === 'dev-secret-change-me') {
      throw new Error('APP_SECRET_KEY must be set in production');
    }
  }

  return {
    port: getValue<number>(config, 'port', 3000),
    databaseUrl: getValue<string>(config, 'database_url')!,
    redisUrl: getValue<string>(config, 'redis_url')!,
    logLevel: getValue<string>(config, 'log_level', 'info'),
    secretKey: getValue<string>(config, 'secret_key')!,
  };
}
```

### Feature Flags Configuration

```typescript
import { mergeConfig, getValue } from './src/index.js';

interface FeatureFlags {
  newDashboard: boolean;
  betaFeatures: boolean;
  experimentalApi: boolean;
  darkMode: boolean;
}

function loadFeatureFlags(): FeatureFlags {
  const result = mergeConfig({
    defaults: {
      feature_new_dashboard: false,
      feature_beta_features: false,
      feature_experimental_api: false,
      feature_dark_mode: true,
    },
    env: process.env,
    envPrefix: 'FF_',  // FF_FEATURE_NEW_DASHBOARD=true
    cli: parseFeatureFlagsFromArgs(),
  });

  if (!result.ok) {
    // Return all defaults on error
    return {
      newDashboard: false,
      betaFeatures: false,
      experimentalApi: false,
      darkMode: true,
    };
  }

  const config = result.config;

  return {
    newDashboard: getValue<boolean>(config, 'feature_new_dashboard', false),
    betaFeatures: getValue<boolean>(config, 'feature_beta_features', false),
    experimentalApi: getValue<boolean>(config, 'feature_experimental_api', false),
    darkMode: getValue<boolean>(config, 'feature_dark_mode', true),
  };
}

function parseFeatureFlagsFromArgs(): Record<string, unknown> {
  const flags: Record<string, unknown> = {};

  for (const arg of process.argv.slice(2)) {
    if (arg.startsWith('--enable-')) {
      const feature = arg.slice(9).replace(/-/g, '_');
      flags[`feature_${feature}`] = true;
    }
    if (arg.startsWith('--disable-')) {
      const feature = arg.slice(10).replace(/-/g, '_');
      flags[`feature_${feature}`] = false;
    }
  }

  return flags;
}
```

### Multi-Environment Configuration

```typescript
import { mergeConfig, getValue, parseJsonFile } from './src/index.js';

type Environment = 'development' | 'staging' | 'production';

const environmentConfigs: Record<Environment, object> = {
  development: {
    api_url: 'http://localhost:3000',
    debug: true,
    cache_ttl: 0,
  },
  staging: {
    api_url: 'https://staging-api.example.com',
    debug: true,
    cache_ttl: 60,
  },
  production: {
    api_url: 'https://api.example.com',
    debug: false,
    cache_ttl: 3600,
  },
};

function loadConfig(env: Environment = 'development') {
  const result = mergeConfig({
    defaults: {
      port: 3000,
      timeout: 30000,
    },
    file: environmentConfigs[env],
    env: process.env,
    envPrefix: 'APP_',
    trackSources: env === 'development',
  });

  if (!result.ok) {
    throw new Error('Configuration error');
  }

  return result.config;
}

// Usage
const env = (process.env.NODE_ENV || 'development') as Environment;
const config = loadConfig(env);
console.log('Loaded config for:', env);
```

---

## Configuration File Patterns

### defaults.json

```json
{
  "port": 8080,
  "host": "0.0.0.0",
  "timeout": 30000,
  "retries": 3,
  "logging": {
    "level": "info",
    "format": "json"
  },
  "features": {
    "experimental": false
  }
}
```

### development.json

```json
{
  "port": 3000,
  "host": "localhost",
  "logging": {
    "level": "debug",
    "format": "pretty"
  },
  "features": {
    "experimental": true
  }
}
```

### production.json

```json
{
  "timeout": 60000,
  "retries": 5,
  "logging": {
    "level": "warn"
  }
}
```

---

## Tips and Best Practices

1. **Layered configuration** - Use defaults → environment file → env vars → CLI
2. **Validate required fields** - Check critical config after merge
3. **Use type assertions** - Leverage TypeScript for type safety
4. **Track sources in dev** - Enable `trackSources` during development
5. **Namespace env vars** - Use consistent prefixes like `APP_`
6. **Document config options** - Maintain a config schema or docs
7. **Secure secrets** - Never commit secrets; use env vars in production

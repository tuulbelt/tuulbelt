# Examples

Real-world validation scenarios using Property Validator.

## API Response Validation

Validate external API responses to ensure data integrity.

```typescript
import { object, string, number, nullable, array, validate } from '@tuulbelt/property-validator';

const GitHubUserSchema = object({
  login: string(),
  id: number(),
  avatar_url: string().url(),
  type: string(),
  name: nullable(string()),
  email: nullable(string().email()),
  bio: nullable(string())
});

async function fetchGitHubUser(username: string) {
  const response = await fetch(`https://api.github.com/users/${username}`);
  const data = await response.json();

  const result = validate(GitHubUserSchema, data);

  if (!result.ok) {
    throw new Error(`Invalid GitHub API response: ${result.error.format('text')}`);
  }

  return result.value;  // Type-safe user object
}

// Usage
const user = await fetchGitHubUser('octocat');
console.log(`User: ${user.login}, ID: ${user.id}`);
```

## Configuration File Validation

Validate application configuration with defaults and built-in validators.

```typescript
import { readFileSync } from 'node:fs';
import { object, string, number, boolean, optional, validate } from '@tuulbelt/property-validator';

const ConfigSchema = object({
  server: object({
    port: number().int().range(1, 65535).default(3000),
    host: string().default('localhost'),
    ssl: boolean().default(false)
  }),
  database: object({
    url: string().url(),
    poolSize: number().int().positive().default(10)
  }),
  logging: optional(object({
    level: string().default('info'),
    file: optional(string())
  }))
});

function loadConfig(path: string) {
  const content = readFileSync(path, 'utf-8');
  const data = JSON.parse(content);

  const result = validate(ConfigSchema, data);

  if (!result.ok) {
    console.error(`Configuration error: ${result.error.format('color')}`);
    process.exit(1);
  }

  return result.value;
}

// Usage - with partial config (defaults applied)
const config = loadConfig('./config.json');
console.log(`Server running on ${config.server.host}:${config.server.port}`);
```

## User Registration with Built-in Validators

Validate user input with email, password complexity, and age constraints.

```typescript
import { object, string, number, boolean, optional, validate } from '@tuulbelt/property-validator';

const UserRegistrationSchema = object({
  username: string().min(3).max(20).pattern(/^[a-z0-9_]+$/),
  email: string().email(),
  password: string()
    .min(8)
    .refine(s => /[A-Z]/.test(s), 'Must contain uppercase')
    .refine(s => /[0-9]/.test(s), 'Must contain number'),
  age: optional(number().int().positive().max(150)),
  agreedToTerms: boolean()
});

function registerUser(formData: unknown) {
  const result = validate(UserRegistrationSchema, formData);

  if (!result.ok) {
    return {
      success: false,
      error: result.error.message,
      field: result.error.path[0]
    };
  }

  const user = result.value;

  if (!user.agreedToTerms) {
    return {
      success: false,
      error: 'You must agree to terms',
      field: 'agreedToTerms'
    };
  }

  // Create user account...
  return { success: true, user };
}
```

## High-Throughput Data Filtering

Use `check()` and `compileCheck()` for performance-critical filtering.

```typescript
import { object, number, string, check, compileCheck } from '@tuulbelt/property-validator';

// Schema for valid sensor data
const SensorReadingSchema = object({
  timestamp: number().positive(),
  value: number().finite(),
  sensorId: string().uuid()
});

// Pre-compile for maximum performance
const isValidReading = compileCheck(SensorReadingSchema);

// Process millions of readings efficiently
function processSensorData(readings: unknown[]) {
  // Filter invalid readings (~55ns per check)
  const validReadings = readings.filter(isValidReading);

  // Process valid data
  const average = validReadings.reduce(
    (sum, r) => sum + r.value,
    0
  ) / validReadings.length;

  return {
    processed: validReadings.length,
    dropped: readings.length - validReadings.length,
    average
  };
}

// Alternative: Use check() for simple filtering
const UserSchema = object({ name: string(), age: number() });
const validUsers = users.filter(u => check(UserSchema, u));
```

## Nested Object Validation

Validate complex nested data structures with nested schemas.

```typescript
import { object, string, number, array, validate } from '@tuulbelt/property-validator';

const AddressSchema = object({
  street: string().min(1),
  city: string().min(1),
  state: string().length(2),
  zip: string().pattern(/^\d{5}(-\d{4})?$/),
  country: string().default('USA')
});

const EmployeeSchema = object({
  id: number().int().positive(),
  name: string().min(1),
  email: string().email(),
  role: string(),
  department: string()
});

const CompanySchema = object({
  name: string().min(1),
  website: string().url(),
  address: AddressSchema,
  employees: array(EmployeeSchema).nonempty()
});

const companyData = {
  name: "Acme Corp",
  website: "https://acme.com",
  address: {
    street: "123 Main St",
    city: "Springfield",
    state: "IL",
    zip: "62701"
  },
  employees: [
    { id: 1, name: "Alice", email: "alice@acme.com", role: "Engineer", department: "Engineering" },
    { id: 2, name: "Bob", email: "bob@acme.com", role: "Manager", department: "Sales" }
  ]
};

const result = validate(CompanySchema, companyData);

if (result.ok) {
  console.log(`Company: ${result.value.name}`);
  console.log(`Employees: ${result.value.employees.length}`);
  console.log(`Country: ${result.value.address.country}`);  // "USA" (default)
}
```

## Discriminated Unions

Validate polymorphic data with type-safe discriminated unions.

```typescript
import { object, string, number, literal, union, array, validate } from '@tuulbelt/property-validator';

// Different notification types
const EmailNotificationSchema = object({
  type: literal('email'),
  to: string().email(),
  subject: string().min(1),
  body: string()
});

const SMSNotificationSchema = object({
  type: literal('sms'),
  phone: string().pattern(/^\+?[1-9]\d{1,14}$/),
  message: string().max(160)
});

const PushNotificationSchema = object({
  type: literal('push'),
  deviceId: string().uuid(),
  title: string(),
  body: string()
});

const NotificationSchema = union([
  EmailNotificationSchema,
  SMSNotificationSchema,
  PushNotificationSchema
]);

const NotificationListSchema = array(NotificationSchema);

// Process notifications
const notifications = [
  { type: 'email', to: 'user@example.com', subject: 'Hello', body: 'Hi there!' },
  { type: 'sms', phone: '+1234567890', message: 'Quick alert' },
  { type: 'push', deviceId: '550e8400-e29b-41d4-a716-446655440000', title: 'Update', body: 'New version available' }
];

const result = validate(NotificationListSchema, notifications);

if (result.ok) {
  for (const notification of result.value) {
    switch (notification.type) {
      case 'email':
        sendEmail(notification.to, notification.subject, notification.body);
        break;
      case 'sms':
        sendSMS(notification.phone, notification.message);
        break;
      case 'push':
        sendPush(notification.deviceId, notification.title, notification.body);
        break;
    }
  }
}
```

## Error Formatting Options

Different ways to format and display validation errors.

```typescript
import { object, string, number, validate, ValidationError } from '@tuulbelt/property-validator';

const UserSchema = object({
  name: string().min(1),
  email: string().email(),
  age: number().int().positive()
});

const invalidData = {
  name: "",
  email: "not-an-email",
  age: -5
};

const result = validate(UserSchema, invalidData);

if (!result.ok) {
  const { error } = result;

  // Text format (for logs)
  console.log(error.format('text'));
  // "Validation failed at name: expected non-empty string, got ''"

  // JSON format (for APIs)
  console.log(error.format('json'));
  // {"path":"name","expected":"non-empty string","actual":""}

  // Colored format (for terminals)
  console.log(error.format('color'));
  // ANSI-colored output

  // Custom error response
  const apiResponse = {
    error: {
      field: error.formatPathString(),  // "name"
      message: error.message,
      expected: error.expected,
      actual: error.actual
    }
  };
}
```

## Data Transformation Pipeline

Combine validation with data transformation.

```typescript
import { object, string, number, validate } from '@tuulbelt/property-validator';

// Raw API data schema
const RawUserSchema = object({
  user_id: number(),
  user_name: string(),
  created_at: string(),
  email_address: string().email()
});

// Transformation function
function transformApiResponse(rawData: unknown) {
  const result = validate(RawUserSchema, rawData);

  if (!result.ok) {
    return null;
  }

  // Transform snake_case to camelCase
  return {
    id: result.value.user_id,
    name: result.value.user_name,
    email: result.value.email_address,
    createdAt: new Date(result.value.created_at)
  };
}

// Or use .transform() for inline transformation
const EmailSchema = string()
  .email()
  .transform(s => s.toLowerCase().trim());

const result = validate(EmailSchema, "  USER@Example.COM  ");
// result.value === "user@example.com"
```

## Conditional Validation with Type Guards

Use validation results as TypeScript type guards.

```typescript
import { object, string, number, check, validate } from '@tuulbelt/property-validator';

const UserSchema = object({
  name: string(),
  age: number()
});

type User = { name: string; age: number };

// Type guard function
function isUser(data: unknown): data is User {
  return check(UserSchema, data);
}

// Usage in code
function processData(data: unknown) {
  if (isUser(data)) {
    // TypeScript knows data is User here
    console.log(`Processing user: ${data.name}, age ${data.age}`);
  } else {
    console.log('Invalid user data');
  }
}

// Or with full validation for detailed errors
function processDataWithErrors(data: unknown) {
  const result = validate(UserSchema, data);

  if (result.ok) {
    const user = result.value;  // Type-safe User
    console.log(`Processing user: ${user.name}`);
  } else {
    console.error(`Validation failed: ${result.error.message}`);
  }
}
```

## Next Steps

- [API Reference](/tools/property-validator/api-reference) - Complete API documentation
- [Library Usage](/tools/property-validator/library-usage) - Comprehensive API guide
- [CLI Usage](/tools/property-validator/cli-usage) - Command-line interface
- [Getting Started](/tools/property-validator/getting-started) - Quick start guide

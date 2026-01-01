# Examples

Real-world validation scenarios using Property Validator.

## API Response Validation

Validate external API responses to ensure data integrity.

```typescript
import { v, validate } from 'property-validator';

const GitHubUserSchema = v.object({
  login: v.string(),
  id: v.number(),
  avatar_url: v.string(),
  type: v.string(),
  name: v.nullable(v.string()),
  email: v.nullable(v.string()),
  bio: v.nullable(v.string())
});

async function fetchGitHubUser(username: string) {
  const response = await fetch(`https://api.github.com/users/${username}`);
  const data = await response.json();

  const result = validate(data, GitHubUserSchema);

  if (!result.ok) {
    throw new Error(`Invalid GitHub API response: ${result.error.message}`);
  }

  return result.value;  // Type-safe user object
}

// Usage
const user = await fetchGitHubUser('octocat');
console.log(`User: ${user.login}, ID: ${user.id}`);
```

## Configuration File Validation

Validate application configuration files.

```typescript
import { readFileSync } from 'fs';
import { v, validate } from 'property-validator';

const ConfigSchema = v.object({
  server: v.object({
    port: v.number(),
    host: v.string(),
    ssl: v.optional(v.boolean())
  }),
  database: v.object({
    url: v.string(),
    poolSize: v.optional(v.number())
  }),
  logging: v.optional(v.object({
    level: v.string(),
    file: v.optional(v.string())
  }))
});

function loadConfig(path: string) {
  const content = readFileSync(path, 'utf-8');
  const data = JSON.parse(content);

  const result = validate(data, ConfigSchema);

  if (!result.ok) {
    console.error(`Configuration error: ${result.error.message}`);
    console.error(`  Path: ${result.error.path.join('.')}`);
    console.error(`  Expected: ${result.error.expected}`);
    process.exit(1);
  }

  return result.value;
}

// Usage
const config = loadConfig('./config.json');
console.log(`Server running on ${config.server.host}:${config.server.port}`);
```

## User Input Validation

Validate user input from forms or CLI arguments.

```typescript
import { v, validate } from 'property-validator';

const UserRegistrationSchema = v.object({
  username: v.string(),
  email: v.string(),
  password: v.string(),
  age: v.optional(v.number()),
  agreedToTerms: v.boolean()
});

function registerUser(formData: unknown) {
  const result = validate(formData, UserRegistrationSchema);

  if (!result.ok) {
    return {
      success: false,
      error: result.error.message,
      field: result.error.path[0]
    };
  }

  // Validated data is type-safe
  const user = result.value;

  // Additional validation (business logic)
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

## Nested Object Validation

Validate complex nested data structures.

```typescript
import { v, validate } from 'property-validator';

const AddressSchema = v.object({
  street: v.string(),
  city: v.string(),
  state: v.string(),
  zip: v.string(),
  country: v.string()
});

const CompanySchema = v.object({
  name: v.string(),
  address: AddressSchema,
  employees: v.array(v.object({
    id: v.number(),
    name: v.string(),
    role: v.string(),
    department: v.string()
  }))
});

const companyData = {
  name: "Acme Corp",
  address: {
    street: "123 Main St",
    city: "Springfield",
    state: "IL",
    zip: "62701",
    country: "USA"
  },
  employees: [
    { id: 1, name: "Alice", role: "Engineer", department: "Engineering" },
    { id: 2, name: "Bob", role: "Manager", department: "Sales" }
  ]
};

const result = validate(companyData, CompanySchema);

if (result.ok) {
  console.log(`Company: ${result.value.name}`);
  console.log(`Employees: ${result.value.employees.length}`);
}
```

## Array Validation

Validate arrays of objects with specific schemas.

```typescript
import { v, validate } from 'property-validator';

const TodoSchema = v.object({
  id: v.number(),
  title: v.string(),
  completed: v.boolean(),
  dueDate: v.nullable(v.string())
});

const TodoListSchema = v.array(TodoSchema);

function validateTodoList(data: unknown) {
  const result = validate(data, TodoListSchema);

  if (!result.ok) {
    console.error(`Invalid todo list: ${result.error.message}`);
    return [];
  }

  return result.value;
}

// Usage
const todos = validateTodoList([
  { id: 1, title: "Buy milk", completed: false, dueDate: null },
  { id: 2, title: "Write docs", completed: true, dueDate: "2026-01-15" }
]);
```

## Error Message Handling

Different ways to handle validation errors.

```typescript
import { v, validate } from 'property-validator';

const UserSchema = v.object({
  name: v.string(),
  age: v.number()
});

// Simple error message
const result1 = validate({ name: "Alice", age: "thirty" }, UserSchema);
if (!result1.ok) {
  console.error(result1.error.message);
  // "Validation failed at age: expected number, got string"
}

// Detailed error breakdown
const result2 = validate({ name: 123, age: 30 }, UserSchema);
if (!result2.ok) {
  console.error(`Field: ${result2.error.path.join('.')}`);
  console.error(`Expected: ${result2.error.expected}`);
  console.error(`Got: ${result2.error.actual}`);
  // Field: name
  // Expected: string
  // Got: number
}

// Custom error handling
function handleValidationError(error: ValidationError) {
  return {
    field: error.path[0],
    message: `The field '${error.path[0]}' must be a ${error.expected}`,
    code: 'VALIDATION_ERROR'
  };
}

const result3 = validate(invalidData, UserSchema);
if (!result3.ok) {
  const formatted = handleValidationError(result3.error);
  console.log(formatted);
  // { field: 'age', message: "The field 'age' must be a number", code: 'VALIDATION_ERROR' }
}
```

## Type-Safe Data Transformation

Use validation to safely transform external data.

```typescript
import { v, validate } from 'property-validator';

const RawDataSchema = v.object({
  user_id: v.number(),
  user_name: v.string(),
  created_at: v.string()
});

function transformUser(rawData: unknown) {
  const result = validate(rawData, RawDataSchema);

  if (!result.ok) {
    return null;
  }

  // Transform to application format
  return {
    id: result.value.user_id,
    name: result.value.user_name,
    createdAt: new Date(result.value.created_at)
  };
}

// Usage
const rawUser = {
  user_id: 123,
  user_name: "Alice",
  created_at: "2026-01-01T00:00:00Z"
};

const user = transformUser(rawUser);
if (user) {
  console.log(`User ${user.name} created at ${user.createdAt}`);
}
```

## Next Steps

- [API Reference](/tools/property-validator/api-reference) - Complete API documentation
- [Library Usage](/tools/property-validator/library-usage) - Import and use in code
- [CLI Usage](/tools/property-validator/cli-usage) - Command-line interface

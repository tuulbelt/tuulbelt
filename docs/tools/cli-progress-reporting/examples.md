# Examples

Real-world usage patterns for CLI Progress Reporting.

## Batch File Processing

Track progress while processing multiple files:

```bash
#!/bin/bash

TASK_ID="process-images-$(date +%s)"
FILES=(images/*.jpg)
TOTAL=${#FILES[@]}

# Initialize progress
prog init --total $TOTAL --message "Processing images" --id "$TASK_ID"

# Process each file
for i in "${!FILES[@]}"; do
  file="${FILES[$i]}"

  # Process the file
  convert "$file" -resize 800x600 "thumbnails/$(basename "$file")"

  # Update progress
  prog increment \
    --message "Processed $(basename "$file")" \
    --id "$TASK_ID"
done

# Mark complete
prog finish --message "All images processed" --id "$TASK_ID"
```

## Parallel Workers

Track progress across multiple concurrent processes:

```bash
#!/bin/bash

TASK_ID="parallel-download"
TOTAL_URLS=100

# Initialize
prog init --total $TOTAL_URLS --message "Downloading files" --id "$TASK_ID"

# Function to download and update progress
download_file() {
  local url=$1
  local output=$2
  curl -s "$url" -o "$output"
  prog increment --id "$TASK_ID"
}

export -f download_file
export TASK_ID

# Download 10 files in parallel
cat urls.txt | xargs -P 10 -I {} bash -c 'download_file "$@"' _ {}

prog finish --message "All downloads complete" --id "$TASK_ID"
```

## CI/CD Pipeline

Track multi-stage build progress:

```bash
#!/bin/bash

PIPELINE_ID="build-${CI_COMMIT_SHA}"

# 5 stages: lint, test, build, package, deploy
prog init --total 5 --message "Starting pipeline" --id "$PIPELINE_ID"

# Stage 1: Lint
npm run lint
prog increment --message "Lint complete" --id "$PIPELINE_ID"

# Stage 2: Test
npm test
prog increment --message "Tests passed" --id "$PIPELINE_ID"

# Stage 3: Build
npm run build
prog increment --message "Build complete" --id "$PIPELINE_ID"

# Stage 4: Package
tar -czf dist.tar.gz dist/
prog increment --message "Package created" --id "$PIPELINE_ID"

# Stage 5: Deploy
scp dist.tar.gz deploy@server:/releases/
prog finish --message "Deployed successfully" --id "$PIPELINE_ID"
```

## Database Migration

Track progress through database migration steps:

```bash
#!/bin/bash

MIGRATION_ID="db-migration-v2.0"

# Count migration files
MIGRATIONS=(migrations/*.sql)
TOTAL=${#MIGRATIONS[@]}

prog init --total $TOTAL --message "Running migrations" --id "$MIGRATION_ID"

for migration in "${MIGRATIONS[@]}"; do
  echo "Running $(basename "$migration")..."
  psql -f "$migration" database_name

  prog increment \
    --message "Applied $(basename "$migration")" \
    --id "$MIGRATION_ID"
done

prog finish --message "All migrations complete" --id "$MIGRATION_ID"
```

## Progress Monitoring Script

Monitor progress from a separate script:

```bash
#!/bin/bash
# monitor.sh - Watch progress in real-time

TASK_ID=$1

if [ -z "$TASK_ID" ]; then
  echo "Usage: $0 <task-id>"
  exit 1
fi

echo "Monitoring progress for: $TASK_ID"
echo "Press Ctrl+C to stop"
echo

while true; do
  clear

  # Get current progress
  PROGRESS=$(prog get --id "$TASK_ID" 2>/dev/null)

  if [ $? -eq 0 ]; then
    # Parse JSON and display
    echo "$PROGRESS" | jq -r '"[\(.percentage)%] \(.current)/\(.total) - \(.message)"'

    # Check if complete
    COMPLETE=$(echo "$PROGRESS" | jq -r '.complete')
    if [ "$COMPLETE" = "true" ]; then
      echo
      echo "Task complete!"
      break
    fi
  else
    echo "Waiting for progress to start..."
  fi

  sleep 1
done
```

Usage:
```bash
# Terminal 1: Run the job
./job.sh

# Terminal 2: Monitor progress
./monitor.sh myjob
```

## TypeScript/Node.js Integration

Use the library API in Node.js applications:

```typescript
import { init, increment, finish, formatProgress } from './src/index.js';

async function processItems(items: string[]) {
  const config = { id: 'batch-processor' };

  // Initialize
  const initResult = init(items.length, 'Processing items', config);
  if (!initResult.ok) {
    throw new Error(initResult.error);
  }

  console.log(formatProgress(initResult.value));

  // Process each item
  for (const [index, item] of items.entries()) {
    await processItem(item);

    const result = increment(1, `Processed ${index + 1} items`, config);
    if (result.ok) {
      console.log(formatProgress(result.value));
    }
  }

  // Finish
  const finalResult = finish('All items processed!', config);
  if (finalResult.ok) {
    console.log(formatProgress(finalResult.value));
  }
}

async function processItem(item: string): Promise<void> {
  // Simulate async work
  await new Promise(resolve => setTimeout(resolve, 100));
  console.log(`Processed: ${item}`);
}

// Run
const items = ['item1', 'item2', 'item3'];
await processItems(items);
```

## Error Recovery

Handle failures and resume progress:

```bash
#!/bin/bash

TASK_ID="resilient-job"
CHECKPOINT_FILE="checkpoint.txt"

# Check if we're resuming
if [ -f "$CHECKPOINT_FILE" ]; then
  RESUME_FROM=$(cat "$CHECKPOINT_FILE")
  echo "Resuming from item $RESUME_FROM"

  # Get existing progress
  CURRENT=$(prog get --id "$TASK_ID" | jq -r '.current')
  START=$CURRENT
else
  START=0
  TOTAL=100
  prog init --total $TOTAL --message "Starting job" --id "$TASK_ID"
fi

# Process items
for i in $(seq $START 99); do
  # Do work (might fail)
  if ! process_item $i; then
    echo "Failed at item $i"
    echo $i > "$CHECKPOINT_FILE"
    exit 1
  fi

  prog increment --id "$TASK_ID"
  echo $((i + 1)) > "$CHECKPOINT_FILE"
done

# Success - cleanup
rm -f "$CHECKPOINT_FILE"
prog finish --message "Job complete" --id "$TASK_ID"
```

## Testing with Progress

Use in test suites to track test execution:

```typescript
import { test } from 'node:test';
import { init, increment, finish } from './src/index.js';

const tests = [
  () => test('test 1', () => { /* ... */ }),
  () => test('test 2', () => { /* ... */ }),
  () => test('test 3', () => { /* ... */ }),
];

const config = { id: 'test-run' };
init(tests.length, 'Running tests', config);

for (const testFn of tests) {
  await testFn();
  increment(1, undefined, config);
}

finish('All tests complete', config);
```

See the [API Reference](/tools/cli-progress-reporting/api-reference) for complete function documentation.

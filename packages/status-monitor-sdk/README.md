# @status-monitor/sdk

TypeScript SDK for sending heartbeats to Status Monitor from internal services that don't have public URLs.

## Installation

```bash
npm install @status-monitor/sdk
# or
pnpm add @status-monitor/sdk
```

## Usage

### Basic Example

```typescript
import { createHeartbeatClient } from '@status-monitor/sdk';

const client = createHeartbeatClient({
  monitorId: 'your-monitor-id',
  apiUrl: 'https://your-status-monitor.com/api',
  apiKey: 'your-api-key', // optional
  interval: 60000, // 1 minute (default)
});

// Start sending heartbeats
await client.start();

// Stop when shutting down
process.on('SIGTERM', () => {
  client.stop();
  process.exit(0);
});
```

### Advanced Example

```typescript
import { HeartbeatClient } from '@status-monitor/sdk';

const client = new HeartbeatClient({
  monitorId: 'worker-service',
  apiUrl: process.env.STATUS_MONITOR_URL!,
  apiKey: process.env.STATUS_MONITOR_API_KEY,
  interval: 30000, // 30 seconds
  timeout: 5000, // 5 seconds timeout
  metadata: {
    version: process.env.npm_package_version,
    environment: process.env.NODE_ENV,
    hostname: os.hostname(),
  },
});

// Handle errors
client.on('error', (error) => {
  console.error('Heartbeat error:', error);
});

// Start monitoring
await client.start();

// Get statistics
const stats = client.getStats();
console.log(`Heartbeats sent: ${stats.sent}, failed: ${stats.failed}`);
```

### Express Middleware Example

```typescript
import express from 'express';
import { createHeartbeatClient } from '@status-monitor/sdk';

const app = express();

const heartbeat = createHeartbeatClient({
  monitorId: 'api-service',
  apiUrl: process.env.STATUS_MONITOR_URL!,
});

// Start heartbeat when server starts
app.listen(3000, async () => {
  await heartbeat.start();
  console.log('Server running with heartbeat monitoring');
});
```

## API Reference

### HeartbeatConfig

```typescript
interface HeartbeatConfig {
  monitorId: string;      // Unique identifier for your service
  apiUrl: string;         // Status Monitor API endpoint
  apiKey?: string;        // Optional API key for authentication
  interval?: number;      // Heartbeat interval in ms (default: 60000)
  timeout?: number;       // Request timeout in ms (default: 5000)
  metadata?: Record<string, unknown>; // Additional data to send
}
```

### HeartbeatClient Methods

- `start(): Promise<void>` - Start sending heartbeats
- `stop(): void` - Stop sending heartbeats
- `sendHeartbeat(): Promise<HeartbeatResponse>` - Send a single heartbeat
- `getStatus(): HeartbeatStatus` - Get current client status
- `getStats(): HeartbeatStats` - Get statistics

## License

MIT
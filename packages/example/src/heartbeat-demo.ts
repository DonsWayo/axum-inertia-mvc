import { createHeartbeatClient } from '@status-monitor/sdk';

console.log('Starting Status Monitor SDK Demo with Bun');

// Create a heartbeat client
const client = createHeartbeatClient({
  monitorId: '13', // CRM Worker monitor
  apiUrl: 'http://localhost:8000/api',
  interval: 5000, // 5 seconds for demo
  metadata: {
    runtime: 'bun',
    version: Bun.version,
    service: 'crm-worker',
    environment: 'development',
    pid: process.pid,
    hostname: 'crm-worker-01',
  },
});

async function runDemo() {
  try {
    console.log('Starting heartbeat client...');
    await client.start();
    console.log('Heartbeat client started successfully');

    // Log stats every 3 seconds
    setInterval(() => {
      const stats = client.getStats();
      console.log(`Stats - Status: ${client.getStatus()}, Sent: ${stats.sent}, Failed: ${stats.failed}, Uptime: ${Math.floor(stats.uptime / 1000)}s`);
    }, 3000);

    // Test manual heartbeat
    setTimeout(async () => {
      console.log('Sending manual heartbeat...');
      try {
        const response = await client.sendHeartbeat();
        console.log('Manual heartbeat response:', response);
      } catch (error) {
        console.error('Manual heartbeat failed:', error);
      }
    }, 2000);

  } catch (error) {
    console.error('Failed to start client:', error);
  }
}

// Handle shutdown gracefully
process.on('SIGINT', () => {
  console.log('\nShutting down...');
  client.stop();
  console.log('Goodbye');
  process.exit(0);
});

// Run the demo
runDemo();
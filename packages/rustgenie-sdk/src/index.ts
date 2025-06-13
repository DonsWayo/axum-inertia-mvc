import { HeartbeatClient } from './heartbeat-client.js';

export { HeartbeatClient };
export type {
  HeartbeatConfig,
  HeartbeatResponse,
  HeartbeatError,
  HeartbeatStatus,
  HeartbeatStats,
} from './types.js';

export function createHeartbeatClient(config: import('./types.js').HeartbeatConfig) {
  return new HeartbeatClient(config);
}
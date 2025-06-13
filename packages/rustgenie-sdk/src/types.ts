export interface HeartbeatConfig {
  monitorId: string;
  apiUrl: string;
  apiKey?: string;
  interval?: number; // milliseconds, default 60000 (1 minute)
  timeout?: number; // milliseconds, default 5000
  metadata?: Record<string, unknown>;
}

export interface HeartbeatResponse {
  success: boolean;
  timestamp: string;
  message?: string;
}

export interface HeartbeatError extends Error {
  code?: string;
  statusCode?: number;
}

export type HeartbeatStatus = 'idle' | 'running' | 'stopped' | 'error';

export interface HeartbeatStats {
  sent: number;
  failed: number;
  lastSuccess?: Date;
  lastError?: Date;
  uptime: number; // milliseconds
}
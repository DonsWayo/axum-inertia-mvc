import type { HeartbeatConfig, HeartbeatResponse, HeartbeatError, HeartbeatStatus, HeartbeatStats } from './types.js';

export class HeartbeatClient {
  private config: HeartbeatConfig & { interval: number; timeout: number; metadata: Record<string, unknown> };
  private intervalId?: NodeJS.Timeout;
  private status: HeartbeatStatus = 'idle';
  private stats: HeartbeatStats = {
    sent: 0,
    failed: 0,
    uptime: 0,
  };
  private startTime?: Date;

  constructor(config: HeartbeatConfig) {
    this.config = {
      interval: 60000, // 1 minute default
      timeout: 5000, // 5 seconds default
      metadata: {},
      ...config,
    };
    
    if (!config.monitorId) {
      throw new Error('monitorId is required');
    }
    
    if (!config.apiUrl) {
      throw new Error('apiUrl is required');
    }
  }

  async start(): Promise<void> {
    if (this.status === 'running') {
      throw new Error('Heartbeat client is already running');
    }

    this.status = 'running';
    this.startTime = new Date();
    
    // Send first heartbeat immediately
    await this.sendHeartbeat();
    
    // Schedule periodic heartbeats
    this.intervalId = setInterval(async () => {
      await this.sendHeartbeat();
    }, this.config.interval);
  }

  stop(): void {
    if (this.intervalId) {
      clearInterval(this.intervalId);
      this.intervalId = undefined;
    }
    this.status = 'stopped';
  }

  async sendHeartbeat(): Promise<HeartbeatResponse> {
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), this.config.timeout);

      const headers: HeadersInit = {
        'Content-Type': 'application/json',
      };

      if (this.config.apiKey) {
        headers['Authorization'] = `Bearer ${this.config.apiKey}`;
      }

      const response = await fetch(`${this.config.apiUrl}/heartbeat/${this.config.monitorId}`, {
        method: 'POST',
        headers,
        body: JSON.stringify({
          timestamp: new Date().toISOString(),
          metadata: this.config.metadata,
          stats: this.getStats(),
        }),
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        throw this.createError(
          `Heartbeat failed: ${response.statusText}`,
          'HEARTBEAT_FAILED',
          response.status
        );
      }

      const data = await response.json() as HeartbeatResponse;
      
      this.stats.sent++;
      this.stats.lastSuccess = new Date();
      
      return data;
    } catch (error) {
      this.stats.failed++;
      this.stats.lastError = new Date();
      this.status = 'error';
      
      if (error instanceof Error) {
        throw error;
      }
      
      throw this.createError('Unknown error occurred', 'UNKNOWN_ERROR');
    }
  }

  getStatus(): HeartbeatStatus {
    return this.status;
  }

  getStats(): HeartbeatStats {
    return {
      ...this.stats,
      uptime: this.startTime ? Date.now() - this.startTime.getTime() : 0,
    };
  }

  private createError(message: string, code?: string, statusCode?: number): HeartbeatError {
    const error = new Error(message) as HeartbeatError;
    error.code = code;
    error.statusCode = statusCode;
    return error;
  }
}
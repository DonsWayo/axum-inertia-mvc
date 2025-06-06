-- Add refresh policies for continuous aggregates
-- These will automatically refresh the aggregates at regular intervals

-- Refresh hourly stats every 30 minutes for the last 6 hours
SELECT add_continuous_aggregate_policy('status_hourly_stats',
    start_offset => INTERVAL '6 hours',
    end_offset => INTERVAL '0 minutes',
    schedule_interval => INTERVAL '30 minutes',
    if_not_exists => TRUE
);

-- Refresh daily stats every hour for the last 7 days  
SELECT add_continuous_aggregate_policy('status_daily_stats',
    start_offset => INTERVAL '7 days',
    end_offset => INTERVAL '0 minutes',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => TRUE
);
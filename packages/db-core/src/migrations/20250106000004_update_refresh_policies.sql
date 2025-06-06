-- Drop existing refresh policies
SELECT remove_continuous_aggregate_policy('status_hourly_stats');
SELECT remove_continuous_aggregate_policy('status_daily_stats');

-- Add new refresh policies with 1-minute intervals
SELECT add_continuous_aggregate_policy('status_hourly_stats',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '1 minute', 
    schedule_interval => INTERVAL '1 minute');

SELECT add_continuous_aggregate_policy('status_daily_stats',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute');

-- Force immediate refresh to sync data
CALL refresh_continuous_aggregate('status_hourly_stats', NOW() - INTERVAL '1 day', NOW());
CALL refresh_continuous_aggregate('status_daily_stats', NOW() - INTERVAL '1 day', NOW());
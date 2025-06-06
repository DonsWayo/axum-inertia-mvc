-- Note: This migration creates continuous aggregates which cannot run in a transaction
-- If using SQLx migrations, you may need to run this separately

-- Add a continuous aggregate for hourly stats
CREATE MATERIALIZED VIEW status_hourly_stats
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 hour', time) AS bucket,
    monitor_id,
    COUNT(*) as check_count,
    COUNT(CASE WHEN status = 'operational' THEN 1 END) as operational_count,
    COUNT(CASE WHEN status != 'operational' THEN 1 END) as incident_count,
    AVG(response_time)::INTEGER as avg_response_time,
    MIN(response_time) as min_response_time,
    MAX(response_time) as max_response_time,
    percentile_cont(0.95) WITHIN GROUP (ORDER BY response_time)::INTEGER as p95_response_time
FROM status_events
GROUP BY bucket, monitor_id
WITH NO DATA;

-- Add refresh policy for continuous aggregate (refresh every hour)
SELECT add_continuous_aggregate_policy('status_hourly_stats',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

-- Create a daily aggregate view
CREATE MATERIALIZED VIEW status_daily_stats
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 day', time) AS bucket,
    monitor_id,
    COUNT(*) as check_count,
    COUNT(CASE WHEN status = 'operational' THEN 1 END) as operational_count,
    COUNT(CASE WHEN status != 'operational' THEN 1 END) as incident_count,
    (COUNT(CASE WHEN status = 'operational' THEN 1 END)::FLOAT / COUNT(*)::FLOAT * 100)::NUMERIC(5,2) as uptime_percentage,
    AVG(response_time)::INTEGER as avg_response_time,
    percentile_cont(0.95) WITHIN GROUP (ORDER BY response_time)::INTEGER as p95_response_time
FROM status_events
GROUP BY bucket, monitor_id
WITH NO DATA;

-- Add refresh policy for daily aggregate
SELECT add_continuous_aggregate_policy('status_daily_stats',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');
-- Add service grouping metadata to existing monitors

-- Group database monitors
UPDATE monitors 
SET metadata = jsonb_build_object(
    'service_group', 'Database Infrastructure',
    'service_category', 'database',
    'priority', 1
)
WHERE display_name LIKE '%Database%';

-- Group API monitors
UPDATE monitors 
SET metadata = jsonb_build_object(
    'service_group', 'API Services',
    'service_category', 'api',
    'priority', 2
)
WHERE display_name LIKE '%API%' AND display_name NOT LIKE '%Database%';
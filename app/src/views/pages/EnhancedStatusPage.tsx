import React from 'react';
import { Head } from '@inertiajs/react';
import { CheckCircle2, AlertCircle, Settings, LayoutGrid, List, Info } from 'lucide-react';
import { Button } from '@/views/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/views/components/ui/tabs';
import { IncidentBanner } from '@/views/components/incident-banner';
import { ServiceGroupCard } from '@/views/components/service-group-card';
import { CompactMonitorCard } from '@/views/components/compact-monitor-card';
import { StatusTracker } from '@/views/components/status-tracker';
import { Card } from '@/views/components/ui/card';

interface StatusDailyStat {
  bucket: string | null;
  monitor_id: number | null;
  check_count: number | null;
  operational_count: number | null;
  incident_count: number | null;
  uptime_percentage: number | null;
  avg_response_time?: number | null;
  p95_response_time?: number | null;
}

interface Monitor {
  id: number;
  name: string;
  display_name: string;
  description?: string;
  metadata?: {
    service_group?: string;
    service_category?: string;
    priority?: number;
  };
}

interface MonitorWithStatus {
  monitor: Monitor;
  current_status: string;
  last_check_time?: string;
  uptime_percentage: number;
  daily_stats: StatusDailyStat[];
}

interface Incident {
  id: number;
  title: string;
  message: string;
  severity: 'info' | 'warning' | 'critical';
  affected_monitors: number[];
  started_at: string;
  resolved_at?: string;
  is_resolved: boolean;
}

interface EnhancedStatusPageData {
  all_operational: boolean;
  last_updated: string;
  monitors: MonitorWithStatus[];
  incidents?: Incident[];
}

interface EnhancedStatusPageProps {
  statusData: EnhancedStatusPageData;
}

// Convert daily stats to tracker data format with missing days filled
const convertDailyStatsToTrackerData = (dailyStats: StatusDailyStat[], days: number = 30) => {
  const dataMap = new Map<string, StatusDailyStat>();
  dailyStats.forEach(stat => {
    // Handle the bucket field which might be null or a timestamp string
    if (stat.bucket) {
      const date = new Date(stat.bucket);
      const dateKey = date.toISOString().split('T')[0];
      dataMap.set(dateKey, stat);
    }
  });
  
  const result = [];
  
  // Use UTC date to match the data from the server
  const today = new Date();
  const todayUTC = new Date(Date.UTC(today.getFullYear(), today.getMonth(), today.getDate()));
  
  for (let i = days - 1; i >= 0; i--) {
    const currentDate = new Date(todayUTC);
    currentDate.setUTCDate(currentDate.getUTCDate() - i);
    const dateKey = currentDate.toISOString().split('T')[0];
    
    // Format for display using local timezone
    const localDate = new Date(currentDate.getUTCFullYear(), currentDate.getUTCMonth(), currentDate.getUTCDate());
    const formattedDate = localDate.toLocaleDateString('en-US', { 
      day: 'numeric', 
      month: 'short', 
      year: 'numeric' 
    });
    
    const stat = dataMap.get(dateKey);
    
    if (stat) {
      let status = 'operational';
      const uptime = stat.uptime_percentage ?? 0;
      const checkCount = stat.check_count ?? 0;
      const operationalCount = stat.operational_count ?? 0;
      
      if (uptime < 50) {
        status = 'major_outage';
      } else if (uptime < 90) {
        status = 'partial_outage';
      } else if (uptime < 98) {
        status = 'degraded';
      } else if (checkCount === 0) {
        status = 'unknown';
      }
      
      const tooltip = `${uptime.toFixed(1)}% uptime`;
      
      result.push({
        date: formattedDate,
        tooltip,
        status,
      });
    } else {
      result.push({
        date: formattedDate,
        tooltip: 'No data available',
        status: 'unknown',
      });
    }
  }
  
  return result;
};

// Group monitors by service
const groupMonitorsByService = (monitors: MonitorWithStatus[]) => {
  const groups = new Map<string, MonitorWithStatus[]>();
  const ungrouped: MonitorWithStatus[] = [];
  
  monitors.forEach(monitor => {
    const serviceGroup = monitor.monitor.metadata?.service_group;
    
    if (serviceGroup) {
      if (!groups.has(serviceGroup)) {
        groups.set(serviceGroup, []);
      }
      groups.get(serviceGroup)!.push(monitor);
    } else {
      ungrouped.push(monitor);
    }
  });
  
  // Convert to array and calculate overall status for each group
  const serviceGroups = Array.from(groups.entries()).map(([name, monitors]) => {
    // Determine overall status (worst status in the group)
    let overallStatus = 'operational';
    for (const monitor of monitors) {
      if (monitor.current_status === 'major_outage') {
        overallStatus = 'major_outage';
        break;
      } else if (monitor.current_status === 'partial_outage' && overallStatus !== 'major_outage') {
        overallStatus = 'partial_outage';
      } else if (monitor.current_status === 'degraded' && overallStatus === 'operational') {
        overallStatus = 'degraded';
      }
    }
    
    return {
      name,
      monitors,
      overallStatus,
    };
  });
  
  // Sort groups by priority or name
  serviceGroups.sort((a, b) => a.name.localeCompare(b.name));
  
  return { serviceGroups, ungrouped };
};

export default function EnhancedStatusPage({ statusData }: EnhancedStatusPageProps) {
  const { all_operational, last_updated, monitors, incidents = [] } = statusData;
  const [viewMode, setViewMode] = React.useState<'grouped' | 'grid'>('grouped');
  const [trackerDays, setTrackerDays] = React.useState(14);
  
  
  // Process monitors with tracker data
  const processedMonitors = monitors.map(monitor => ({
    ...monitor,
    daily_stats: convertDailyStatsToTrackerData(monitor.daily_stats, trackerDays),
  }));
  
  // Group monitors by service (use processedMonitors instead of monitors)
  const { serviceGroups, ungrouped } = groupMonitorsByService(processedMonitors);
  
  // Calculate overall system status
  const operationalCount = monitors.filter(m => m.current_status === 'operational').length;
  const hasActiveIncidents = incidents.some(i => !i.is_resolved);
  
  return (
    <>
      <Head title="System Status" />
      
      <div className="min-h-screen bg-background">
        <div className="mx-auto max-w-7xl px-4 py-8">
          {/* Header */}
          <div className="mb-8">
            <div className="flex items-start justify-between mb-2">
              <div>
                <h1 className="text-3xl font-bold text-foreground mb-2 flex items-center gap-3">
                  {all_operational && !hasActiveIncidents ? (
                    <>
                      <CheckCircle2 className="size-8 text-emerald-500" />
                      All systems operational
                    </>
                  ) : (
                    <>
                      <AlertCircle className="size-8 text-amber-500" />
                      Some systems are experiencing issues
                    </>
                  )}
                </h1>
                <p className="text-muted-foreground">
                  {operationalCount} of {monitors.length} services operational • 
                  Last updated {new Date(last_updated).toLocaleString('en-US', {
                    month: 'short',
                    day: 'numeric',
                    hour: 'numeric',
                    minute: '2-digit',
                    hour12: true,
                    timeZoneName: 'short'
                  })}
                </p>
              </div>
              
              <div className="flex items-center gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setViewMode(viewMode === 'grouped' ? 'grid' : 'grouped')}
                >
                  {viewMode === 'grouped' ? <LayoutGrid className="size-4" /> : <List className="size-4" />}
                  <span className="ml-2">{viewMode === 'grouped' ? 'Grid View' : 'Grouped View'}</span>
                </Button>
              </div>
            </div>
          </div>

          {/* Active Incidents */}
          <IncidentBanner 
            incidents={incidents} 
            monitors={monitors.map(m => m.monitor)}
          />

          {/* View Modes */}
          {viewMode === 'grouped' ? (
            <div className="space-y-4">
              {/* Service Groups */}
              {serviceGroups.map((group) => (
                <ServiceGroupCard
                  key={group.name}
                  group={group}
                  defaultOpen={group.overallStatus !== 'operational'}
                  trackerDays={trackerDays}
                />
              ))}
              
              {/* Ungrouped Monitors */}
              {ungrouped.length > 0 && (
                <ServiceGroupCard
                  group={{
                    name: 'Other Services',
                    monitors: ungrouped,
                    overallStatus: ungrouped.some(m => m.current_status !== 'operational') 
                      ? 'degraded' 
                      : 'operational',
                  }}
                  defaultOpen={true}
                  trackerDays={trackerDays}
                />
              )}
            </div>
          ) : (
            /* Grid View */
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
              {processedMonitors.map((monitor) => (
                <CompactMonitorCard
                  key={monitor.monitor.id}
                  data={monitor}
                  trackerDays={trackerDays}
                  showDescription={true}
                />
              ))}
            </div>
          )}

          {/* Legend and Info */}
          <div className="mt-8 space-y-4">
            <div className="flex flex-wrap gap-4 text-sm">
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-emerald-500 rounded" />
                <span>Operational</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-amber-500 rounded" />
                <span>Degraded</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-orange-500 rounded" />
                <span>Partial Outage</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-red-500 rounded" />
                <span>Major Outage</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-blue-500 rounded" />
                <span>Maintenance</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-gray-400 rounded" />
                <span>No Data</span>
              </div>
            </div>
            
          </div>
        </div>
      </div>
    </>
  );
}
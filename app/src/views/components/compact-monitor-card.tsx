import React from 'react';
import { CheckCircle2, AlertCircle, XCircle, Wrench, HelpCircle, Info } from 'lucide-react';
import { Card } from '@/views/components/ui/card';
import { StatusTracker } from '@/views/components/status-tracker';
import { cn } from '@/views/lib/utils';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/views/components/ui/tooltip";

interface MonitorData {
  monitor: {
    id: number;
    name: string;
    display_name: string;
    description?: string;
  };
  current_status: string;
  last_check_time?: string;
  uptime_percentage: number;
  daily_stats: Array<{
    date: string;
    tooltip: string;
    status: string;
  }>;
}

interface CompactMonitorCardProps {
  data: MonitorData;
  trackerDays?: number;
  showDescription?: boolean;
}

const statusConfig = {
  operational: {
    icon: CheckCircle2,
    color: 'text-emerald-500',
    bgColor: 'bg-emerald-50 dark:bg-emerald-950',
  },
  degraded: {
    icon: AlertCircle,
    color: 'text-amber-500',
    bgColor: 'bg-amber-50 dark:bg-amber-950',
  },
  partial_outage: {
    icon: AlertCircle,
    color: 'text-orange-500',
    bgColor: 'bg-orange-50 dark:bg-orange-950',
  },
  major_outage: {
    icon: XCircle,
    color: 'text-red-500',
    bgColor: 'bg-red-50 dark:bg-red-950',
  },
  maintenance: {
    icon: Wrench,
    color: 'text-blue-500',
    bgColor: 'bg-blue-50 dark:bg-blue-950',
  },
  unknown: {
    icon: HelpCircle,
    color: 'text-gray-400',
    bgColor: 'bg-gray-50 dark:bg-gray-950',
  },
};

export function CompactMonitorCard({ 
  data, 
  trackerDays = 14,
  showDescription = false 
}: CompactMonitorCardProps) {
  const config = statusConfig[data.current_status as keyof typeof statusConfig] || statusConfig.unknown;
  const Icon = config.icon;
  
  // Get last N days of data for compact view
  const trackerData = data.daily_stats.slice(-trackerDays);
  
  return (
    <Card className="p-4 hover:shadow-lg transition-shadow">
      <div className="space-y-3">
        {/* Header */}
        <div className="flex items-start justify-between gap-2">
          <div className="flex items-center gap-2 flex-1 min-w-0">
            <Icon className={cn('size-4 flex-shrink-0', config.color)} />
            <h3 className="font-medium text-sm truncate">
              {data.monitor.display_name}
            </h3>
          </div>
          <span className="text-xs font-medium text-muted-foreground whitespace-nowrap">
            {data.uptime_percentage.toFixed(1)}%
          </span>
        </div>
        
        {/* Description (optional) */}
        {showDescription && data.monitor.description && (
          <p className="text-xs text-muted-foreground line-clamp-2">
            {data.monitor.description}
          </p>
        )}
        
        {/* Mini Status Tracker */}
        <div className="space-y-1">
          <StatusTracker 
            data={trackerData} 
            className="w-full h-6"
          />
          <div className="flex justify-between text-[10px] text-muted-foreground">
            <span>{trackerDays}d ago</span>
            <span>Today</span>
          </div>
        </div>
        
        {/* Last check time */}
        {data.last_check_time && (
          <div className="text-[10px] text-muted-foreground">
            Last checked {new Date(data.last_check_time).toRelativeTime()}
          </div>
        )}
      </div>
    </Card>
  );
}

// Helper function for relative time (you might want to use a library like date-fns)
declare global {
  interface Date {
    toRelativeTime(): string;
  }
}

Date.prototype.toRelativeTime = function() {
  const seconds = Math.floor((new Date().getTime() - this.getTime()) / 1000);
  
  if (seconds < 60) return 'just now';
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
  return `${Math.floor(seconds / 86400)}d ago`;
};
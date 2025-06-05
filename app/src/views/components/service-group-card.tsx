import React from 'react';
import * as Collapsible from '@radix-ui/react-collapsible';
import { ChevronDown, ChevronRight, CheckCircle2, AlertCircle, XCircle } from 'lucide-react';
import { Card } from '@/views/components/ui/card';
import { CompactMonitorCard } from '@/views/components/compact-monitor-card';
import { cn } from '@/views/lib/utils';

interface ServiceGroup {
  name: string;
  description?: string;
  monitors: any[]; // MonitorWithStatus type
  overallStatus: string;
}

interface ServiceGroupCardProps {
  group: ServiceGroup;
  defaultOpen?: boolean;
  trackerDays?: number;
}

const groupStatusConfig = {
  operational: {
    icon: CheckCircle2,
    color: 'text-emerald-500',
    bgColor: 'bg-emerald-50 dark:bg-emerald-950/50',
    borderColor: 'border-emerald-200 dark:border-emerald-800',
  },
  degraded: {
    icon: AlertCircle,
    color: 'text-amber-500',
    bgColor: 'bg-amber-50 dark:bg-amber-950/50',
    borderColor: 'border-amber-200 dark:border-amber-800',
  },
  partial_outage: {
    icon: AlertCircle,
    color: 'text-orange-500',
    bgColor: 'bg-orange-50 dark:bg-orange-950/50',
    borderColor: 'border-orange-200 dark:border-orange-800',
  },
  major_outage: {
    icon: XCircle,
    color: 'text-red-500',
    bgColor: 'bg-red-50 dark:bg-red-950/50',
    borderColor: 'border-red-200 dark:border-red-800',
  },
};

export function ServiceGroupCard({ 
  group, 
  defaultOpen = true,
  trackerDays = 14 
}: ServiceGroupCardProps) {
  const [isOpen, setIsOpen] = React.useState(defaultOpen);
  
  const config = groupStatusConfig[group.overallStatus as keyof typeof groupStatusConfig] || groupStatusConfig.operational;
  const Icon = config.icon;
  
  // Calculate group statistics
  const operationalCount = group.monitors.filter(m => m.current_status === 'operational').length;
  const totalCount = group.monitors.length;
  
  return (
    <Collapsible.Root open={isOpen} onOpenChange={setIsOpen}>
      <Card className={cn(
        'overflow-hidden transition-colors',
        config.bgColor,
        config.borderColor
      )}>
        <Collapsible.Trigger asChild>
          <button className="w-full p-4 flex items-center justify-between hover:bg-black/5 dark:hover:bg-white/5 transition-colors">
            <div className="flex items-center gap-3">
              {isOpen ? (
                <ChevronDown className="size-4 text-muted-foreground" />
              ) : (
                <ChevronRight className="size-4 text-muted-foreground" />
              )}
              
              <Icon className={cn('size-5', config.color)} />
              
              <div className="text-left">
                <h3 className="font-semibold">{group.name}</h3>
                {group.description && (
                  <p className="text-sm text-muted-foreground">{group.description}</p>
                )}
              </div>
            </div>
            
            <div className="flex items-center gap-4">
              <span className="text-sm font-medium text-muted-foreground">
                {operationalCount}/{totalCount} operational
              </span>
            </div>
          </button>
        </Collapsible.Trigger>
        
        <Collapsible.Content>
          <div className="px-4 pb-4">
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
              {group.monitors.map((monitor) => (
                <CompactMonitorCard
                  key={monitor.monitor.id}
                  data={monitor}
                  trackerDays={trackerDays}
                />
              ))}
            </div>
          </div>
        </Collapsible.Content>
      </Card>
    </Collapsible.Root>
  );
}
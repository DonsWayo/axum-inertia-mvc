import React from 'react';
import { AlertCircle, AlertTriangle, Info, X } from 'lucide-react';
import { cn } from '@/views/lib/utils';
import { Button } from '@/views/components/ui/button';
import { Card } from '@/views/components/ui/card';
import * as Collapsible from '@radix-ui/react-collapsible';

export interface Incident {
  id: number;
  title: string;
  message: string;
  severity: 'info' | 'warning' | 'critical';
  affected_monitors: number[];
  started_at: string;
  resolved_at?: string;
  is_resolved: boolean;
}

interface IncidentBannerProps {
  incidents: Incident[];
  monitors?: Array<{ id: number; display_name: string }>;
}

const severityConfig = {
  info: {
    icon: Info,
    bgColor: 'bg-blue-50 dark:bg-blue-950',
    borderColor: 'border-blue-200 dark:border-blue-800',
    textColor: 'text-blue-800 dark:text-blue-200',
    iconColor: 'text-blue-600 dark:text-blue-400',
  },
  warning: {
    icon: AlertTriangle,
    bgColor: 'bg-amber-50 dark:bg-amber-950',
    borderColor: 'border-amber-200 dark:border-amber-800',
    textColor: 'text-amber-800 dark:text-amber-200',
    iconColor: 'text-amber-600 dark:text-amber-400',
  },
  critical: {
    icon: AlertCircle,
    bgColor: 'bg-red-50 dark:bg-red-950',
    borderColor: 'border-red-200 dark:border-red-800',
    textColor: 'text-red-800 dark:text-red-200',
    iconColor: 'text-red-600 dark:text-red-400',
  },
};

export function IncidentBanner({ incidents, monitors = [] }: IncidentBannerProps) {
  const [dismissedIds, setDismissedIds] = React.useState<Set<number>>(new Set());
  
  const activeIncidents = incidents.filter(
    incident => !incident.is_resolved && !dismissedIds.has(incident.id)
  );
  
  if (activeIncidents.length === 0) return null;
  
  const handleDismiss = (id: number) => {
    setDismissedIds(prev => new Set(prev).add(id));
  };
  
  return (
    <div className="space-y-3 mb-6">
      {activeIncidents.map(incident => {
        const config = severityConfig[incident.severity];
        const Icon = config.icon;
        const affectedMonitorNames = incident.affected_monitors
          .map(id => monitors.find(m => m.id === id)?.display_name)
          .filter(Boolean);
        
        return (
          <Collapsible.Root key={incident.id}>
            <Card className={cn(
              'border p-4',
              config.bgColor,
              config.borderColor,
              config.textColor
            )}>
              <div className="flex items-start gap-3">
                <Icon className={cn('size-5 mt-0.5 flex-shrink-0', config.iconColor)} />
                
                <div className="flex-1 space-y-2">
                  <div className="flex items-start justify-between gap-4">
                    <div className="space-y-1">
                      <h3 className="font-semibold">{incident.title}</h3>
                      <p className="text-sm opacity-90">{incident.message}</p>
                    </div>
                    
                    <Button
                      variant="ghost"
                      size="sm"
                      className="p-1 h-auto"
                      onClick={() => handleDismiss(incident.id)}
                    >
                      <X className="size-4" />
                    </Button>
                  </div>
                  
                  {affectedMonitorNames.length > 0 && (
                    <Collapsible.Trigger asChild>
                      <button className="text-sm font-medium hover:underline">
                        Affecting {affectedMonitorNames.length} service{affectedMonitorNames.length !== 1 ? 's' : ''} ▼
                      </button>
                    </Collapsible.Trigger>
                  )}
                  
                  <Collapsible.Content className="mt-2">
                    <div className="text-sm opacity-75">
                      <span className="font-medium">Affected services:</span> {affectedMonitorNames.join(', ')}
                    </div>
                  </Collapsible.Content>
                  
                  <div className="text-xs opacity-75">
                    Started {new Date(incident.started_at).toLocaleString()}
                  </div>
                </div>
              </div>
            </Card>
          </Collapsible.Root>
        );
      })}
    </div>
  );
}
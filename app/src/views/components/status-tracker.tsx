import React from 'react';
import * as HoverCardPrimitives from '@radix-ui/react-hover-card';
import { cn } from "@/views/lib/utils"

interface TrackerDataPoint {
  date: string;
  tooltip: string;
  status: string;
}

const colorMapping: Record<string, string> = {
  operational: 'bg-emerald-500',
  degraded: 'bg-amber-500',
  partial_outage: 'bg-orange-500',
  major_outage: 'bg-red-500',
  maintenance: 'bg-blue-500',
  unknown: 'bg-gray-400',
};

interface BlockProps {
  color: string;
  tooltip: string;
  date: string;
  hoverEffect?: boolean;
}

const Block: React.FC<BlockProps> = ({ color, tooltip, date, hoverEffect = true }) => {
  const [open, setOpen] = React.useState(false);
  
  return (
    <HoverCardPrimitives.Root
      open={open}
      onOpenChange={setOpen}
      openDelay={0}
      closeDelay={0}
    >
      <HoverCardPrimitives.Trigger onClick={() => setOpen(true)} asChild>
        <div className="size-full overflow-hidden px-[0.5px] transition first:rounded-l-[4px] first:pl-0 last:rounded-r-[4px] last:pr-0 sm:px-px">
          <div
            className={cn(
              'size-full rounded-[1px]',
              color,
              hoverEffect && 'hover:opacity-50'
            )}
          />
        </div>
      </HoverCardPrimitives.Trigger>
      <HoverCardPrimitives.Portal>
        <HoverCardPrimitives.Content
          sideOffset={10}
          side="top"
          align="center"
          avoidCollisions
          className={cn(
            'flex min-w-44 max-w-52 space-x-2 rounded-md p-2 shadow-md',
            'text-sm',
            'bg-popover text-popover-foreground',
            'border border-border'
          )}
        >
          <div
            className={cn('w-1 shrink-0 rounded', color)}
            aria-hidden={true}
          />
          <div className="space-y-1">
            <p className="font-medium">
              {tooltip}
            </p>
            <p className="text-xs text-muted-foreground">
              {date}
            </p>
          </div>
        </HoverCardPrimitives.Content>
      </HoverCardPrimitives.Portal>
    </HoverCardPrimitives.Root>
  );
};

interface TrackerProps {
  data: TrackerDataPoint[];
  className?: string;
  hoverEffect?: boolean;
}

export const StatusTracker: React.FC<TrackerProps> = ({ 
  data = [], 
  className, 
  hoverEffect = true 
}) => {
  const combinedData = data.map((item) => ({
    ...item,
    color: colorMapping[item.status] || colorMapping.unknown,
  }));

  return (
    <div
      className={cn('flex h-8 items-center', className)}
    >
      {combinedData.map((props, index) => (
        <Block
          key={index}
          hoverEffect={hoverEffect}
          {...props}
        />
      ))}
    </div>
  );
};
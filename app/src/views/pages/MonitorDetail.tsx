import { router } from "@inertiajs/react"
import MainLayout from "@/views/layouts/Main"
import { ReactNode } from "react"
import { Button } from "@/views/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/views/components/ui/card"
import { Badge } from "@/views/components/ui/badge"

import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/views/components/ui/table"
import { 
  IconArrowLeft, 
  IconEdit, 
  IconActivity, 
  IconClock, 
  IconGlobe,
  IconServer,
  IconWifi,
  IconWorld,
  IconCheck,
  IconX,
  IconAlertTriangle
} from "@tabler/icons-react"

interface Monitor {
  id: number
  name: string
  display_name: string
  description?: string
  url?: string
  monitor_type: string
  check_interval: number
  timeout: number
  is_active: boolean
  created_at: string
  updated_at: string
}

interface MonitorStatusSummary {
  monitor_id: number
  current_status: string
  last_check_time: string
  uptime_24h: number
  uptime_7d: number
  uptime_30d: number
  uptime_90d: number
  avg_response_time_24h?: number
  incident_count_24h: number
}

interface TrackerDataPoint {
  date: string
  tooltip: string
  status: string
}

interface StatusEvent {
  time: string
  monitor_id: number
  status: string
  response_time?: number
  status_code?: number
  error_message?: string
  created_at: string
}

interface MonitorDetailProps {
  monitor: Monitor
  summary: MonitorStatusSummary
  tracker_data: TrackerDataPoint[]
  recent_events: StatusEvent[]
}

const getMonitorTypeIcon = (type: string) => {
  switch (type) {
    case "http":
      return <IconGlobe className="h-5 w-5" />
    case "tcp":
      return <IconServer className="h-5 w-5" />
    case "ping":
      return <IconWifi className="h-5 w-5" />
    case "dns":
      return <IconWorld className="h-5 w-5" />
    default:
      return <IconActivity className="h-5 w-5" />
  }
}

const getStatusBadge = (status: string) => {
  switch (status) {
    case "operational":
      return <Badge className="bg-green-100 text-green-800 hover:bg-green-100">Operational</Badge>
    case "degraded":
      return <Badge className="bg-yellow-100 text-yellow-800 hover:bg-yellow-100">Degraded</Badge>
    case "partial_outage":
      return <Badge className="bg-orange-100 text-orange-800 hover:bg-orange-100">Partial Outage</Badge>
    case "major_outage":
      return <Badge className="bg-red-100 text-red-800 hover:bg-red-100">Major Outage</Badge>
    case "maintenance":
      return <Badge className="bg-blue-100 text-blue-800 hover:bg-blue-100">Maintenance</Badge>
    default:
      return <Badge variant="secondary">Unknown</Badge>
  }
}

const getStatusIcon = (status: string) => {
  switch (status) {
    case "operational":
      return <IconCheck className="h-4 w-4 text-green-600" />
    case "degraded":
    case "partial_outage":
      return <IconAlertTriangle className="h-4 w-4 text-yellow-600" />
    case "major_outage":
      return <IconX className="h-4 w-4 text-red-600" />
    default:
      return <IconActivity className="h-4 w-4 text-gray-600" />
  }
}

const formatUptime = (percentage: number) => {
  return `${percentage.toFixed(2)}%`
}

const formatResponseTime = (ms?: number) => {
  if (!ms) return "N/A"
  return `${ms}ms`
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString()
}

const formatRelativeTime = (dateString: string) => {
  const date = new Date(dateString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  
  if (diffMins < 1) return "Just now"
  if (diffMins < 60) return `${diffMins}m ago`
  if (diffMins < 1440) return `${Math.floor(diffMins / 60)}h ago`
  return `${Math.floor(diffMins / 1440)}d ago`
}

function MonitorDetailPage({ monitor, summary, tracker_data, recent_events }: MonitorDetailProps) {
  const handleBack = () => {
    router.visit("/monitors")
  }

  const handleEdit = () => {
    router.visit(`/monitors/${monitor.id}/edit`)
  }

  return (
    <div className="container mx-auto px-4 py-6 max-w-6xl">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-4">
          <Button
            variant="ghost"
            size="sm"
            onClick={handleBack}
            className="gap-2"
          >
            <IconArrowLeft className="h-4 w-4" />
            Back to Monitors
          </Button>
          <div className="flex items-center gap-3">
            {getMonitorTypeIcon(monitor.monitor_type)}
            <div>
              <h1 className="text-2xl font-bold">{monitor.display_name}</h1>
              <p className="text-muted-foreground">{monitor.url}</p>
            </div>
          </div>
        </div>
        
        <div className="flex items-center gap-3">
          {getStatusBadge(summary.current_status)}
          <Badge variant={monitor.is_active ? "default" : "secondary"}>
            {monitor.is_active ? "Active" : "Inactive"}
          </Badge>
          <Button onClick={handleEdit} className="gap-2">
            <IconEdit className="h-4 w-4" />
            Edit Monitor
          </Button>
        </div>
      </div>

      {/* Monitor Info */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
        <Card>
          <CardHeader>
            <CardTitle>Monitor Details</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div>
              <p className="text-sm font-medium text-muted-foreground">Type</p>
              <p className="capitalize">{monitor.monitor_type}</p>
            </div>
            <div>
              <p className="text-sm font-medium text-muted-foreground">Check Interval</p>
              <p>{monitor.check_interval} seconds</p>
            </div>
            <div>
              <p className="text-sm font-medium text-muted-foreground">Timeout</p>
              <p>{monitor.timeout} seconds</p>
            </div>
            <div>
              <p className="text-sm font-medium text-muted-foreground">Last Check</p>
              <p>{formatRelativeTime(summary.last_check_time)}</p>
            </div>
            {monitor.description && (
              <div>
                <p className="text-sm font-medium text-muted-foreground">Description</p>
                <p className="text-sm">{monitor.description}</p>
              </div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Uptime Statistics</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="flex justify-between">
              <span className="text-sm text-muted-foreground">24 hours</span>
              <span className="font-medium">{formatUptime(summary.uptime_24h)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm text-muted-foreground">7 days</span>
              <span className="font-medium">{formatUptime(summary.uptime_7d)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm text-muted-foreground">30 days</span>
              <span className="font-medium">{formatUptime(summary.uptime_30d)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm text-muted-foreground">90 days</span>
              <span className="font-medium">{formatUptime(summary.uptime_90d)}</span>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Performance</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="flex justify-between">
              <span className="text-sm text-muted-foreground">Avg Response Time (24h)</span>
              <span className="font-medium">{formatResponseTime(summary.avg_response_time_24h)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm text-muted-foreground">Incidents (24h)</span>
              <span className="font-medium">{summary.incident_count_24h}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm text-muted-foreground">Current Status</span>
              <div className="flex items-center gap-2">
                {getStatusIcon(summary.current_status)}
                <span className="font-medium capitalize">{summary.current_status.replace('_', ' ')}</span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Status Tracker */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>90-Day Status History</CardTitle>
          <CardDescription>
            Each square represents one day. Hover for details.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-wrap gap-1">
            {tracker_data.map((point, index) => (
              <div
                key={index}
                className={`w-3 h-3 rounded-sm ${
                  point.status === "operational" ? "bg-green-500" :
                  point.status === "degraded" ? "bg-yellow-500" :
                  point.status === "partial_outage" ? "bg-orange-500" :
                  point.status === "major_outage" ? "bg-red-500" :
                  point.status === "maintenance" ? "bg-blue-500" :
                  "bg-gray-300"
                }`}
                title={`${point.date}: ${point.tooltip}`}
              />
            ))}
          </div>
          <div className="flex items-center justify-between mt-4 text-sm text-muted-foreground">
            <span>90 days ago</span>
            <div className="flex items-center gap-4">
              <div className="flex items-center gap-1">
                <div className="w-3 h-3 bg-green-500 rounded-sm"></div>
                <span>Operational</span>
              </div>
              <div className="flex items-center gap-1">
                <div className="w-3 h-3 bg-yellow-500 rounded-sm"></div>
                <span>Degraded</span>
              </div>
              <div className="flex items-center gap-1">
                <div className="w-3 h-3 bg-red-500 rounded-sm"></div>
                <span>Outage</span>
              </div>
            </div>
            <span>Today</span>
          </div>
        </CardContent>
      </Card>

      {/* Recent Events */}
      <Card>
        <CardHeader>
          <CardTitle>Recent Events</CardTitle>
          <CardDescription>
            Latest {recent_events.length} monitoring events
          </CardDescription>
        </CardHeader>
        <CardContent>
          {recent_events.length === 0 ? (
            <div className="text-center py-8">
              <IconActivity className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
              <h3 className="text-lg font-medium mb-2">No recent events</h3>
              <p className="text-muted-foreground">
                Monitor events will appear here once checks begin
              </p>
            </div>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Time</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead>Response Time</TableHead>
                  <TableHead>Status Code</TableHead>
                  <TableHead>Message</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {recent_events.map((event, index) => (
                  <TableRow key={index}>
                    <TableCell>
                      <div>
                        <p className="font-medium">{formatRelativeTime(event.time)}</p>
                        <p className="text-xs text-muted-foreground">{formatDate(event.time)}</p>
                      </div>
                    </TableCell>
                    <TableCell>
                      <div className="flex items-center gap-2">
                        {getStatusIcon(event.status)}
                        <span className="capitalize">{event.status.replace('_', ' ')}</span>
                      </div>
                    </TableCell>
                    <TableCell>
                      {formatResponseTime(event.response_time)}
                    </TableCell>
                    <TableCell>
                      {event.status_code || "N/A"}
                    </TableCell>
                    <TableCell>
                      <span className="text-sm">{event.error_message || "OK"}</span>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </CardContent>
      </Card>
    </div>
  )
}

MonitorDetailPage.layout = (page: ReactNode) => <MainLayout children={page} />

export default MonitorDetailPage 
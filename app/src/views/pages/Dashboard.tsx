import { ChartAreaInteractive } from "@/views/components/chart-area-interactive"
import { DataTable } from "@/views/components/data-table"
import { SectionCards } from "@/views/components/section-cards"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/views/components/ui/card"
import { Button } from "@/views/components/ui/button"
import { Badge } from "@/views/components/ui/badge"
import { router } from "@inertiajs/react"
import { IconPlus, IconActivity, IconEye } from "@tabler/icons-react"

import data from "@/data/dashboard/data.json"
import MainLayout from "@/views/layouts/Main"
import { ReactNode } from "react"

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

interface MonitorWithStatus {
  monitor: Monitor
  current_status: string
  last_check_time?: string
  uptime_percentage: number
}

interface StatusData {
  all_operational: boolean
  last_updated: string
  monitors: MonitorWithStatus[]
  incidents: any[]
}

interface DashboardProps {
  message: string
  documents: any[]
  statusData?: StatusData
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

const formatUptime = (percentage: number) => {
  return `${percentage.toFixed(1)}%`
}

function Page({ message, statusData }: DashboardProps) {
    const handleViewMonitors = () => {
        router.visit("/monitors")
    }

    const handleCreateMonitor = () => {
        router.visit("/monitors/new")
    }

    const handleViewStatus = () => {
        router.visit("/status")
    }

    return (
        <>
            {/* Welcome Message */}
            <div className="px-4 lg:px-6 mb-6">
                <h1 className="text-2xl font-bold mb-2">Dashboard</h1>
                <p className="text-muted-foreground">{message}</p>
            </div>

            {/* Monitor Status Overview */}
            {statusData && (
                <div className="px-4 lg:px-6 mb-6">
                    <Card>
                        <CardHeader className="flex flex-row items-center justify-between">
                            <div>
                                <CardTitle className="flex items-center gap-2">
                                    <IconActivity className="h-5 w-5" />
                                    System Status
                                </CardTitle>
                                <CardDescription>
                                    Current status of all monitored services
                                </CardDescription>
                            </div>
                            <div className="flex items-center gap-2">
                                <Button variant="outline" size="sm" onClick={handleViewStatus}>
                                    <IconEye className="h-4 w-4 mr-2" />
                                    View Status Page
                                </Button>
                                <Button size="sm" onClick={handleViewMonitors}>
                                    Manage Monitors
                                </Button>
                            </div>
                        </CardHeader>
                        <CardContent>
                            <div className="flex items-center justify-between mb-4">
                                <div className="flex items-center gap-2">
                                    <div className={`h-3 w-3 rounded-full ${statusData.all_operational ? 'bg-green-500' : 'bg-red-500'}`}></div>
                                    <span className="font-medium">
                                        {statusData.all_operational ? 'All Systems Operational' : 'System Issues Detected'}
                                    </span>
                                </div>
                                <div className="text-sm text-muted-foreground">
                                    Last updated: {new Date(statusData.last_updated).toLocaleString()}
                                </div>
                            </div>

                            {statusData.monitors.length === 0 ? (
                                <div className="text-center py-8">
                                    <IconActivity className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
                                    <h3 className="text-lg font-medium mb-2">No monitors configured</h3>
                                    <p className="text-muted-foreground mb-4">
                                        Get started by creating your first monitor to track service availability
                                    </p>
                                    <Button onClick={handleCreateMonitor} className="gap-2">
                                        <IconPlus className="h-4 w-4" />
                                        Create First Monitor
                                    </Button>
                                </div>
                            ) : (
                                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                    {statusData.monitors.slice(0, 6).map(({ monitor, current_status, uptime_percentage }) => (
                                        <div key={monitor.id} className="border rounded-lg p-4">
                                            <div className="flex items-center justify-between mb-2">
                                                <h4 className="font-medium">{monitor.display_name}</h4>
                                                {getStatusBadge(current_status)}
                                            </div>
                                            <p className="text-sm text-muted-foreground mb-2">{monitor.url}</p>
                                            <div className="flex items-center justify-between text-sm">
                                                <span className="text-muted-foreground">90d uptime</span>
                                                <span className={uptime_percentage >= 99 ? "text-green-600" : uptime_percentage >= 95 ? "text-yellow-600" : "text-red-600"}>
                                                    {formatUptime(uptime_percentage)}
                                                </span>
                                            </div>
                                        </div>
                                    ))}
                                </div>
                            )}

                            {statusData.monitors.length > 6 && (
                                <div className="mt-4 text-center">
                                    <Button variant="outline" onClick={handleViewMonitors}>
                                        View All {statusData.monitors.length} Monitors
                                    </Button>
                                </div>
                            )}
                        </CardContent>
                    </Card>
                </div>
            )}

            <SectionCards />
            <div className="px-4 lg:px-6">
                <ChartAreaInteractive />
            </div>
            <DataTable data={data} />
        </>
    )
}


Page.layout = (page: ReactNode) => <MainLayout children={page} />

export default Page
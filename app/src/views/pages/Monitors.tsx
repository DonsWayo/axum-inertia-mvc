
import { router } from "@inertiajs/react"
import MainLayout from "@/views/layouts/Main"
import { ReactNode } from "react"
import { Button } from "@/views/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/views/components/ui/card"
import { 
  IconPlus, 
  IconActivity,
  IconClock
} from "@tabler/icons-react"

import { MonitorDataTable } from "@/views/components/monitor-data-table"

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

interface MonitorsProps {
  monitors: MonitorWithStatus[]
}

// Transform monitor data to match the MonitorDataTable schema
interface MonitorTableData {
  id: number
  display_name: string
  monitor_type: string
  current_status: string
  url?: string
  uptime_percentage: number
  check_interval: number
  is_active: boolean
  last_check_time?: string
  monitor_data: MonitorWithStatus
}

const formatUptime = (percentage: number) => {
  return `${percentage.toFixed(2)}%`
}

function MonitorsPage({ monitors }: MonitorsProps) {
  const handleCreateNew = () => {
    router.visit("/monitors/new")
  }

  // Transform monitor data to match MonitorDataTable schema
  const tableData: MonitorTableData[] = monitors.map(({ monitor, current_status, last_check_time, uptime_percentage }) => ({
    id: monitor.id,
    display_name: monitor.display_name,
    monitor_type: monitor.monitor_type,
    current_status: current_status,
    url: monitor.url,
    uptime_percentage: uptime_percentage,
    check_interval: monitor.check_interval,
    is_active: monitor.is_active,
    last_check_time: last_check_time,
    monitor_data: { monitor, current_status, last_check_time, uptime_percentage }
  }))

  const operationalCount = monitors.filter(m => m.current_status === "operational").length
  const totalCount = monitors.length
  const averageUptime = monitors.length > 0 
    ? monitors.reduce((sum, m) => sum + m.uptime_percentage, 0) / monitors.length 
    : 0

  return (
    <div className="container mx-auto px-4 py-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold">Monitors</h1>
          <p className="text-muted-foreground">
            Manage and configure your service monitors
          </p>
        </div>
        <Button onClick={handleCreateNew} className="gap-2">
          <IconPlus className="h-4 w-4" />
          Add Monitor
        </Button>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">Total Monitors</p>
                <p className="text-2xl font-bold">{totalCount}</p>
              </div>
              <IconActivity className="h-8 w-8 text-muted-foreground" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">Operational</p>
                <p className="text-2xl font-bold text-green-600">{operationalCount}</p>
              </div>
              <div className="h-8 w-8 rounded-full bg-green-100 flex items-center justify-center">
                <div className="h-3 w-3 rounded-full bg-green-600"></div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">Average Uptime</p>
                <p className="text-2xl font-bold">{formatUptime(averageUptime)}</p>
              </div>
              <IconClock className="h-8 w-8 text-muted-foreground" />
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Powerful DataTable */}
      {tableData.length === 0 ? (
        <Card>
          <CardContent className="p-8">
            <div className="text-center">
              <IconActivity className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
              <h3 className="text-lg font-medium mb-2">No monitors configured</h3>
              <p className="text-muted-foreground mb-4">
                Get started by creating your first monitor to track service availability
              </p>
              <Button onClick={handleCreateNew} className="gap-2">
                <IconPlus className="h-4 w-4" />
                Create First Monitor
              </Button>
            </div>
          </CardContent>
        </Card>
             ) : (
         <MonitorDataTable data={tableData} />
       )}
    </div>
  )
}

MonitorsPage.layout = (page: ReactNode) => <MainLayout children={page} />

export default MonitorsPage 
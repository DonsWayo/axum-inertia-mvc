import * as React from "react"
import {
  DndContext,
  KeyboardSensor,
  MouseSensor,
  TouchSensor,
  closestCenter,
  useSensor,
  useSensors,
  type DragEndEvent,
  type UniqueIdentifier,
} from "@dnd-kit/core"
import { restrictToVerticalAxis } from "@dnd-kit/modifiers"
import {
  SortableContext,
  arrayMove,
  useSortable,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable"
import { CSS } from "@dnd-kit/utilities"
import {
  IconChevronDown,
  IconChevronLeft,
  IconChevronRight,
  IconChevronsLeft,
  IconChevronsRight,
  IconCircleCheckFilled,
  IconDotsVertical,
  IconGripVertical,
  IconLayoutColumns,
  IconLoader,
  IconEdit,
  IconTrash,
  IconEye,
  IconGlobe,
  IconServer,
  IconWifi,
  IconWorld,
  IconActivity,
  IconAlertTriangle,
  IconX,
} from "@tabler/icons-react"
import {
  ColumnDef,
  ColumnFiltersState,
  Row,
  SortingState,
  VisibilityState,
  flexRender,
  getCoreRowModel,
  getFacetedRowModel,
  getFacetedUniqueValues,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { toast } from "sonner"
import { z } from "zod"
import { router } from "@inertiajs/react"


import { Badge } from "@/views/components/ui/badge"
import { Button } from "@/views/components/ui/button"
import { Checkbox } from "@/views/components/ui/checkbox"

import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/views/components/ui/dropdown-menu"
import { Input } from "@/views/components/ui/input"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/views/components/ui/select"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/views/components/ui/table"

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

export const monitorSchema = z.object({
  id: z.number(),
  display_name: z.string(),
  monitor_type: z.string(),
  current_status: z.string(),
  url: z.string().optional(),
  uptime_percentage: z.number(),
  check_interval: z.number(),
  is_active: z.boolean(),
  last_check_time: z.string().optional(),
  monitor_data: z.any(),
})

type MonitorData = z.infer<typeof monitorSchema>

const getMonitorTypeIcon = (type: string) => {
  switch (type) {
    case "http":
      return <IconGlobe className="h-4 w-4" />
    case "tcp":
      return <IconServer className="h-4 w-4" />
    case "ping":
      return <IconWifi className="h-4 w-4" />
    case "dns":
      return <IconWorld className="h-4 w-4" />
    default:
      return <IconActivity className="h-4 w-4" />
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
      return <IconCircleCheckFilled className="h-4 w-4 text-green-600" />
    case "degraded":
    case "partial_outage":
      return <IconAlertTriangle className="h-4 w-4 text-yellow-600" />
    case "major_outage":
      return <IconX className="h-4 w-4 text-red-600" />
    default:
      return <IconLoader className="h-4 w-4 text-gray-600 animate-spin" />
  }
}

const formatUptime = (percentage: number) => {
  return `${percentage.toFixed(1)}%`
}

const formatLastCheck = (timestamp?: string) => {
  if (!timestamp) return "Never"
  
  const date = new Date(timestamp)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  
  if (diffMins < 1) return "Just now"
  if (diffMins < 60) return `${diffMins}m ago`
  if (diffMins < 1440) return `${Math.floor(diffMins / 60)}h ago`
  return `${Math.floor(diffMins / 1440)}d ago`
}

// Create a separate component for the drag handle
function DragHandle({ id }: { id: number }) {
  const { attributes, listeners } = useSortable({
    id,
  })

  return (
    <Button
      {...attributes}
      {...listeners}
      variant="ghost"
      size="icon"
      className="text-muted-foreground size-7 hover:bg-transparent"
    >
      <IconGripVertical className="text-muted-foreground size-3" />
      <span className="sr-only">Drag to reorder</span>
    </Button>
  )
}

const columns: ColumnDef<MonitorData>[] = [
  {
    id: "drag",
    header: () => null,
    cell: ({ row }) => <DragHandle id={row.original.id} />,
    enableSorting: false,
    enableHiding: false,
  },
  {
    id: "select",
    header: ({ table }) => (
      <div className="flex items-center justify-center">
        <Checkbox
          checked={
            table.getIsAllPageRowsSelected() ||
            (table.getIsSomePageRowsSelected() && "indeterminate")
          }
          onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
          aria-label="Select all"
        />
      </div>
    ),
    cell: ({ row }) => (
      <div className="flex items-center justify-center">
        <Checkbox
          checked={row.getIsSelected()}
          onCheckedChange={(value) => row.toggleSelected(!!value)}
          aria-label="Select row"
        />
      </div>
    ),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: "display_name",
    header: "Monitor",
    cell: ({ row }) => {
      const monitor = row.original
      return (
        <div className="min-w-0">
          <div className="flex items-center gap-2">
            <p className="font-medium truncate">{monitor.display_name}</p>
            {!monitor.is_active && (
              <Badge variant="secondary" className="text-xs">Inactive</Badge>
            )}
          </div>
          <p className="text-sm text-muted-foreground truncate">{monitor.url || "N/A"}</p>
        </div>
      )
    },
    enableHiding: false,
  },
  {
    accessorKey: "monitor_type",
    header: "Type",
    cell: ({ row }) => (
      <div className="flex items-center gap-2">
        {getMonitorTypeIcon(row.original.monitor_type)}
        <span className="capitalize">{row.original.monitor_type}</span>
      </div>
    ),
  },
  {
    accessorKey: "current_status",
    header: "Status",
    cell: ({ row }) => (
      <div className="flex items-center gap-2">
        {getStatusIcon(row.original.current_status)}
        {getStatusBadge(row.original.current_status)}
      </div>
    ),
  },
  {
    accessorKey: "uptime_percentage",
    header: "Uptime (90d)",
    cell: ({ row }) => {
      const uptime = row.original.uptime_percentage
      return (
        <span className={uptime >= 99 ? "text-green-600 font-medium" : uptime >= 95 ? "text-yellow-600 font-medium" : "text-red-600 font-medium"}>
          {formatUptime(uptime)}
        </span>
      )
    },
  },
  {
    accessorKey: "last_check_time",
    header: "Last Check",
    cell: ({ row }) => (
      <span className="text-sm">{formatLastCheck(row.original.last_check_time)}</span>
    ),
  },
  {
    accessorKey: "check_interval",
    header: "Interval",
    cell: ({ row }) => (
      <span className="text-sm">{row.original.check_interval}s</span>
    ),
  },
  {
    id: "actions",
    header: () => null,
    cell: ({ row }) => {
      const monitor = row.original.monitor_data.monitor
      
      const handleView = () => {
        router.visit(`/monitors/${monitor.id}`)
      }
      
      const handleEdit = () => {
        router.visit(`/monitors/${monitor.id}/edit`)
      }
      
      const handleDelete = async () => {
        if (!confirm(`Are you sure you want to delete "${monitor.display_name}"? This action cannot be undone.`)) {
          return
        }
        
        try {
          router.delete(`/api/monitors/${monitor.id}`, {
            onSuccess: () => {
              toast.success("Monitor deleted successfully!")
            },
            onError: (errors) => {
              toast.error("Failed to delete monitor")
              console.error(errors)
            },
          })
        } catch (error) {
          toast.error("An unexpected error occurred")
          console.error(error)
        }
      }
      
      return (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="sm">
              <IconDotsVertical className="h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem onClick={handleView}>
              <IconEye className="h-4 w-4 mr-2" />
              View Details
            </DropdownMenuItem>
            <DropdownMenuItem onClick={handleEdit}>
              <IconEdit className="h-4 w-4 mr-2" />
              Edit
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem 
              onClick={handleDelete}
              className="text-destructive"
            >
              <IconTrash className="h-4 w-4 mr-2" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )
    },
    enableSorting: false,
    enableHiding: false,
  },
]

function DraggableRow({ row }: { row: Row<MonitorData> }) {
  const {
    transform,
    transition,
    setNodeRef,
    isDragging,
  } = useSortable({
    id: row.original.id,
  })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  }

  return (
    <TableRow
      ref={setNodeRef}
      style={style}
      data-state={row.getIsSelected() && "selected"}
      className={isDragging ? "opacity-50" : ""}
    >
      {row.getVisibleCells().map((cell) => (
        <TableCell key={cell.id}>
          {flexRender(cell.column.columnDef.cell, cell.getContext())}
        </TableCell>
      ))}
    </TableRow>
  )
}

export function MonitorDataTable({
  data: initialData,
}: {
  data: MonitorData[]
}) {
  const [data, setData] = React.useState(initialData)
  const [sorting, setSorting] = React.useState<SortingState>([])
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>([])
  const [columnVisibility, setColumnVisibility] = React.useState<VisibilityState>({})
  const [rowSelection, setRowSelection] = React.useState({})

  const table = useReactTable({
    data,
    columns,
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: setRowSelection,
    getFacetedRowModel: getFacetedRowModel(),
    getFacetedUniqueValues: getFacetedUniqueValues(),
    state: {
      sorting,
      columnFilters,
      columnVisibility,
      rowSelection,
    },
  })

  const sensors = useSensors(
    useSensor(MouseSensor, {}),
    useSensor(TouchSensor, {}),
    useSensor(KeyboardSensor, {})
  )

  function handleDragEnd(event: DragEndEvent) {
    const { active, over } = event

    if (active && over && active.id !== over.id) {
      setData((data) => {
        const oldIndex = data.findIndex((item) => item.id === active.id)
        const newIndex = data.findIndex((item) => item.id === over.id)

        return arrayMove(data, oldIndex, newIndex)
      })
    }
  }

  const dataIds = React.useMemo<UniqueIdentifier[]>(
    () => data?.map(({ id }) => id),
    [data]
  )

  return (
    <div className="space-y-4">
      {/* Filters and Controls */}
      <div className="flex items-center justify-between">
        <div className="flex flex-1 items-center space-x-2">
          <Input
            placeholder="Filter monitors..."
            value={(table.getColumn("display_name")?.getFilterValue() as string) ?? ""}
            onChange={(event) =>
              table.getColumn("display_name")?.setFilterValue(event.target.value)
            }
            className="h-8 w-[150px] lg:w-[250px]"
          />
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" size="sm" className="ml-auto h-8">
                <IconLayoutColumns className="mr-2 h-4 w-4" />
                View
                <IconChevronDown className="ml-2 h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-[150px]">
              {table
                .getAllColumns()
                .filter(
                  (column) =>
                    typeof column.accessorFn !== "undefined" && column.getCanHide()
                )
                .map((column) => {
                  return (
                    <DropdownMenuCheckboxItem
                      key={column.id}
                      className="capitalize"
                      checked={column.getIsVisible()}
                      onCheckedChange={(value) =>
                        column.toggleVisibility(!!value)
                      }
                    >
                      {column.id}
                    </DropdownMenuCheckboxItem>
                  )
                })}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>

      {/* Table */}
      <div className="rounded-md border">
        <DndContext
          collisionDetection={closestCenter}
          modifiers={[restrictToVerticalAxis]}
          onDragEnd={handleDragEnd}
          sensors={sensors}
        >
          <Table>
            <TableHeader>
              {table.getHeaderGroups().map((headerGroup) => (
                <TableRow key={headerGroup.id}>
                  {headerGroup.headers.map((header) => {
                    return (
                      <TableHead key={header.id}>
                        {header.isPlaceholder
                          ? null
                          : flexRender(
                              header.column.columnDef.header,
                              header.getContext()
                            )}
                      </TableHead>
                    )
                  })}
                </TableRow>
              ))}
            </TableHeader>
            <TableBody>
              <SortableContext
                items={dataIds}
                strategy={verticalListSortingStrategy}
              >
                {table.getRowModel().rows?.length ? (
                  table.getRowModel().rows.map((row) => (
                    <DraggableRow key={row.id} row={row} />
                  ))
                ) : (
                  <TableRow>
                    <TableCell
                      colSpan={columns.length}
                      className="h-24 text-center"
                    >
                      No results.
                    </TableCell>
                  </TableRow>
                )}
              </SortableContext>
            </TableBody>
          </Table>
        </DndContext>
      </div>

      {/* Pagination */}
      <div className="flex items-center justify-between px-2">
        <div className="flex-1 text-sm text-muted-foreground">
          {table.getFilteredSelectedRowModel().rows.length} of{" "}
          {table.getFilteredRowModel().rows.length} row(s) selected.
        </div>
        <div className="flex items-center space-x-6 lg:space-x-8">
          <div className="flex items-center space-x-2">
            <p className="text-sm font-medium">Rows per page</p>
            <Select
              value={`${table.getState().pagination.pageSize}`}
              onValueChange={(value) => {
                table.setPageSize(Number(value))
              }}
            >
              <SelectTrigger className="h-8 w-[70px]">
                <SelectValue placeholder={table.getState().pagination.pageSize} />
              </SelectTrigger>
              <SelectContent side="top">
                {[10, 20, 30, 40, 50].map((pageSize) => (
                  <SelectItem key={pageSize} value={`${pageSize}`}>
                    {pageSize}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="flex w-[100px] items-center justify-center text-sm font-medium">
            Page {table.getState().pagination.pageIndex + 1} of{" "}
            {table.getPageCount()}
          </div>
          <div className="flex items-center space-x-2">
            <Button
              variant="outline"
              className="hidden h-8 w-8 p-0 lg:flex"
              onClick={() => table.setPageIndex(0)}
              disabled={!table.getCanPreviousPage()}
            >
              <span className="sr-only">Go to first page</span>
              <IconChevronsLeft className="h-4 w-4" />
            </Button>
            <Button
              variant="outline"
              className="h-8 w-8 p-0"
              onClick={() => table.previousPage()}
              disabled={!table.getCanPreviousPage()}
            >
              <span className="sr-only">Go to previous page</span>
              <IconChevronLeft className="h-4 w-4" />
            </Button>
            <Button
              variant="outline"
              className="h-8 w-8 p-0"
              onClick={() => table.nextPage()}
              disabled={!table.getCanNextPage()}
            >
              <span className="sr-only">Go to next page</span>
              <IconChevronRight className="h-4 w-4" />
            </Button>
            <Button
              variant="outline"
              className="hidden h-8 w-8 p-0 lg:flex"
              onClick={() => table.setPageIndex(table.getPageCount() - 1)}
              disabled={!table.getCanNextPage()}
            >
              <span className="sr-only">Go to last page</span>
              <IconChevronsRight className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </div>
    </div>
  )
} 
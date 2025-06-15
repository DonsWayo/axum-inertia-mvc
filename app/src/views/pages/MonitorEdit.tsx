import { useState } from "react"
import { useForm } from "react-hook-form"
import { router } from "@inertiajs/react"
import MainLayout from "@/views/layouts/Main"
import { ReactNode } from "react"
import { Button } from "@/views/components/ui/button"
import { Input } from "@/views/components/ui/input"
import { Label } from "@/views/components/ui/label"
import { Textarea } from "@/views/components/ui/textarea"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/views/components/ui/select"
import { Switch } from "@/views/components/ui/switch"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/views/components/ui/card"

import { Badge } from "@/views/components/ui/badge"
import { IconArrowLeft, IconCheck, IconX, IconLoader2 } from "@tabler/icons-react"
import { toast } from "sonner"

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
  metadata?: any
  created_at: string
  updated_at: string
}

interface MonitorEditProps {
  monitor?: Monitor
  isNew?: boolean
}

interface MonitorFormData {
  name: string
  display_name: string
  description: string
  url: string
  monitor_type: string
  check_interval: number
  timeout: number
  is_active: boolean
}

const monitorTypes = [
  { value: "http", label: "HTTP/HTTPS" },
  { value: "tcp", label: "TCP Port" },
  { value: "ping", label: "Ping" },
  { value: "dns", label: "DNS" },
  { value: "custom", label: "Custom" },
]

function MonitorEditPage({ monitor, isNew = false }: MonitorEditProps) {
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [selectedType, setSelectedType] = useState(monitor?.monitor_type || "http")

  const {
    register,
    handleSubmit,
    setValue,
    watch,
    formState: { errors, isDirty },
  } = useForm<MonitorFormData>({
    defaultValues: {
      name: monitor?.name || "",
      display_name: monitor?.display_name || "",
      description: monitor?.description || "",
      url: monitor?.url || "",
      monitor_type: monitor?.monitor_type || "http",
      check_interval: monitor?.check_interval || 300,
      timeout: monitor?.timeout || 30,
      is_active: monitor?.is_active ?? true,
    },
  })

  const watchedIsActive = watch("is_active")

  const onSubmit = async (data: MonitorFormData) => {
    setIsSubmitting(true)
    
    try {
      // Convert form data to a plain object that Inertia can handle
      const payload = {
        name: data.name,
        display_name: data.display_name,
        description: data.description,
        url: data.url,
        monitor_type: data.monitor_type,
        check_interval: data.check_interval,
        timeout: data.timeout,
        is_active: data.is_active,
      }

      if (isNew) {
        router.post("/api/monitors", payload, {
          onSuccess: () => {
            toast.success("Monitor created successfully!")
            router.visit("/monitors")
          },
          onError: (errors) => {
            toast.error("Failed to create monitor")
            console.error(errors)
          },
        })
      } else {
        router.put(`/api/monitors/${monitor?.id}`, payload, {
          onSuccess: () => {
            toast.success("Monitor updated successfully!")
            router.visit("/monitors")
          },
          onError: (errors) => {
            toast.error("Failed to update monitor")
            console.error(errors)
          },
        })
      }
    } catch (error) {
      toast.error("An unexpected error occurred")
      console.error(error)
    } finally {
      setIsSubmitting(false)
    }
  }

  const handleCancel = () => {
    if (isDirty) {
      if (confirm("You have unsaved changes. Are you sure you want to leave?")) {
        router.visit("/monitors")
      }
    } else {
      router.visit("/monitors")
    }
  }

  const handleDelete = async () => {
    if (!monitor || isNew) return
    
    if (confirm("Are you sure you want to delete this monitor? This action cannot be undone.")) {
      setIsSubmitting(true)
      
      try {
        await router.delete(`/api/monitors/${monitor.id}`, {
          onSuccess: () => {
            toast.success("Monitor deleted successfully!")
            router.visit("/monitors")
          },
          onError: (errors) => {
            toast.error("Failed to delete monitor")
            console.error(errors)
          },
        })
      } catch (error) {
        toast.error("An unexpected error occurred")
        console.error(error)
      } finally {
        setIsSubmitting(false)
      }
    }
  }

  return (
    <div className="container mx-auto px-4 py-6 max-w-4xl">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-4">
          <Button
            variant="ghost"
            size="sm"
            onClick={handleCancel}
            className="gap-2"
          >
            <IconArrowLeft className="h-4 w-4" />
            Back to Monitors
          </Button>
          <div>
            <h1 className="text-2xl font-bold">
              {isNew ? "Create Monitor" : "Edit Monitor"}
            </h1>
            <p className="text-muted-foreground">
              {isNew 
                ? "Configure a new monitor to track service availability" 
                : `Configure monitoring settings for ${monitor?.display_name}`
              }
            </p>
          </div>
        </div>
        
        {!isNew && monitor && (
          <div className="flex items-center gap-2">
            <Badge variant={monitor.is_active ? "default" : "secondary"}>
              {monitor.is_active ? "Active" : "Inactive"}
            </Badge>
            <Badge variant="outline">
              {monitorTypes.find(t => t.value === monitor.monitor_type)?.label || monitor.monitor_type}
            </Badge>
          </div>
        )}
      </div>

      <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
        {/* Basic Information */}
        <Card>
          <CardHeader>
            <CardTitle>Basic Information</CardTitle>
            <CardDescription>
              Configure the basic details for your monitor
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="name">Monitor Name *</Label>
                <Input
                  id="name"
                  {...register("name", { 
                    required: "Monitor name is required",
                    pattern: {
                      value: /^[a-zA-Z0-9_-]+$/,
                      message: "Only letters, numbers, underscores, and hyphens allowed"
                    }
                  })}
                  placeholder="e.g., api-server"
                />
                {errors.name && (
                  <p className="text-sm text-destructive">{errors.name.message}</p>
                )}
              </div>

              <div className="space-y-2">
                <Label htmlFor="display_name">Display Name *</Label>
                <Input
                  id="display_name"
                  {...register("display_name", { required: "Display name is required" })}
                  placeholder="e.g., API Server"
                />
                {errors.display_name && (
                  <p className="text-sm text-destructive">{errors.display_name.message}</p>
                )}
              </div>
            </div>

            <div className="space-y-2">
              <Label htmlFor="description">Description</Label>
              <Textarea
                id="description"
                {...register("description")}
                placeholder="Optional description of what this monitor checks"
                rows={3}
              />
            </div>

            <div className="flex items-center space-x-2">
              <Switch
                id="is_active"
                checked={watchedIsActive}
                onCheckedChange={(checked) => setValue("is_active", checked)}
              />
              <Label htmlFor="is_active">
                Monitor is active
              </Label>
            </div>
          </CardContent>
        </Card>

        {/* Monitor Configuration */}
        <Card>
          <CardHeader>
            <CardTitle>Monitor Configuration</CardTitle>
            <CardDescription>
              Configure how and what to monitor
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="monitor_type">Monitor Type *</Label>
                <Select
                  value={selectedType}
                  onValueChange={(value) => {
                    setSelectedType(value)
                    setValue("monitor_type", value)
                  }}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select monitor type" />
                  </SelectTrigger>
                  <SelectContent>
                    {monitorTypes.map((type) => (
                      <SelectItem key={type.value} value={type.value}>
                        {type.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label htmlFor="url">
                  {selectedType === "http" ? "URL *" : 
                   selectedType === "tcp" ? "Host:Port *" :
                   selectedType === "ping" ? "Host/IP *" :
                   selectedType === "dns" ? "Domain *" : "Target *"}
                </Label>
                <Input
                  id="url"
                  {...register("url", { 
                    required: "Target URL/Host is required" 
                  })}
                  placeholder={
                    selectedType === "http" ? "https://example.com" :
                    selectedType === "tcp" ? "example.com:80" :
                    selectedType === "ping" ? "example.com" :
                    selectedType === "dns" ? "example.com" : "Target to monitor"
                  }
                />
                {errors.url && (
                  <p className="text-sm text-destructive">{errors.url.message}</p>
                )}
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="check_interval">Check Interval (seconds) *</Label>
                <Input
                  id="check_interval"
                  type="number"
                  min="60"
                  max="3600"
                  {...register("check_interval", { 
                    required: "Check interval is required",
                    min: { value: 60, message: "Minimum interval is 60 seconds" },
                    max: { value: 3600, message: "Maximum interval is 3600 seconds" }
                  })}
                />
                {errors.check_interval && (
                  <p className="text-sm text-destructive">{errors.check_interval.message}</p>
                )}
                <p className="text-xs text-muted-foreground">
                  How often to check the monitor (60-3600 seconds)
                </p>
              </div>

              <div className="space-y-2">
                <Label htmlFor="timeout">Timeout (seconds) *</Label>
                <Input
                  id="timeout"
                  type="number"
                  min="5"
                  max="300"
                  {...register("timeout", { 
                    required: "Timeout is required",
                    min: { value: 5, message: "Minimum timeout is 5 seconds" },
                    max: { value: 300, message: "Maximum timeout is 300 seconds" }
                  })}
                />
                {errors.timeout && (
                  <p className="text-sm text-destructive">{errors.timeout.message}</p>
                )}
                <p className="text-xs text-muted-foreground">
                  How long to wait for a response (5-300 seconds)
                </p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Actions */}
        <div className="flex items-center justify-between pt-6">
          <div>
            {!isNew && monitor && (
              <Button
                type="button"
                variant="destructive"
                onClick={handleDelete}
                disabled={isSubmitting}
                className="gap-2"
              >
                <IconX className="h-4 w-4" />
                Delete Monitor
              </Button>
            )}
          </div>
          
          <div className="flex items-center gap-3">
            <Button
              type="button"
              variant="outline"
              onClick={handleCancel}
              disabled={isSubmitting}
            >
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={isSubmitting || !isDirty}
              className="gap-2"
            >
              {isSubmitting ? (
                <IconLoader2 className="h-4 w-4 animate-spin" />
              ) : (
                <IconCheck className="h-4 w-4" />
              )}
              {isNew ? "Create Monitor" : "Save Changes"}
            </Button>
          </div>
        </div>
      </form>
    </div>
  )
}

MonitorEditPage.layout = (page: ReactNode) => <MainLayout children={page} />

export default MonitorEditPage 
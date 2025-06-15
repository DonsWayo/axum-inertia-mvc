import * as React from "react"
import {
  IconChartBar,
  IconDashboard,
  IconInnerShadowTop,
  IconListDetails,
} from "@tabler/icons-react"

import { NavMain } from "@/views/components/nav-main"
import { NavUser } from "@/views/components/nav-user"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/views/components/ui/sidebar"

const data = {
  user: {
    name: "RustGenie User",
    email: "user@rustgenie.dev",
    avatar: "/avatars/user.jpg",
  },
  navMain: [
    {
      title: "Dashboard",
      url: "/",
      icon: IconDashboard,
    },
    {
      title: "Monitors",
      url: "/monitors",
      icon: IconListDetails,
    },
    {
      title: "Status Page",
      url: "/status",
      icon: IconChartBar,
    },
  ],
  navClouds: [],
  navSecondary: [],
  documents: [],
}

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar collapsible="offcanvas" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              asChild
              className="data-[slot=sidebar-menu-button]:!p-1.5"
            >
              <a href="/">
                <IconInnerShadowTop className="!size-5" />
                <span className="text-base font-semibold">RustGenie</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavMain items={data.navMain} />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={data.user} />
      </SidebarFooter>
    </Sidebar>
  )
}

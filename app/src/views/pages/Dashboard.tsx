import { ChartAreaInteractive } from "@/views/components/chart-area-interactive"
import { DataTable } from "@/views/components/data-table"
import { SectionCards } from "@/views/components/section-cards"

import data from "@/data/dashboard/data.json"
import MainLayout from "@/views/layouts/Main"
import { ReactNode } from "react"

function Page() {
    return (
        <>
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
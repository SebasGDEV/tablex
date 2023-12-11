"use client"
import { Button } from "@/components/ui/button"
import { Command, CommandGroup, CommandItem } from "@/components/ui/command"
import {
  Popover,
  PopoverContent,
  PopoverTrigger
} from "@/components/ui/popover"
import { SupportedDrivers } from "@/lib/types"
import { cn } from "@/lib/utils"
import { Check, ChevronsUpDown } from "lucide-react"
import Image from "next/image"
import { useState } from "react"
import ConnectionRadio from "./_components/conn-radio"
import SqliteConnection from "./_components/sqlite-connection"

const drivers = [
  {
    value: "mysql",
    label: "MySQL"
  },
  {
    value: "psql",
    label: "PostgreSQL"
  },
  {
    value: "sqlite",
    label: "SQLite"
  }
]

const ConnectionPage = () => {
  const [isPopoverOpen, setIsPopoverOpen] = useState(false)
  const [selectedDriver, setSelectedDriver] = useState<string | null>(null)
  return (
    <main className="relative h-full flex flex-col items-center gap-y-9 justify-center">
      <Popover open={isPopoverOpen} onOpenChange={setIsPopoverOpen}>
        <PopoverTrigger asChild>
          <Button
            role="combobox"
            aria-expanded={isPopoverOpen}
            className="w-[230px] justify-between"
          >
            {selectedDriver
              ? drivers.find((driver) => driver.value === selectedDriver)?.label
              : "Select a Database Driver"}
            <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-[200px] p-0">
          <Command>
            <CommandGroup>
              {drivers.map((driver) => (
                <CommandItem
                  key={driver.value}
                  value={driver.value}
                  onSelect={(currentValue) => {
                    setSelectedDriver(
                      currentValue === selectedDriver ? null : currentValue
                    )
                    setIsPopoverOpen(false)
                  }}
                >
                  <Check
                    className={cn(
                      "mr-2 h-4 w-4",
                      driver.value === selectedDriver
                        ? "opacity-100"
                        : "opacity-0"
                    )}
                  />
                  {driver.label}
                </CommandItem>
              ))}
            </CommandGroup>
          </Command>
        </PopoverContent>
      </Popover>
      {selectedDriver === SupportedDrivers.SQLITE && <SqliteConnection />}
      {(selectedDriver === SupportedDrivers.PSQL ||
        selectedDriver === SupportedDrivers.MYSQL) && (
        <ConnectionRadio driver={selectedDriver} />
      )}
      <Image
        src={"/bg-2.svg"}
        fill
        alt="bg"
        aria-hidden
        className="object-cover -z-10 opacity-20 object-center"
      />
    </main>
  )
}

export default ConnectionPage

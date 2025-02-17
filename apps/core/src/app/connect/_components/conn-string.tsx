"use client"
import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { Drivers, DriversValues } from "@/lib/types"
import { customToast } from "@/lib/utils"
import { zodResolver } from "@hookform/resolvers/zod"
import { useRouter } from "next/navigation"
import { useForm } from "react-hook-form"
import { z } from "zod"
import { createConnectionRecord, testConnection } from "../actions"

const formSchema = z.object({
  connName: z.string(),
  connString: z.string()
})

interface ConnectionParamsFormProps {
  driver: Exclude<DriversValues, typeof Drivers.SQLite>
}

const ConnectionStringForm = ({ driver }: ConnectionParamsFormProps) => {
  const router = useRouter()
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema)
  })

  const onClickConnect = async (values: z.infer<typeof formSchema>) => {
    const action = createConnectionRecord(
      values.connName,
      values.connString,
      driver
    )
    customToast(
      action,
      {
        success: () => {
          router.push("/connections")
          return "Successfully created connection"
        },
        error: (e) => e
      },
      "create_connection"
    )
  }

  const onClickTest = async (values: z.infer<typeof formSchema>) => {
    await testConnection(values.connString, driver)
  }

  return (
    <Form {...form}>
      <form className="space-y-10">
        <FormField
          control={form.control}
          name="connName"
          render={({ field }) => (
            <FormItem className="col-span-full">
              <FormLabel>Connection Name</FormLabel>
              <FormControl>
                <Input placeholder="e.g awesome project dev" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="connString"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Connection String</FormLabel>
              <FormControl>
                <Input
                  placeholder={`${driver}://user:password@host:port/dbName?sslmode=`}
                  className="w-[300px] lg:w-[400px]"
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <div className="col-span-2 flex items-center justify-center gap-x-4">
          <Button
            variant={"secondary"}
            onClick={form.handleSubmit(onClickConnect)}
          >
            Connect
          </Button>
          <Button
            className="bg-green-500 hover:bg-green-700"
            onClick={form.handleSubmit(onClickTest)}
          >
            Test
          </Button>
        </div>
      </form>
    </Form>
  )
}

export default ConnectionStringForm

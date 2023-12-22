import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import { Drivers } from "./types"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

type ConnectionStringParams =
  | {
      driver: typeof Drivers.SQLite
      filePath: string
    }
  | {
      driver: typeof Drivers.PostgreSQL | typeof Drivers.MySQL
      username: string
      password: string
      host: string
      port: number
      db: string
    }

/**
 * creates connection string for the specified driver accordingly
 * @returns connection string
 */
export function constructConnectionString(params: ConnectionStringParams) {
  let connString = ""
  switch (params.driver) {
    case Drivers.SQLite:
      connString = `${params.driver}:${params.filePath}`
      break
    case Drivers.PostgreSQL:
    case Drivers.MySQL:
      connString = `${params.driver}://${params.username}:${params.password}@${params.host}:${params.port}/${params.db}`
      break
  }
  return connString
}

/**
 * @see https://github.com/orgs/react-hook-form/discussions/1991#discussioncomment-31308
 */
export function dirtyValues(
  dirtyFields: object | boolean,
  allValues: object
): object {
  if (dirtyFields === true || Array.isArray(dirtyFields)) return allValues
  return Object.fromEntries(
    Object.keys(dirtyFields).map((key) => [
      key,
      // @ts-ignore
      dirtyValues(dirtyFields[key], allValues[key])
    ])
  )
}


export type ProcessLogType = 'info' | 'debug' | 'error' | 'event'

export type ProcessLog = {
  type: ProcessLogType
  date: Date
  text: string
}

export const useProcessLogger = () => {
  const processLogs = ref<ProcessLog[]>([])

  const info = (text: string) => {
    processLogs.value.push({
      type: 'info',
      date: new Date(),
      text,
    })
  }

  const debug = (text: string) => {
    processLogs.value.push({
      type: 'debug',
      date: new Date(),
      text,
    })
  }

  const error = (text: string) => {
    processLogs.value.push({
      type: 'error',
      date: new Date(),
      text,
    })
  }

  const event = (text: string) => {
    processLogs.value.push({
      type: 'event',
      date: new Date(),
      text,
    })
  }

  event('Connected')

  return {
    logs: processLogs,

    info,
    debug,
    error,
    event,
  }
}

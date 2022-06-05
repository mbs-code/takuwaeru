import { invoke } from '@tauri-apps/api/tauri'
import { Page } from '@/apis/usePageAPI'

export type Queue = {
  id: number
  site_id: number
  page_id: number
  priority: number
  created_at: string
  updated_at: string
  page: Page
}

///

export type FormQueue = {
  site_id: number,
  page_id: number,
  priority: number,
}

export type SearchQueue = {
  siteId: number,
  page: number,
  perPage: number,
  order?: string,
  desc?: boolean,
}

///

export const useQueueAPI = () => {
  const count = async (siteId: number) => {
    const num: number = await invoke('queue_count', {
      siteId,
    })
    return num
  }

  const list = async (search: SearchQueue) => {
    const queues: Queue[] = await invoke('queue_list', search)
    return queues
  }

  const get = async (queueId: number) => {
    const queue: Queue = await invoke('queue_get', {
      queueId,
    })
    return queue
  }

  const create = async (param: FormQueue) => {
    const queue: Queue = await invoke('queue_create', {
      param,
    })
    return queue
  }

  const update = async (queueId: number, param: FormQueue) => {
    const queue: Queue = await invoke('queue_update', {
      pageId: queueId, param,
    })
    return queue
  }

  const remove = async (queueId: number) => {
    const result: boolean = await invoke('queue_delete', {
      pageId: queueId,
    })
    return result
  }

  const clear = async (siteId: number) => {
    const result: boolean = await invoke('queue_clear', {
      siteId,
    })
    return result
  }

  return {
    count,
    list,
    get,
    create,
    update,
    remove,
    clear,
  }
}

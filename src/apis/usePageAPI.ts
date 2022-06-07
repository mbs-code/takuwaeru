import { invoke } from '@tauri-apps/api/tauri'

export type Page = {
  id: number
  site_id: number
  parent_page_id?: number
  url: string
  title ?: string
  is_persist: boolean
  created_at: string
  updated_at: string
}

///

export type FormPage = {
  site_id: number
  parent_page_id?: number
  url: string
  title?: string
  is_persist: boolean
}

export type SearchPage = {
  siteId: number
  page: number
  perPage: number
  order?: string
  desc?: boolean
}

///

export const usePageAPI = () => {
  const count = async (siteId: number) => {
    const num: number = await invoke('page_count', {
      siteId,
    })
    return num
  }

  const list = async (search: SearchPage) => {
    const pages: Page[] = await invoke('page_list', search)
    return pages
  }

  const get = async (pageId: number) => {
    const page: Page = await invoke('page_get', {
      pageId,
    })
    return page
  }

  const update = async (pageId: number, param: FormPage) => {
    const page: Page = await invoke('page_update', {
      pageId, param,
    })
    return page
  }

  const remove = async (pageId: number) => {
    const result: boolean = await invoke('page_delete', {
      pageId,
    })
    return result
  }

  const clear = async (siteId: number, keepPersist: boolean) => {
    const result: boolean = await invoke('page_clear', {
      siteId, keepPersist,
    })
    return result
  }

  return {
    count,
    list,
    get,
    update,
    remove,
    clear,
  }
}

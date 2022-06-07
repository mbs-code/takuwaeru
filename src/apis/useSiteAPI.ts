import { invoke } from '@tauri-apps/api/tauri'

export type ProcessorType = 'extract' | 'download'

export type SiteQuery = {
  id: number
  site_id: number
  key: string
  url_pattern: string
  processor: ProcessorType
  dom_selector?: string
  url_filter: string
  title_filter?: string
  nest_parent: number
  is_persist: boolean
  priority: number
  created_at: string
  updated_at: string
}

export type Site = {
  id: number
  key: string
  url: string
  title?: string
  created_at: string
  updated_at: string
  site_queries: SiteQuery[]
}

///

export type FormSiteQuery = {
  id?: number
  key: string
  url_pattern: string
  processor: ProcessorType
  dom_selector?: string
  url_filter: string
  title_filter?: string
  nest_parent: number
  is_persist: boolean
  priority: number
}

export type FormSite = {
  key: string
  url: string
  title?: string
  site_queries: FormSiteQuery[]
}

export type SearchSite = {
  page: number
  perPage: number
  order?: string
  desc?: boolean
}

///

export const useSiteAPI = () => {
  const count = async () => {
    const num: number = await invoke('site_count')
    return num
  }

  const list = async (search: SearchSite) => {
    const sites: Site[] = await invoke('site_list', search)
    return sites
  }

  const get = async (siteId: number) => {
    const site: Site = await invoke('site_get', {
      siteId,
    })
    return site
  }

  const create = async (param: FormSite) => {
    const site: Site = await invoke('site_create', {
      param,
    })
    return site
  }

  const update = async (siteId: number, param: FormSite) => {
    const site: Site = await invoke('site_update', {
      siteId, param,
    })
    return site
  }

  const remove = async (siteId: number) => {
    const result: boolean = await invoke('site_delete', {
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
  }
}

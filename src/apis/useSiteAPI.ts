import { invoke } from '@tauri-apps/api/tauri'

export type ProcessorType = 'extract' | 'image'

export type SiteQuery = {
  id: number
  site_id: number
  key: string
  url_pattern: string
  processor: ProcessorType,
  url_filter: string
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

export type SiteQueryParam = {
  key: string,
  url_pattern: string,
  processor: ProcessorType,
  url_filter: string,
  priority: number,
}

export type SiteParam = {
  key: string,
  url: string,
  title?: string,
  site_queries: SiteQueryParam[]
}

// export type FormReport = {
//   title?: string
//   body: string
//   tag_names: string[]
// }

// export type SearchReport = {
//   text?: string
//   page: number
//   count: number
//   latest: boolean
//   useUpdatedAt: boolean
//   timezoneOffsetSec: number
// }

export const useSiteAPI = () => {
  const list = async () => {
    const sites: Site[] = await invoke('site_list', {
      page: 1,
    })
    return sites
  }

  const get = async (siteId: number) => {
    const site: Site = await invoke('site_get', {
      siteId,
    })
    return site
  }

  const create = async (param: SiteParam) => {
    const site: Site = await invoke('site_create', {
      param,
    })
    return site
  }

  const update = async (siteId: number, param: SiteParam) => {
    const site: Site = await invoke('site_update', {
      siteId, param,
    })
    return site
  }
  // const update = async (reportId: number, form: FormReport) => {
  //   const report: Report = await invoke('report_update', {
  //     reportId: reportId,
  //     params: form,
  //   })
  //   return report
  // }

  // const remove = async (reportId: number) => {
  //   const result: boolean = await invoke('report_remove', {
  //     reportId: reportId,
  //   })
  //   return result
  // }

  return {
    list,
    get,
    create,
    update,
    // remove,
  }
}

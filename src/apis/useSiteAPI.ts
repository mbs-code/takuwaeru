import { invoke } from '@tauri-apps/api/tauri'

export type SiteQuery = {
  id: number
  site_id: number
  key: string
  url_pattern: string
  processor: string
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

  // const create = async (form: FormReport) => {
  //   const report: Report = await invoke('report_create', {
  //     params: form,
  //   })
  //   return report
  // }

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
    // create,
    // update,
    // remove,
  }
}

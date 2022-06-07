import { Queue } from '~~/src/apis/useQueueAPI'
import { Site, SiteQuery } from '~~/src/apis/useSiteAPI'

type QueryStatus = 'wait' | 'exec' | 'success' | 'skip'

type QueryResult = {
  query: SiteQuery,
  status: QueryStatus,
  task: number,
  maxTask: number,
}

export const useProcessResult = () => {
  const selectedQueue = ref<Queue>()
  const queryResults = ref<{ [key: string]: QueryResult }>({})

  const init = (site: Site) => {
    selectedQueue.value = null
    queryResults.value = Object.fromEntries(site.site_queries.map((query: SiteQuery) =>
      [query.id, { query, status: 'wait', task: 0, maxTask: 0 }]
    ))
  }

  const setQueue = (queue?: Queue) => {
    selectedQueue.value = queue
  }

  const setQueryStatus = (query: SiteQuery, status: QueryStatus) => {
    queryResults.value[query.id].status = status
  }

  const setQueryTaskCnt = (query: SiteQuery, taskCnt: number) => {
    queryResults.value[query.id].task = 0
    queryResults.value[query.id].maxTask = taskCnt
  }

  const setQueryTaskIncrement = (query: SiteQuery) => {
    queryResults.value[query.id].task++
  }

  ///

  const latestBlob = ref<number[]>([])

  return {
    selectedQueue,
    queryResults,
    latestBlob,

    init,
    setQueue,
    setQueryStatus,
    setQueryTaskCnt,
    setQueryTaskIncrement,
  }
}

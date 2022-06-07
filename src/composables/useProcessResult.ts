import { Queue } from '~~/src/apis/useQueueAPI'
import { SiteQuery } from '~~/src/apis/useSiteAPI'

type QueryStatus = 'wait' | 'exec' | 'success' | 'skip'

type QueryResult = {
  query: SiteQuery,
  status: QueryStatus,
  task: number,
  maxTask: number,
}

export const useProcessResult = () => {
  const selectedQueue = ref<Queue>()

  const setQueue = (queue?: Queue) => {
    selectedQueue.value = queue
  }

  ///

  const queryResults = ref<{ [key: string]: QueryResult }>({})

  const setQueryStatus = (query: SiteQuery, status: QueryStatus) => {
    queryResults.value[query.id] = {
      query,
      status,
      task: 0,
      maxTask: 0,
    }
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

  ///

  const clear = () => {
    selectedQueue.value = null
    queryResults.value = {}
  }

  return {
    selectedQueue,
    queryResults,
    latestBlob,

    clear,
    setQueue,
    setQueryStatus,
    setQueryTaskCnt,
    setQueryTaskIncrement,
  }
}

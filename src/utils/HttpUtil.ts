import { fetch, Response, ResponseType } from '@tauri-apps/api/http'
import { load as cheerioLoad } from 'cheerio'
import valvelet from 'valvelet'

const INTERVAL = 1200 // TODO: 再調整、valvelet内部で再度delay関数を使ってゆらぎを出してもいいかも

// throttle
const limit = valvelet(
  (callback: () => Promise<unknown>) => { return callback() },
  1,
  INTERVAL,
)

export default class HttpUtil {
  public static async fetchBody (
    url: string,
    refererUrl?: string,
    onFetched?: (res: Response<string>) => void,
  ) {
    // http を叩いて取ってくる
    const res = (await limit(() => fetch(url, {
      method: 'GET',
      responseType: ResponseType.Text,
      headers: { Referer: refererUrl }
    })
    )) as unknown as Response<string>
    if (onFetched) { onFetched(res) }

    // dom変換する
    const body = res.data
    const $ = cheerioLoad(body)
    return $
  }

  public static async fetchBlob (
    url: string,
    refererUrl?: string,
    onFetched?: (res: Response<Buffer>) => void,
  ) {
    // http を叩いて取ってくる
    const res = (await limit(() => fetch(url, {
      method: 'GET',
      responseType: ResponseType.Binary,
      headers: { Referer: refererUrl }
    })
    )) as unknown as Response<Buffer>
    if (onFetched) { onFetched(res) }

    // data 取得
    const body = res.data
    return body
  }
}

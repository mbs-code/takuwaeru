import { fetch, Response, ResponseType } from '@tauri-apps/api/http'
import { load as cheerioLoad } from 'cheerio'

export default class HttpUtil {
  public static async fetchBody (
    url: string,
    refererUrl?: string,
    onFetched?: (res: Response<string>) => void,
  ) {
    // TODO: スロットル作成

    // http を叩いて取ってくる
    const res = await fetch<string>(url, {
      method: 'GET',
      responseType: ResponseType.Text,
      headers: { Referer: refererUrl }
    })
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
    // TODO: スロットル作成

    // http を叩いて取ってくる
    const res = await fetch<Buffer>(url, {
      method: 'GET',
      responseType: ResponseType.Binary,
      headers: { Referer: refererUrl }
    })
    if (onFetched) { onFetched(res) }

    // data 取得
    const body = res.data
    return body
  }
}

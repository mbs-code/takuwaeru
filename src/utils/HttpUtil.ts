import { fetch, ResponseType } from '@tauri-apps/api/http'
import { load as cheerioLoad } from 'cheerio'

export default class HttpUtil {
  public static async fetchBody (
    url: string,
    refererUrl?: string,
    onFetched?: (size: number) => void,
  ) {
    // TODO: スロットル作成

    // http を叩いて取ってくる
    const res = await fetch(url, {
      method: 'GET',
      responseType: ResponseType.Text,
      headers: { Referer: refererUrl }
    })

    const body = res.data as string
    if (onFetched) { onFetched(body.length) }

    // dom変換する
    const $ = cheerioLoad(body)
    return $
  }

  public static async fetchBlob (
    url: string,
    refererUrl?: string,
    onFetched?: (size: number) => void,
  ) {
    // TODO: スロットル作成

    // http を叩いて取ってくる
    const res = await fetch(url, {
      method: 'GET',
      responseType: ResponseType.Binary,
      headers: { Referer: refererUrl }
    })

    const body = res.data as number[]
    if (onFetched) { onFetched(body.length) }

    return body
  }
}

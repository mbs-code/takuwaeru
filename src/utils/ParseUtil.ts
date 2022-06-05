import { CheerioAPI } from 'cheerio'

export default class ParseUtil {
  public static extractLinks ($: CheerioAPI, domQuery = '[href],[src]', urlFilter?: string) {
    // リンクを抽出する
    const set = new Set<string>()

    $(domQuery).each((_i, el) => {
      const href = $(el).attr('href')
      if (href) { set.add(href) }

      const src = $(el).attr('src')
      if (src) { set.add(src) }
    })

    let links = [...set]

    // フィルターをかける
    if (urlFilter) {
      const matcher = new RegExp(urlFilter)
      links = links.filter(link => matcher.test(link))
    }

    return links
  }
}

export default class ParseUtil {
  public static extractLinks (document: Document, domSelector = '[href],[src]', urlFilter?: string) {
    // リンクを抽出する
    const set = new Set<string>()

    const doms = document.querySelectorAll(domSelector)
    for (const dom of doms) {
      const href = dom.getAttribute('href')
      if (href) { set.add(href) }

      const src = dom.getAttribute('src')
      if (src) { set.add(src) }
    }

    let links = [...set]

    // フィルターをかける
    if (urlFilter) {
      const matcher = new RegExp(urlFilter)
      links = links.filter(link => matcher.test(link))
    }

    return links
  }

  public static urlLastName (url: string) {
    const u = new URL(url)
    const pathname = u.pathname
    const lastname = pathname.slice(Math.max(0, pathname.lastIndexOf('/') + 1))
    return lastname
  }
}

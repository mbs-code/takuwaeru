export class InterruptError extends Error {
  constructor (message?: string) {
    super(message ?? '強制停止')
  }
}

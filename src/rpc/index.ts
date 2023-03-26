import { v1 } from 'uuid'
import type { Transfer } from './transfer'
import { IPCTransfer } from './transfer'

const JUDAGE_SERVICE = 'JUDAGE_SERVICE'

export enum NError {
  MethodNotFound = 'MethodNotFound',
  InvalidParams = 'InvalidParams',
  InternalError = 'InternalError',
}

export interface NRequest<T = any> {
  id: string
  service: string
  method: string
  params: T[]
}

export interface NResponse<T = any> {
  id: string
  result: T
  error: NError | null
  message: string
}

export class Client {
  private _proxyObjectlist: Record<string, any> = {}
  private _transfer: Transfer = new IPCTransfer()

  async get<T extends Record<string, any>>(service: string): Promise<T> {
    if (this._proxyObjectlist[service]) return this._proxyObjectlist[service]

    const rsp = await this._transfer.invoke({
      id: v1(),
      service,
      method: JUDAGE_SERVICE,
      params: [],
    })

    if (rsp.error) throw new Error(rsp.message)

    const po: Record<string, any> = {}

    const createProxy = (method: string) => {
      if (['then', 'catch', 'finally'].includes(method)) return

      return async (...args: any[]): Promise<any> => {
        const rsp = await this._transfer.invoke({
          id: v1(),
          service,
          method,
          params: args,
        })

        if (rsp.error) throw new Error(rsp.message)

        return rsp.result
      }
    }

    for (const key in po)
      po[key] = createProxy(key)

    const proxy = new Proxy(po, {
      get: (target, key) => target[key as string],
    }) as T

    this._proxyObjectlist[service] = proxy
    return proxy
  }
}

export const client = new Client()

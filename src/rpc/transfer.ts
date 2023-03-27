import { invoke } from '@tauri-apps/api/tauri'
import { NError } from '.'
import type { NRequest, NResponse } from '.'

export interface Transfer {
  invoke: (request: NRequest) => Promise<NResponse>
}

export class IPCTransfer implements Transfer {
  async invoke(request: NRequest): Promise<NResponse> {
    try {
      const resp = await invoke<NResponse>('recive_message', { request })
      if (resp.id !== request.id)
        throw new Error('Message error')
      return resp
    }
    catch (error) {
      // eslint-disable-next-line prefer-promise-reject-errors
      return Promise.reject<NResponse>({
        id: request.id,
        result: null,
        error: NError.InternalError,
        message: 'System error',
      })
    }
  }
}

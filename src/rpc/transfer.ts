import { invoke } from "@tauri-apps/api/tauri";
import { NRequest, NResponse, NError } from ".";

export interface Transfer {
    invoke: (request: NRequest) => Promise<NResponse>
}

export class IPCTransfer implements Transfer {
    async invoke(request: NRequest): Promise<NResponse> {
        try {
            const resp = await invoke<NResponse>('recive_message', { request: request})
            if (resp.id !== request.id) {
                throw new Error()
            }
            return resp
        } catch (error) {
            return Promise.reject<NResponse>({
                id: request.id,
                result: null,
                error: NError.InternalError,
                message: "System error"
            })
        }   
    }
}
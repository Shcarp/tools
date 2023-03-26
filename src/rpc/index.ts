import { v1 } from "uuid";
import { Transfer, IPCTransfer } from "./transfer";

const JUDAGE_SERVICE = "JUDAGE_SERVICE"

export enum NError {
    MethodNotFound = "MethodNotFound",
    InvalidParams = "InvalidParams",
    InternalError = "InternalError",
}

export interface NRequest<T = any> {
    id: string;
    service: string;
    method: string;
    params: T[];
}

export interface NResponse<T = any> {
    id: string;
    result: T;
    error: NError | null;
    message: string;
}

export class Client {
    private _proxyObjectlist: Record<string, any> = {};
    private _transfer: Transfer = new IPCTransfer();

    async get<T extends Record<string, any>>(service: string): Promise<T> {
        if (this._proxyObjectlist[service]) {
            return this._proxyObjectlist[service];
        }

        const rsp = await this._transfer.invoke({
            id: v1(),
            service,
            method: JUDAGE_SERVICE,
            params: [],
        });

        if (rsp.error) {
            throw new Error(rsp.message);
        }
        const po: Record<string, any> = {};
        const proxy = new Proxy(po, {
            get: (target, key) => {
                if(["then", "catch", "finally"].includes(key as string)) {
                    return
                }
                target[key as string] = () => {};
                return new Proxy(target[key as string], {
                    apply: (target, thisArg, args) => {
                        return new Promise(async (resolve, reject) => {
                            const callRes = await this._transfer.invoke({
                                id: v1(),
                                service: service,
                                method: key as string,
                                params: args,
                            });
                            if (callRes.error) reject(callRes.message)
                            resolve(callRes.result)
                        });
                    },
                });
            },
        }) as T;
        this._proxyObjectlist[service] = proxy;
        return proxy;
    }
}

export const client = new Client()

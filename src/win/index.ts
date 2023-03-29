import { invoke } from "@tauri-apps/api"

export type WinOptions = {
    win_type: string,
    overopen: boolean, // 是否多开
    url: string, // 当前支持page 名称 
    position: [number, number],
    height: number,
    width: number,
    min_width: number,
    min_height: number,
    max_width: number,
    max_height: number,
    resizable: boolean,
    title: string,
    fullscreen: string,
    focus: boolean
}
type Value = string | null | boolean | number | Array<Value> | Object;

export interface WinInterface {
    open: (label: string, args: Record<string, Value>) => Promise<string>,
    register: (options: WinOptions) => Promise<void>,
    close: (label: string) => Promise<void>,
    hide: (label: string) => Promise<void>
}

class Win implements WinInterface {
    register (options: WinOptions): Promise<void> {
        return Promise.resolve()
    }

    close (label: string): Promise<void> {
        return Promise.resolve()
    }

    hide (label: string): Promise<void> {
        return Promise.resolve()
    }

    open(label: string, args: Record<string, Value>): Promise<string> {
        return Promise.resolve("main")
    }
}



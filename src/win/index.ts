import { invoke } from "@tauri-apps/api";

type WinRequired = {
    win_type: string;
    url: string; // 当前支持page 名称
}

type WinOptional = {
    center?: boolean
    overopen?: boolean; // 是否多开
    position?: [number, number];
    height?: number;
    width?: number;
    min_width?: number;
    min_height?: number;
    max_width?: number;
    max_height?: number;
    resizable?: boolean;
    title?: string;
    fullscreen?: string;
    focus?: boolean;
}

export type WinOptions = WinRequired & WinOptional

type Value = string | null | boolean | number | Array<Value> | Object;

export interface WinInterface {
    open: (label: string, args: WinOptions) => Promise<string>;
    register: (options: WinOptions) => Promise<void>;
    close: (label: string) => Promise<void>;
    hide: (label: string) => Promise<void>;
}

class Win implements WinInterface {
    async register(options: WinOptions): Promise<void> {
        try {
            await invoke('register_win', {options})
        } catch (error) {
            console.log(error)
        }
    }

    async close(label: string): Promise<void> {
        try {
            await invoke('register_win', {label})
        } catch (error) {
            console.log(error)
        }
    }

    async hide(label: string): Promise<void> {
        try {
            await invoke('register_win', {label})
        } catch (error) {
            console.log(error)
        }
    }

    async open(label: string, args: Record<string, any>): Promise<string> {
        try {
            const newWinLabel = await invoke('open', { label: label, args })
            return newWinLabel as string
        } catch (error) {
            console.log(error)
            return ''
        }
    }
}

export default new Win()


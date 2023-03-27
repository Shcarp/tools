import { invoke } from '@tauri-apps/api/tauri'
import type { IEditProps } from './page/edit'

async function openWindow(name: 'main', args: {}): Promise<void>
async function openWindow(name: 'edit', args: IEditProps): Promise<void>
async function openWindow(name: string, args: any) {
  return await invoke('open', { name, args })
}

export const open = openWindow

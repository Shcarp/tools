import { invoke } from '@tauri-apps/api/tauri'
import type { IEditProps } from './page/edit'

async function openWindow(name: 'main', args: {}): Promise<void>
async function openWindow(name: 'edit', args: IEditProps): Promise<void>
async function openWindow(name: string, args: any) {
  try {
    const res = await invoke('open', { label: name, args })
    console.log(res)
    return res
  }
  catch (error) {
    console.log(error)
  }
}

export const open = openWindow



import { invoke } from '@tauri-apps/api/tauri'
import { Button } from 'antd'
import { run } from '../../entry'
import type { PageCommonProps } from '../interface'

export interface IEditProps {
  id: string
}

const Edit: React.FC<IEditProps & PageCommonProps> = () => {
  return (
    <Button onClick={async () => {
      await invoke('open', { name: 'main', args: '111' })
    }}> open</Button>
  )
}

run<IEditProps>(Edit)

import { invoke } from '@tauri-apps/api/tauri'
import { Button } from 'antd'
import { run } from '../../entry'
import type { PageCommonProps } from '../interface'

export interface IEditProps {
  id: string
}

const Edit: React.FC<IEditProps & PageCommonProps> = (props) => {
  console.log(props)
  return (
    <Button onClick={async () => {
      await invoke('open', { name: 'main', args: { hhhh: '我是二号啊', x: '你干啥', y: ['111', '22', '333'] } })
    }}> open</Button>
  )
}

run<IEditProps>(Edit)

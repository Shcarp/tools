import React from 'react'
import { Button } from 'antd'
import { client } from '../../rpc'
import type { PageCommonProps } from '../interface'
import { info, error } from 'tauri-plugin-log-api'
import 'antd/dist/reset.css'
import './App.less'
import win from '../../win'

interface TestRemoteObj {
  height: () => Promise<number>
  width: () => Promise<number>
}

export interface IAppProps {
  id: string
}

function App(props: IAppProps & PageCommonProps) {
  info(JSON.stringify(props), {file: "/log"})
  error(JSON.stringify(props))
  const getTest = async () => {
    const res = await client.get<TestRemoteObj>('test')
    console.log(await res.height())
  }

  return (
    <div className="container">
      <Button onClick={async () => {
        await getTest()
      }}>FASON</Button>
      <Button onClick={async () => {
        win.open('edit', { id: 'edit' })
      }}>
        Open ediyt
      </Button>
    </div>
  )
}

export default App

import React from 'react'
import { Button } from 'antd'
import { client } from '../../rpc'
import type { PageCommonProps } from '../interface'
import { open } from '../../utils'
import 'antd/dist/reset.css'
import './App.less'

interface TestRemoteObj {
  height: () => Promise<number>
  width: () => Promise<number>
}

export interface IAppProps {
  id: string
}

function App(props: IAppProps & PageCommonProps) {
  console.log(props)
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
        open('edit', { id: 'edit' })
      }}>
        Open ediyt
      </Button>
    </div>
  )
}

export default App

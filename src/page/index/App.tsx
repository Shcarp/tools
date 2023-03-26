import React from 'react'
import { Button } from 'antd'
import { client } from '../../rpc'
import 'antd/dist/reset.css'
import './App.less'

interface TestRemoteObj {
  height: () => Promise<number>
  width: () => Promise<number>
}

function App() {
  const getTest = async () => {
    const res = await client.get<TestRemoteObj>('test')
    console.log(await res.height())
  }

  return (
    <div className="container">
      <Button onClick={async () => {
        await getTest()
      }}>FASON</Button>
    </div>
  )
}

export default App

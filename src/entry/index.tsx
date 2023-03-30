import React, { useEffect, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { appWindow } from '@tauri-apps/api/window'
import type { PageCommonProps } from '../page/interface'
import '../page/index'

export function run<T>(Entry: React.FC<T & PageCommonProps>) {
  const Wrap = () => {
    const [initState, setInitState] = useState<T & PageCommonProps>(JSON.parse(window?.__MY_CUSTOM_PROPERTY__ ?? null))
    useEffect(() => {
      const data = JSON.parse(window?.__MY_CUSTOM_PROPERTY__ ?? null)
      setInitState(data)
      appWindow.listen('open', (args) => {
        setInitState(JSON.parse(args?.payload as string ?? '{}'))
      })
    }, [])
    return <>
      <Entry {...initState}></Entry>
    </>
  }

  ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
      <Wrap />
    </React.StrictMode>,
  )
}

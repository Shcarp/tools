import React from 'react'
import ReactDOM from 'react-dom/client'
import type { PageCommonProps } from '../page/interface'

export function run<T>(Entry: React.FC<T & PageCommonProps>) {
  const initState = JSON.parse(window.__MY_CUSTOM_PROPERTY__)

  ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
      <Entry {...initState}></Entry>
    </React.StrictMode>,
  )
}

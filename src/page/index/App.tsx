import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window'
import { Button } from "antd";
import "antd/dist/reset.css";
import "./App.less";
import { client } from "../../rpc";

interface TestRemoteObj {
  height: () => Promise<number>,
  width: () => Promise<number>,
}

function App() {
  const [name, setName] = useState("");
  const getTest = async () => {
    let res = await client.get<TestRemoteObj>('test')
    console.log(await res.height())
  }

  return (
    <div className="container">
      <Button onClick={async () => {
        await getTest()
      }}>FASON</Button>
    </div>
  );
}

export default App;

import { Button, Layout, PageHeader } from 'antd';
import './App.css';
import AocMenu from './AocMenu.js';
import { Outlet } from "react-router-dom";

import * as wasm from "aocwasm";

const { Header, Sider, Content } = Layout;

wasm.greet()

function App() {
  return (
    <div className="App">
    <PageHeader className="site-page-header" title="AoC Web 2022 !" />
    <Layout>
      <Sider><AocMenu /></Sider>
      <Content>
        <Outlet />
      </Content>
    </Layout>
    </div>
  );
}

export default App;

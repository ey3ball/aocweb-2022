import { Layout } from 'antd';
import { PageHeader } from '@ant-design/pro-layout';
import AocMenu from './AocMenu.js';
import { Outlet } from "react-router-dom";

import 'antd/dist/reset.css';
import './App.css';

import * as wasm from "aocwasm";

const { Header, Sider, Content } = Layout;

wasm.set_panic_hook();

function App() {
  return (
    <div className="App">
    <PageHeader className="site-page-header" title="AoC Web 2022 !" />
    <Layout>
      <Sider><AocMenu /></Sider>
      <Content className="page-content">
        <Outlet />
      </Content>
    </Layout>
    </div>
  );
}

export default App;

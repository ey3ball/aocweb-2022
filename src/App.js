import { Button, Layout, PageHeader } from 'antd';
import './App.css';
import AocMenu from './AocMenu.js';

const { Header, Sider, Content } = Layout;

function App() {
  return (
    <div className="App">
    <PageHeader className="site-page-header" title="AoC Web 2022 !" />
    <Layout>
      <Sider><AocMenu /></Sider>
      <Content><h1>Welcome to the AoC web toy project</h1>This project will be used as a playground to get familiar with : React, Antd, and WASM</Content>
    </Layout>
    Hello World<br />
    <Button type="primary">Button</Button>
    </div>
  );
}

export default App;

import { Button, Layout, PageHeader } from 'antd';
import './App.css';
import AocMenu from './AocMenu.js';
import ErrorPage from './Error404.js';
import {
  createBrowserRouter,
  RouterProvider,
  Route,
} from "react-router-dom";
import Root from './routes/Root.js';
import Day01 from './routes/Day01.js';

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />
  },
  {
    path: "/day/01",
    element: <Day01 />
  }
]);

const { Header, Sider, Content } = Layout;

function App() {
  return (
    <div className="App">
    <PageHeader className="site-page-header" title="AoC Web 2022 !" />
    <Layout>
      <Sider><AocMenu /></Sider>
      <Content>
        <RouterProvider router={router} />
      </Content>
    </Layout>
    </div>
  );
}

export default App;

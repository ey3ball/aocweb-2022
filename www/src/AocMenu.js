import React from 'react';
import { Menu } from 'antd';
import { Link } from "react-router-dom";

// works when >=4.20.0, recommended ✅
const items = [
  { label: (<Link to ="/">README</Link>), key: 'home' },
  { label: (<Link to="/day/01">Day 1</Link>), key: 'day-01' },
  { label: (<Link to="/day/02">Day 2</Link>), key: 'day-02' },
  { label: '...', key: 'day-xx' },
];

class AocMenu extends React.Component {
    render() {
      return <Menu theme="dark" items={items} />;
    }
}

export default AocMenu;

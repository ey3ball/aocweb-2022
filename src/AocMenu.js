import React from 'react';
import { Menu } from 'antd';

// works when >=4.20.0, recommended ✅
const items = [
  { label: 'README', key: 'home' }, // remember to pass the key prop
  { label: 'Day 1', key: 'day-01' }, // remember to pass the key prop
  { label: 'Day 2', key: 'day-02' }, // which is required
  { label: 'Day 3', key: 'day-03' }, // which is required
  { label: 'Day 4', key: 'day-04' }
];

class AocMenu extends React.Component {
    render() {
      return <Menu theme="dark" items={items} />;
    }
}

export default AocMenu;

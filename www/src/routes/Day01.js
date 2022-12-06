import React from 'react';
import { Button, Form, Input } from 'antd';
import * as wasm from "aocwasm";

const { TextArea } = Input;


export default class Day01 extends React.Component {
  constructor(props) {
    super(props)
    this.state = {
        part1: "",
        part2: ""
    }
  }

  onFinish(values) {
    console.log("test")
    console.log(this)
    var day = wasm.Day01.parse(values.input)
    var p1 = day.part1();
    var p2 = day.part2();

    this.setState({part1: p1, part2: p2});
    console.log("test")
  }

  render() {
    return (
      <>
        <Form
          labelCol={{ span: 8 }}
          wrapperCol={{ span: 16 }}
          onFinish={(values) => this.onFinish(values)}
        >
          <Form.Item
            Label="Problem Input"
            name="input"
            rules={[{ required: true, message: 'Problem input required !' }]}
          >
            <TextArea rows={10} style={{flex: 0.8}} />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit">
              Solution ?
            </Button>
          </Form.Item>
        </Form>
        <Input addonBefore="Part ⭐" disabled={true} style={{width: 300}} value={this.state.part1} />
        <br />
        <Input addonBefore="Part 🤩" disabled={true} style={{width: 300}} value={this.state.part2} />
      </>
    )
  }
}

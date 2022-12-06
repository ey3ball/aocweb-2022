import React from 'react';
import { Divider, Button, Form, Input, Select } from 'antd';
import * as wasm from "aocwasm";

const { TextArea } = Input;

const SAMPLE = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`;

const OPTIONS = [
  {label: "sample", value: SAMPLE},
  {label: "custom", value: ""}
]

export default class Day01 extends React.Component {
  formRef = React.createRef();

  constructor(props) {
    super(props)
    this.state = {
        part1: "",
        part2: ""
    }
  }

  onFinish(values) {
    var day = wasm.Day01.parse(values.input)
    var p1 = day.part1();
    var p2 = day.part2();

    this.setState({part1: p1, part2: p2});
  }

  onSelect(label, option) {
    this.formRef.current.setFieldsValue({input: option.value})
  }

  render() {
    return (
      <>
        <h1>Day 1</h1>
        <Form
          ref={this.formRef}
          labelCol={{ span: 8 }}
          wrapperCol={{ span: 16 }}
          onFinish={(values) => this.onFinish(values)}
        >
          <Form.Item
            label="Input file"
            name="preset_input"
          >
            <Select options={OPTIONS} onSelect={(label, option) => this.onSelect(label, option)}/>
          </Form.Item>
          <Form.Item
            label="Problem Input"
            name="input"
            initialValue={OPTIONS[0].value}
            rules={[{ required: true, message: 'Problem input required !' }]}
          >
            <TextArea rows={10} />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit">
              Solution ?
            </Button>
          </Form.Item>
        </Form>
        <Divider>Solutions</Divider>
        <Input addonBefore="Part ⭐" disabled={true} style={{width: 300}} value={this.state.part1} />
        <br />
        <Input addonBefore="Part 🤩" disabled={true} style={{width: 300}} value={this.state.part2} />
      </>
    )
  }
}

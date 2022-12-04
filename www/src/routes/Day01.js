import React from 'react';
import { Button, Checkbox, Form, Input } from 'antd';

const { TextArea } = Input;

export default function Day01() {
  return (
    <>
      <Form
        labelCol={{ span: 8 }}
        wrapperCol={{ span: 16 }}
      >
        <Form.Item
          Label="Problem Input"
          name="ProblemInput"
          rules={[{ required: true, message: 'Problem input required !' }]}
        >
          <TextArea rows={10}/>
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Compute solution
          </Button>
        </Form.Item>
      </Form>
    </>
  )
}

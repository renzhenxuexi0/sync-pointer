import { RedoOutlined, SaveOutlined } from '@ant-design/icons';
import { Button, Card, Form } from 'antd';
import { ReactNode, useState } from 'react';

interface SettingsCardProps<T extends Record<string, unknown>> {
  title: string;
  initialValues: T;
  onFinish: (values: T) => void;
  children: ReactNode;
}

export default function SettingsCard<T extends Record<string, unknown>>({
  title,
  initialValues,
  onFinish,
  children,
}: SettingsCardProps<T>) {
  const [isChanged, setIsChanged] = useState(false);
  const [form] = Form.useForm();

  const handleReset = () => {
    form.resetFields();
    setIsChanged(false);
  };

  const handleSave = () => {
    form.submit();
    setIsChanged(false);
  };

  return (
    <Card
      title={title}
      extra={
        isChanged && (
          <div>
            <Button
              icon={<RedoOutlined />}
              type="text"
              onClick={handleReset}
            ></Button>
            <Button
              icon={<SaveOutlined />}
              type="text"
              onClick={handleSave}
            ></Button>
          </div>
        )
      }
    >
      <Form<T>
        form={form}
        labelAlign="left"
        labelCol={{ span: 6 }}
        size="small"
        onFinish={onFinish}
        onFinishFailed={() => form.resetFields()}
        onValuesChange={() => {
          setIsChanged(true);
        }}
        initialValues={initialValues}
      >
        {children}
      </Form>
    </Card>
  );
}

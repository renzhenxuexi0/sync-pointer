import { FooterToolbar, ProForm } from '@ant-design/pro-components';

export interface SettingsFormProps<T extends Record<string, unknown> | undefined> {
  initialValues: T;
  onFinish: (values: T) => void;
  children?: React.ReactNode;
}

function SettingsForm<T extends Record<string, unknown> | undefined>(props: SettingsFormProps<T>) {
  return (
    <div>
      <ProForm
        submitter={{
          render: (_, dom) => <FooterToolbar>{dom}</FooterToolbar>,
        }}
        layout="horizontal"
        colon={false}
        onFinish={props.onFinish}
        initialValues={props.initialValues}
      >
        {props.children}
      </ProForm>
    </div>
  );
}

export default SettingsForm;

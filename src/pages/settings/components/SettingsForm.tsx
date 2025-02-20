import { FooterToolbar, ProForm, ProFormInstance } from '@ant-design/pro-components';
import { notification } from 'antd';
import React, { useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';

export interface SettingsFormProps<T extends Record<string, unknown> | undefined> {
  initialValues: T;
  onFinish: (values: T) => Promise<void>;
  children?: React.ReactNode;
}

function SettingsForm<T extends Record<string, unknown> | undefined>(props: SettingsFormProps<T>) {
  const { t } = useTranslation();
  const formRef = useRef<ProFormInstance | undefined>(undefined);
  const [api, contextHolder] = notification.useNotification();
  const [isChange, setIsChange] = useState(false);
  return (
    <div>
      {/* <Context.Provider value={{ name: 'Ant Design' }}> */}
      {contextHolder}
      <ProForm
        formRef={formRef}
        submitter={{
          render: (_, dom) => <FooterToolbar>{dom}</FooterToolbar>,
          searchConfig: {
            resetText: t('settings.form.reset'),
            submitText: t('settings.form.save'),
          },
          submitButtonProps: {
            disabled: !isChange,
          },
          resetButtonProps: {
            disabled: !isChange,
          },
        }}
        onValuesChange={() => {
          if (!isChange) {
            setIsChange(true);
          }
        }}
        layout="horizontal"
        colon={false}
        onReset={() => {
          setIsChange(false);
        }}
        onFinish={async (values: T) => {
          await props.onFinish(values);
          api.success({
            message: t('settings.form.save-success'),
            duration: 1,
          });
          setIsChange(false);
        }}
        initialValues={props.initialValues}
      >
        {props.children}
      </ProForm>
    </div>
  );
}

export default SettingsForm;

import { FooterToolbar, ProForm, ProFormInstance } from '@ant-design/pro-components';
import { notification } from 'antd';
import React, { useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';

export interface SettingsFormProps<T extends Record<string, unknown> | undefined> {
  initialValues: T;
  onFinish: (values: T, form?: ProFormInstance) => Promise<void>;
  children?: React.ReactNode;
}

function SettingsForm<T extends Record<string, unknown> | undefined>(props: SettingsFormProps<T>) {
  const { t } = useTranslation();
  const formRef = useRef<ProFormInstance | undefined>(undefined);
  const [api, contextHolder] = notification.useNotification();
  const [isChange, setIsChange] = useState(false);
  return (
    <div>
      {contextHolder}
      <ProForm
        formRef={formRef}
        requiredMark={false}
        request={async () => props.initialValues}
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
          onReset: () => {
            formRef.current?.setFieldsValue(props.initialValues);
            setIsChange(false);
          },
        }}
        onValuesChange={() => {
          if (!isChange) {
            setIsChange(true);
          }
        }}
        layout="horizontal"
        colon={false}
        onFinish={async (values: T) => {
          await props.onFinish(values, formRef.current);
          api.success({
            message: t('settings.form.save-success'),
            duration: 1,
          });
          setIsChange(false);
        }}
      >
        {props.children}
      </ProForm>
    </div>
  );
}

export default SettingsForm;

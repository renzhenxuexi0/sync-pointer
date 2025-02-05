<script setup lang="ts">
import { usePreferenceStore } from '@/store/preference';

const langs = ref([
    {
        name: 'English(US)',
        value: 'en-US',
    },
    {
        name: '简体中文',
        value: 'zh-CN',
    },
]);
const { preference, setPreferenceLocale } = usePreferenceStore();
</script>

<template>
    <div
        class="flex h-full w-full flex-col items-center justify-center bg-surface-0 dark:bg-surface-900"
    >
        <Select
            :model-value="preference.locale"
            :options="langs"
            @change="(e) => setPreferenceLocale(e.value)"
            option-label="name"
            option-value="value"
            :placeholder="$t('setting.select-language')"
            class="w-full md:w-56"
        />
    </div>
</template>

<script setup lang="ts">
import { usePreferenceStore } from '@/store/preference';
import FluentSettings16Filled from '~icons/fluent/settings-16-filled';
import FluentWeatherMoon16Filled from '~icons/fluent/weather-moon-16-filled';
import FluentWeatherSunny16Filled from '~icons/fluent/weather-sunny-16-filled';
const { t } = useI18n();

const themeOptions = ref([
    {
        label: () => t('default.light-theme'),
        i: FluentWeatherSunny16Filled,
        value: 'light',
    },
    {
        label: () => t('default.system-theme'),
        i: FluentSettings16Filled,
        value: 'system',
    },
    {
        label: () => t('default.dark-theme'),
        i: FluentWeatherMoon16Filled,
        value: 'dark',
    },
]);
const preferenceStore = usePreferenceStore();
const { setPreferenceTheme, preference } = preferenceStore;
</script>

<template>
    <el-container>
        <el-aside width="200px">
            <el-menu
                default-active="1"
                class="h-full"
            >
                <el-menu-item index="1">
                    <el-icon>
                        <i-fluent-layout-row-two-focus-top-settings-20-filled />
                    </el-icon>
                    <span>Navigator Two</span>
                </el-menu-item>
            </el-menu>
        </el-aside>
    </el-container>
</template>

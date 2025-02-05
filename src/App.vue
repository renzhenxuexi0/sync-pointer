<script setup lang="ts">
import FluentSettings16Filled from '~icons/fluent/settings-16-filled';
import FluentWeatherMoon16Filled from '~icons/fluent/weather-moon-16-filled';
import FluentWeatherSunny16Filled from '~icons/fluent/weather-sunny-16-filled';
// import { usePreferenceStore } from './store/preference';

const { t } = useI18n();

const themeOptions = reactive([
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
// const { setPreferenceTheme, getPreferenceTheme } = usePreferenceStore();
</script>

<template>
    <v-layout>
        <title-bar :resizable="true" />
        <!-- <SelectButton
                        :model-value="getPreferenceTheme"
                        @change="
                            (event) => {
                                setPreferenceTheme(event.value);
                            }
                        "
                        :options="themeOptions"
                        option-label="label"
                        option-value="value"
                    >
                        <template #option="slotProps">
                            <component
                                :is="slotProps.option.i"
                                v-tooltip="slotProps.option.label()"
                                class="size-5"
                            />
                        </template>
                    </SelectButton> -->
        <router-view class="h-full w-[95%]" />
    </v-layout>
</template>

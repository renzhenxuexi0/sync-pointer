<script setup lang="ts">
import logo from '@/assets/logo.png';
import { MenuItem } from 'primevue/menuitem';
import { FunctionalComponent } from 'vue';
import FluentHome24Regular from '~icons/fluent/home-24-regular';
import FluentSettings16Filled from '~icons/fluent/settings-16-filled';
import FluentSettings24Regular from '~icons/fluent/settings-24-regular';
import FluentWeatherMoon16Filled from '~icons/fluent/weather-moon-16-filled';
import FluentWeatherSunny16Filled from '~icons/fluent/weather-sunny-16-filled';
import { usePreferenceStore } from './store/preference';

const { t } = useI18n();

const items = ref<
    (MenuItem & {
        routerName?: string;
        i?: FunctionalComponent;
    })[]
>([
    {
        separator: true,
    },
    {
        label: () => t('default.home-title'),
        routerName: 'home',
        i: FluentHome24Regular as FunctionalComponent,
    },
    {
        label: () => t('default.setting-title'),
        routerName: 'setting',
        i: FluentSettings24Regular as FunctionalComponent,
    },
    {
        separator: true,
    },
]);

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
const { setPreferenceTheme, getPreferenceTheme } = usePreferenceStore();
</script>

<template>
    <div class="flex h-screen w-screen flex-col">
        <title-bar
            :resizable="true"
            class="max-h-[5%]"
        />
        <div class="flex h-[95%] w-full">
            <Menu
                :model="items"
                :pt="{
                    root: '!rounded-none',
                    start: 'h-1/8',
                    end: 'h-1/8',
                    list: 'h-3/4',
                }"
                auto-z-index
                class="h-full"
            >
                <template #start>
                    <div class="mb-1 flex w-full items-center justify-center">
                        <Avatar
                            :image="logo"
                            size="xlarge"
                        />
                    </div>
                </template>
                <template #item="{ item, props }">
                    <router-link
                        :to="{ name: item.routerName }"
                        v-slot="{ href, navigate }"
                        class="flex w-full items-center"
                        custom
                    >
                        <a
                            v-ripple
                            :href="href"
                            v-bind="props.action"
                            @click="navigate"
                        >
                            <component
                                :is="item.i"
                                class="size-5"
                            />
                            <span class="ml-2">{{
                                typeof item.label === 'function'
                                    ? item.label()
                                    : item.label
                            }}</span>
                        </a>
                    </router-link>
                </template>
                <template #end>
                    <div class="flex w-full items-center justify-center">
                        <SelectButton
                            :model-value="getPreferenceTheme"
                            @change="
                                (event) => {
                                    const theme = event.value as
                                        | 'light'
                                        | 'system'
                                        | 'dark';
                                    if (theme === 'system') {
                                        setPreferenceTheme(undefined);
                                    } else {
                                        setPreferenceTheme(theme);
                                    }
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
                        </SelectButton>
                    </div>
                </template>
            </Menu>
            <router-view class="h-full w-full" />
        </div>
    </div>
</template>

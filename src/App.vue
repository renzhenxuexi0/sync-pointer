<script setup lang="ts">
import logo from '@/assets/logo.png';
import { MenuItem } from 'primevue/menuitem';
import { FunctionalComponent } from 'vue';
import FluentHome24Regular from '~icons/fluent/home-24-regular';
import FluentSettings24Regular from '~icons/fluent/settings-24-regular';

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
        label: 'Home',
        routerName: 'home',
        i: FluentHome24Regular as FunctionalComponent,
    },
    {
        label: 'Setting',
        routerName: 'setting',
        i: FluentSettings24Regular as FunctionalComponent,
    },
]);
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
                auto-z-index
                class="h-full"
            >
                <template #start>
                    <div
                        class="mb-1 flex h-16 w-full items-center justify-center"
                    >
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
                            <span class="ml-2">{{ item.label }}</span>
                        </a>
                    </router-link>
                </template>
            </Menu>
            <router-view class="h-full" />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

const props = defineProps({
    resizable: Boolean,
});

const appWindow = getCurrentWebviewWindow();

async function minimize() {
    await appWindow.minimize();
}

async function close() {
    await appWindow.close();
}

async function maximize() {
    if (await appWindow.isMaximized()) {
        await appWindow.unmaximize();
    } else {
        await appWindow.maximize();
    }
}
</script>

<template>
    <div
        class="bg-opacity-40 text-primary-foreground flex h-6 w-full content-center justify-end bg-slate-200 dark:bg-slate-900"
        data-tauri-drag-region
    >
        <n-button
            class="!h-6 !w-10 rounded-none hover:!bg-slate-200 dark:hover:!bg-slate-800"
            v-if="props.resizable"
            variant="ghost"
            @click="minimize"
            quaternary
        >
            <i-fluent-minimize-16-regular
                class="text-slate-300 dark:text-slate-400"
            />
        </n-button>
        <n-button
            class="!h-6 !w-10 !rounded-none hover:!bg-slate-200 dark:hover:!bg-slate-800"
            v-if="props.resizable"
            variant="ghost"
            @click="maximize"
            quaternary
        >
            <i-fluent-maximize-16-regular
                class="text-slate-300 dark:text-slate-400"
            />
        </n-button>
        <n-button
            class="!h-6 !w-10 !rounded-none rounded-tr-lg hover:!bg-red-500"
            variant="ghost"
            @click="close"
            quaternary
        >
            <i-fluent-dismiss-16-regular
                class="text-slate-300 hover:text-slate-100 dark:text-slate-400 dark:hover:text-slate-100"
            />
        </n-button>
    </div>
</template>

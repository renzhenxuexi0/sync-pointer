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
        class="text-primary-foreground flex h-full w-full content-center justify-end border-b border-surface-200 bg-surface-0 dark:border-surface-700 dark:bg-surface-900"
        data-tauri-drag-region
    >
        <Button
            class="h-full w-10 !rounded-none"
            v-if="props.resizable"
            @click="minimize"
            severity="secondary"
            variant="text"
        >
            <i-fluent-minimize-16-regular />
        </Button>
        <Button
            class="h-full w-10 !rounded-none"
            v-if="props.resizable"
            @click="maximize"
            severity="secondary"
            variant="text"
        >
            <i-fluent-maximize-16-regular />
        </Button>
        <Button
            class="h-full w-10 !rounded-none rounded-tr-lg"
            @click="close"
            severity="danger"
            variant="text"
        >
            <i-fluent-dismiss-16-regular />
        </Button>
    </div>
</template>

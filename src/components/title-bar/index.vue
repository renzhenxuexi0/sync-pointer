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
        class="text-primary-foreground flex h-full w-full content-center justify-end bg-surface-800"
        data-tauri-drag-region
    >
        <Button
            class="h-full w-10 !rounded-none !text-surface-200"
            v-if="props.resizable"
            @click="minimize"
            variant="text"
        >
            <i-fluent-minimize-16-regular />
        </Button>
        <Button
            class="h-full w-10 !rounded-none !text-surface-200"
            v-if="props.resizable"
            @click="maximize"
            variant="text"
        >
            <i-fluent-maximize-16-regular />
        </Button>
        <Button
            class="h-full w-10 !rounded-none rounded-tr-lg !text-surface-200 hover:!bg-[#e81123]"
            @click="close"
            variant="text"
        >
            <i-fluent-dismiss-16-regular />
        </Button>
    </div>
</template>

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
    <v-app-bar>
        <template #prepend>
            <v-app-bar-nav-icon />
        </template>
        <template #append>
            <v-btn
                class="h-full w-10 !rounded-none"
                v-if="props.resizable"
                @click="minimize"
                severity="secondary"
                variant="text"
            >
                <i-fluent-minimize-16-regular />
            </v-btn>
            <v-btn
                class="h-full w-10 !rounded-none"
                v-if="props.resizable"
                @click="maximize"
                severity="secondary"
                variant="text"
            >
                <i-fluent-maximize-16-regular />
            </v-btn>
            <v-btn
                class="h-full w-10 !rounded-none rounded-tr-lg"
                @click="close"
                severity="danger"
                variant="text"
            >
                <i-fluent-dismiss-16-regular />
            </v-btn>
        </template>
    </v-app-bar>
</template>

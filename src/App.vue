<script setup lang="ts"></script>

<template>
    <n-config-provider>
        <div class="flex h-screen w-screen flex-col">
            <title-bar :resizable="true" />
            <router-view class="flex-1" />
        </div>
    </n-config-provider>
</template>

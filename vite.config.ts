import tailwindcss from '@tailwindcss/vite';
import vue from '@vitejs/plugin-vue';
import path from 'path';
import AutoImport from 'unplugin-auto-import/vite';
import IconsResolver from 'unplugin-icons/resolver';
import Icons from 'unplugin-icons/vite';
import Components from 'unplugin-vue-components/vite';
import { defineConfig, loadEnv } from 'vite';
import vuetify from 'vite-plugin-vuetify';

// https://vitejs.dev/config/
export default defineConfig(async ({ mode }) => {
    const env = loadEnv(mode, process.cwd()); // 获取.env文件里定义的环境变量
    const host = env.TAURI_DEV_HOST;
    const devTool = env.VITE_DEVTOOLS;
    return {
        plugins: [
            vue(),
            vuetify({ autoImport: { labs: true } }),
            tailwindcss(),
            Icons({
                compiler: 'vue3',
            }),
            AutoImport({
                dts: 'src/auto-imports.d.ts',
                imports: ['vue', 'vue-i18n', 'vue-router'],
            }),
            Components({
                // 指定组件位置，默认是src/components
                dirs: ['src/components'],
                extensions: ['vue'],
                // 配置文件生成位置
                dts: 'src/components.d.ts',
                // ui库解析器
                resolvers: [IconsResolver()],
            }),
            devTool === 'true'
                ? import('vite-plugin-vue-devtools').then((i) => i.default())
                : undefined,
        ],
        resolve: {
            alias: {
                '@': path.resolve(__dirname, './src'),
            },
        },

        // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
        //
        // 1. prevent vite from obscuring rust errors
        clearScreen: false,
        // 2. tauri expects a fixed port, fail if that port is not available
        server: {
            port: 1420,
            strictPort: true,
            host: host || false, // 如果有 TAURI_DEV_HOST 则使用，否则仅监听 localhost
            hmr: host
                ? {
                      protocol: 'ws', // WebSocket 协议
                      host, // HMR 服务器地址
                      port: 1421, // HMR 服务器端口
                  }
                : undefined, // 如果没有 host，使用默认 HMR 配置
            watch: {
                // 3. tell vite to ignore watching `src-tauri`
                ignored: ['**/src-tauri/**'],
            },
        },
    };
});

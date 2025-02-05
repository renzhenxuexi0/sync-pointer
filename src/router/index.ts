import { loadLocaleMessages } from '@/locales';
import {
    createRouter,
    createWebHistory,
    type RouteRecordRaw,
} from 'vue-router';

const routes: RouteRecordRaw[] = [
    {
        name: 'setting',
        path: '/',
        component: () => import('@/views/setting/index.vue'),
        children: [
            {
                name: 'screen-layout',
                path: '/screen-layout',
                component: () =>
                    import('@/views/setting/screen-layout/index.vue'),
            },
        ],
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

// 配置路由守卫懒加载本地化配置
router.beforeEach(async (to, _from, next) => {
    await loadLocaleMessages(to.name as string);
    next();
});

export default router;

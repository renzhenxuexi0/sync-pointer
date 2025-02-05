import { loadLocaleMessages } from '@/locales';
import {
    createRouter,
    createWebHistory,
    type RouteRecordRaw,
} from 'vue-router';

const routes: RouteRecordRaw[] = [
    {
        name: 'home',
        path: '/',
        component: () => import('@/views/home/index.vue'),
    },
    {
        name: 'setting',
        path: '/setting',
        component: () => import('@/views/setting/index.vue'),
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

// 配置路由守卫懒加载本地化配置
router.beforeEach(async (to, _from, next) => {
    await loadLocaleMessages(to.name as 'home' | 'setting');
    next();
});

export default router;

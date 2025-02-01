import pluginJs from '@eslint/js';
import pluginPrettierRecommended from 'eslint-plugin-prettier/recommended';
import pluginVue from 'eslint-plugin-vue';
import globals from 'globals';
import tseslint from 'typescript-eslint';
import parseVue from 'vue-eslint-parser';

/** @type {import('eslint').Linter.Config[]} */
export default [
    // 基础配置：应用所有文件（显式覆盖 ESLint 默认忽略规则）
    {
        files: ['**/*.{js,mjs,cjs,ts,vue}'],
    },
    { languageOptions: { globals: globals.browser } },
    pluginJs.configs.recommended,
    {
        ignores: ['src-tauri/**/*.js'],
    },
    ...tseslint.configs.recommended,
    ...pluginVue.configs['flat/essential'],
    {
        files: ['**/*.vue'],
        languageOptions: {
            parser: parseVue,
            parserOptions: { parser: tseslint.parser },
        },
    },
    pluginPrettierRecommended,
    {
        rules: {
            'prettier/prettier': 'error',
            ...pluginVue.configs.base.rules,
            ...pluginVue.configs['vue3-essential'].rules,
            ...pluginVue.configs['vue3-strongly-recommended'].rules,
            ...pluginVue.configs['vue3-recommended'].rules,

            'vue/attribute-hyphenation': [
                'error',
                'always',
                {
                    ignore: [],
                },
            ],
            'vue/attributes-order': 'off',
            'vue/block-order': [
                'error',
                {
                    order: ['script', 'template', 'style'],
                },
            ],
            'vue/component-name-in-template-casing': ['error', 'PascalCase'],
            'vue/component-options-name-casing': ['error', 'PascalCase'],
            'vue/custom-event-name-casing': ['error', 'camelCase'],
            'vue/define-macros-order': [
                'error',
                {
                    order: [
                        'defineOptions',
                        'defineProps',
                        'defineEmits',
                        'defineSlots',
                    ],
                },
            ],
            'vue/dot-location': ['error', 'property'],
            'vue/dot-notation': ['error', { allowKeywords: true }],
            'vue/eqeqeq': ['error', 'smart'],
            'vue/html-closing-bracket-newline': 'error',
            'vue/html-indent': 'off',
            // 'vue/html-indent': ['error', 2],
            'vue/html-quotes': ['error', 'double'],
            'vue/html-self-closing': [
                'error',
                {
                    html: {
                        component: 'always',
                        normal: 'never',
                        void: 'always',
                    },
                    math: 'always',
                    svg: 'always',
                },
            ],
            'vue/max-attributes-per-line': 'off',
            'vue/multi-word-component-names': 'off',
            'vue/multiline-html-element-content-newline': 'error',
            'vue/no-empty-pattern': 'error',
            'vue/no-extra-parens': ['error', 'functions'],
            'vue/no-irregular-whitespace': 'error',
            'vue/no-loss-of-precision': 'error',
            'vue/no-reserved-component-names': 'off',
            'vue/no-restricted-syntax': [
                'error',
                'DebuggerStatement',
                'LabeledStatement',
                'WithStatement',
            ],
            'vue/no-restricted-v-bind': ['error', '/^v-/'],
            'vue/no-sparse-arrays': 'error',
            'vue/no-unused-refs': 'error',
            'vue/no-useless-v-bind': 'error',
            'vue/object-shorthand': [
                'error',
                'always',
                {
                    avoidQuotes: true,
                    ignoreConstructors: false,
                },
            ],
            'vue/one-component-per-file': 'error',
            'vue/prefer-import-from-vue': 'error',
            'vue/prefer-separate-static-class': 'error',
            'vue/prefer-template': 'error',
            'vue/prop-name-casing': ['error', 'camelCase'],
            'vue/require-default-prop': 'error',
            'vue/require-explicit-emits': 'error',
            'vue/require-prop-types': 'off',
            'vue/script-setup-uses-vars': 'error',
            'vue/singleline-html-element-content-newline': 'off',
            'vue/space-infix-ops': 'error',
            'vue/space-unary-ops': ['error', { nonwords: false, words: true }],
            'vue/v-on-event-hyphenation': [
                'error',
                'always',
                {
                    autofix: true,
                    ignore: [],
                },
            ],
        },
    },
];

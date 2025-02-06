const config = {
    endOfLine: 'auto',
    printWidth: 80,
    tabWidth: 4,
    useTabs: false,
    semi: true,
    singleQuote: true,
    quoteProps: 'as-needed',
    trailingComma: 'all',
    bracketSpacing: true,
    arrowParens: 'always',
    htmlWhitespaceSensitivity: 'css',
    singleAttributePerLine: true,
    overrides: [
        {
            files: ['*.json5'],
            options: {
                quoteProps: 'preserve',
                singleQuote: false,
            },
        },
    ],
    plugins: ['prettier-plugin-tailwindcss'],
    proseWrap: 'never',
};

export default config;

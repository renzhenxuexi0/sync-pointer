const config = {
  endOfLine: 'auto',
  printWidth: 100,
  tabWidth: 2,
  useTabs: false,
  semi: true,
  singleQuote: true,
  quoteProps: 'as-needed',
  trailingComma: 'all',
  bracketSpacing: true,
  arrowParens: 'always',
  jsxSingleQuote: false, // JSX 中使用双引号
  jsxBracketSameLine: false, // JSX 标签的 > 放在新行
  htmlWhitespaceSensitivity: 'css',
  singleAttributePerLine: true,
  overrides: [
    {
      files: ['*.json', '*.json5'],
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

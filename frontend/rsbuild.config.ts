import { defineConfig } from '@rsbuild/core'
import { pluginReact } from '@rsbuild/plugin-react'

export default defineConfig({
  html: {
    title: 'Car Alerts',
    favicon: './public/favicon.svg',
  },
  plugins: [pluginReact()],
  tools: {
    postcss: {
      postcssOptions: {
        plugins: ['@tailwindcss/postcss'],
      },
    },
  },
})

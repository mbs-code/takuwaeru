import { defineNuxtConfig } from 'nuxt'
import eslintPlugin from 'vite-plugin-eslint'

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  srcDir: 'src/',
  ssr: false,
  target: 'static',

  modules: [
    '@inkline/nuxt',
  ],

  inkline: {
    // Plugin options (optional)
  },

  vite: {
    plugins: [eslintPlugin()]
  },

  build: {
    postcss: {
      postcssOptions: {
        plugins: {
          tailwindcss: {},
        },
      },
    },
  },
})

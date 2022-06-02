import { defineNuxtConfig } from 'nuxt'
import eslintPlugin from 'vite-plugin-eslint'

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  srcDir: 'src/',
  ssr: false,
  target: 'static',

  css: [
    '@/assets/index.scss',
  ],

  components: [
    {
      path: '@/components/',
      pathPrefix: false,
    },
  ],

  build: {
    transpile: ['primevue']
  },

  vite: {
    plugins: [eslintPlugin({
      fix: true,
    })]
  },
})

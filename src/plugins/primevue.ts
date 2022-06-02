import { defineNuxtPlugin } from 'nuxt/app'
import PrimeVue from 'primevue/config'
import ToastService from 'primevue/toastservice'

import Button from 'primevue/button'
import Dialog from 'primevue/Dialog'
import Avatar from 'primevue/avatar'
import InputText from 'primevue/inputtext'
import Toast from 'primevue/toast'

import 'primevue/resources/themes/saga-blue/theme.css'
import 'primevue/resources/primevue.min.css'
import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css'

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.use(PrimeVue, { ripple: true })
  nuxtApp.vueApp.use(ToastService)

  nuxtApp.vueApp.component('Button', Button)
  nuxtApp.vueApp.component('Dialog', Dialog)
  nuxtApp.vueApp.component('Avatar', Avatar)
  nuxtApp.vueApp.component('InputText', InputText)
  nuxtApp.vueApp.component('Toast', Toast)
})

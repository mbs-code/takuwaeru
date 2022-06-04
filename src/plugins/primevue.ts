import { defineNuxtPlugin } from 'nuxt/app'
import PrimeVue from 'primevue/config'
import ToastService from 'primevue/toastservice'
import ConfirmationService from 'primevue/confirmationservice'

import Button from 'primevue/button'
import Dialog from 'primevue/Dialog'
import Avatar from 'primevue/avatar'
import InputText from 'primevue/inputtext'
import Toast from 'primevue/toast'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Toolbar from 'primevue/toolbar'
import ConfirmDialog from 'primevue/confirmdialog'
import Card from 'primevue/card'
import SelectButton from 'primevue/selectbutton'
import Textarea from 'primevue/textarea'

import 'primevue/resources/themes/saga-blue/theme.css'
import 'primevue/resources/primevue.min.css'
import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css'

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.use(PrimeVue, { ripple: true })
  nuxtApp.vueApp.use(ToastService)
  nuxtApp.vueApp.use(ConfirmationService)

  nuxtApp.vueApp.component('Button', Button)
  nuxtApp.vueApp.component('Dialog', Dialog)
  nuxtApp.vueApp.component('Avatar', Avatar)
  nuxtApp.vueApp.component('InputText', InputText)
  nuxtApp.vueApp.component('Toast', Toast)
  nuxtApp.vueApp.component('DataTable', DataTable)
  nuxtApp.vueApp.component('Column', Column)
  nuxtApp.vueApp.component('Toolbar', Toolbar)
  nuxtApp.vueApp.component('ConfirmDialog', ConfirmDialog)
  nuxtApp.vueApp.component('Card', Card)
  nuxtApp.vueApp.component('SelectButton', SelectButton)
  nuxtApp.vueApp.component('Textarea', Textarea)
})

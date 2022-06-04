<template>
  <div>
    <div>
      <nuxt-link class="no-underline-all" :to="{ name: 'index' }">
        <Button class="m-1" label="Home" />
      </nuxt-link>

      <Button class="m-1" label="Edit" @click="showEditModal = true" />
    </div>

    {{ site }}

    <!-- <p>{{ $route.params.id }}</p>
    <p>{{ site }}</p>

    <div class="card card-bordered card-compact border-sky-500 border-1">
      <div class="card-body">
        <h2 class="card-title">
          <div class="badge badge-secondary">
            {{ site.key }}
          </div>
          {{ site.title }}
        </h2>
        <p>{{ site.url }}</p>
      </div>
    </div>

    <div v-for="(query, _) in site.site_queries" :key="_">
      {{ query }}
    </div> -->

    <SiteEditDialog
      v-model:show="showEditModal"
      :site="site"
      @saved="onSaved"
    />
  </div>
</template>

<script setup lang="ts">
import { Site, useSiteAPI } from '@/apis/useSiteAPI'

const route = useRoute()
const siteAPI = useSiteAPI()

const site = ref<Site>()

onMounted(async () => {
  const siteId = parseInt(route.params.id?.['0'])
  site.value = await siteAPI.get(siteId)
})

const showEditModal = ref<boolean>(false)
const onSaved = (newSite: Site) => {
  site.value = newSite
}
</script>

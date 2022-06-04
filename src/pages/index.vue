<template>
  <article class="h-screen">
    <Toolbar>
      <template #start>
        <nuxt-link class="no-underline-all" :to="{ name: 'sites-id', params: { id: 5 } }">
          <Button class="p-button-secondary" label="Site 5" />
        </nuxt-link>
      </template>
    </Toolbar>

    <SiteListComponent />
  </article>
</template>

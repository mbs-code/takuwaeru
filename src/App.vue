<template>
  <Toolbar>
    <template #start>
      <nuxt-link class="no-underline-all" :to="{ name: 'index' }">
        <Button class="p-button-secondary" label="Home" />
      </nuxt-link>
    </template>
  </Toolbar>

  <NuxtPage class="bg-indigo-50 h-screen" />

  <Toast position="top-right" />
  <ConfirmDialog />
</template>

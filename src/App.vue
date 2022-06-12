<template>
  <div class="bg-indigo-50 flex flex-column h-screen">
    <Toolbar>
      <template #start>
        <nuxt-link class="no-underline-all" :to="{ name: 'index' }">
          <Button class="p-button-secondary" label="Home" />
        </nuxt-link>
      </template>
    </Toolbar>

    <div class="flex-grow-1">
      <NuxtPage />
    </div>

    <Toast position="top-right" />
    <ConfirmDialog />
  </div>
</template>

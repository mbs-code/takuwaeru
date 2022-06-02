<template>
  <div>
    <!-- <i-button tag="nuxt-link" :to="{ name: 'sites-id', params: { id: 5 } }">
      Site test
    </i-button> -->
    <nuxt-link class="no-underline-all" :to="{ name: 'sites-id', params: { id: 5 } }">
      <Button label="Site 5" />
    </nuxt-link>

    <!-- <div>{{ sites[5] }}</div> -->

    <!-- <NuxtWelcome /> -->
  </div>
</template>

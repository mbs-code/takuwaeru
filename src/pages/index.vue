<template>
  <div>
    <i-button tag="nuxt-link" :to="{ name: 'sites-id', params: { id: 5 } }">
      Site test
    </i-button>

    <!-- <div>{{ sites[5] }}</div> -->

    <NuxtWelcome />
  </div>
</template>

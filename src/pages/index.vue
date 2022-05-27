<template>
  <div>
    <img alt="Vue logo" src="~assets/logo.png">
    <HelloWorld msg="Hello Vue 3 + TypeScript + Vite" />
    <h1 class="pl-24 pt-12 text-3xl text-red-600 underline">
      Hello world!
    </h1>

    <button class="btn">
      Hello daisyUI
    </button>

    <nuxt-link class="btn" :to="{ name: 'sites-id', params: { id: 5 } }">
      asd
    </nuxt-link>

    <!-- <div>{{ sites[5] }}</div> -->

    <!-- <NuxtWelcome /> -->
  </div>
</template>

<template>
  <v-col class="text-right" id="nav-bar">
    <v-btn :to="{ name: 'roomSearch' }" variant="outlined"> Book </v-btn>
    <v-btn> Discover </v-btn>
    <v-btn> Account </v-btn>
    <v-btn> Service </v-btn>
  </v-col>
</template>

<script>
export default {
  name: "the-navigation-bar",
};
</script>

<style>
</style>
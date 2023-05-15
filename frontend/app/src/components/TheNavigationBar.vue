<template>
  <v-col class="text-right" id="nav-bar">
    <div style="position: relative; top: 10px">
      <v-btn :to="{ name: 'roomSearch' }" variant="outlined"> Book </v-btn>
      <v-btn> Discover </v-btn>
      <v-btn> Account </v-btn>
      <v-btn> Service </v-btn>
    </div>
  </v-col>
</template>

<script>
export default {
  name: "the-navigation-bar",
};
</script>

<style>
#nav-bar {
  position: fixed;
  right: 0;
  width: 40vw;
  background: linear-gradient(to right, rgba(0, 0, 0, 0), rgba(0, 0, 0, 1));
  height: 100%;
}
</style>
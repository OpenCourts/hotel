<template>
  <div class="page">
    <the-header />
    <div class="mt-10">
      <slot> </slot>
    </div>
    <!-- <the-footer /> -->
  </div>
</template>

<script>
import TheHeader from "./TheHeader.vue";
export default {
  components: { TheHeader },
  name: "base-view",
};
</script>

<style scoped>
.page {
  height: 80%;
}
</style>
<template>
  <div class="page">
    <the-header />
    <div class="mt-10">
      <slot> </slot>
    </div>
    <the-footer />
  </div>
</template>

<script>
import TheFooter from "./TheFooter.vue";
import TheHeader from "./TheHeader.vue";
export default {
  components: { TheHeader, TheFooter },
  name: "base-view",
};
</script>

<style scoped>
.page {
  height: 100%;
}
</style>
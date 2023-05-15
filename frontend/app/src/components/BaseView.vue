<template>
  <div class="page">
    <the-header />
    <slot> </slot>
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
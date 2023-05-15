<template>
  <v-autocomplete :items="locations" />
</template>

<script>
import { mapState } from "vuex";
export default {
  name: "hotel-picker",
  computed: {
    ...mapState("hotel", ["hotels"]),
    locations() {
      return this.hotels.map((h) => h.location);
    },
  },
};
</script>

<style>
</style>
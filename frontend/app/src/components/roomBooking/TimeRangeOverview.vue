<template>
  <v-container>
    <v-row>
      <v-col :lg="space ? 6 : 3">
        <p>from</p>
        <p class="text-h4">{{ startDate.toLocaleDateString() }}</p>
      </v-col>
      <v-col :lg="space ? 6 : 3">
        <p>to</p>
        <p class="text-h4">{{ endDate.toLocaleDateString() }}</p>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import { mapGetters } from "vuex";
export default {
  name: "time-range-overview",
  computed: {
    ...mapGetters("roomType", ["startDate", "endDate"]),
  },
  props: {
    space: {
        type: Boolean,
        required: false,
        default: false
    }
  }
};
</script>

<style>
</style>
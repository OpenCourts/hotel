<template>
  <v-container>
    <v-card>
      <v-list>
        <v-label class="ml-4 my-2 text-h6"> Price Range</v-label>
        <v-list-item>
          <v-text-field label="min. price" variant="outlined" />
        </v-list-item>
        <v-list-item>
          <v-text-field label="max. price" variant="outlined" />
        </v-list-item>
        <v-divider v-if="availableFilters?.length > 1" />
        <v-label class="ml-4 my-2 text-h6" v-if="availableFilters?.length > 1"> Amenities </v-label>
        <v-list-item v-for="filter in availableFilters" :key="filter">
          <v-switch
            v-model="amenitiesFilters"
            :value="filter"
            hide-details
            color="primary"
            density="compact"
            inset
            @change="updateAmenitiesFilters"
          >
            <template #label>
              <v-label class="text-button">{{ filter }}</v-label>
            </template>
          </v-switch>
        </v-list-item>
      </v-list>
    </v-card>
  </v-container>
</template>

<script>
import { mapMutations, mapGetters, mapState } from "vuex";
export default {
  name: "filter-list",
  data() {
    return {
      amenitiesFilters: [],
    };
  },
  computed: {
    ...mapGetters("roomType", ["availableFilters"]),
    ...mapState("roomType", ["activeAmenitiesFilters"]),
  },
  methods: {
    ...mapMutations("roomType", ["setActiveAmenitiesFilters"]),
    updateAmenitiesFilters() {
      this.setActiveAmenitiesFilters(this.amenitiesFilters);
    },
  },
  created() {
    this.amenitiesFilters = this.activeAmenitiesFilters;
  },
};
</script>

<style>
</style>
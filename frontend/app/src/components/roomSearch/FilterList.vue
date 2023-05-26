<template>
  <v-container>
    <v-card>
      <v-list>
        <v-label class="ml-4 py-2 text-h6"> Price Range</v-label>
        <v-list-item>
          <v-range-slider
            class="py-2 pr-4 mt-6"
            strict
            v-model="priceRange"
            :min="10"
            :max="500"
            :step="10"
            show-ticks="always"
            :ticks="[]"
            prepend-icon="mdi-currency-eur"
            label="Price per night"
            thumb-label="always"
            @update:modelValue="setPrice"
          >
          </v-range-slider>
        </v-list-item>
        <v-divider v-if="availableFilters?.length > 1" />
        <v-label class="ml-4 my-2 text-h6" v-if="availableFilters?.length > 1">
          Amenities
        </v-label>
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
      maxPrice: null,
      minPrice: null,
      priceRange: [10, 500],
    };
  },
  computed: {
    ...mapGetters("roomType", ["availableFilters"]),
    ...mapState("roomType", ["activeAmenitiesFilters"]),
  },
  methods: {
    ...mapMutations("roomType", ["setActiveAmenitiesFilters", "setFilters"]),
    updateAmenitiesFilters() {
      this.setActiveAmenitiesFilters(this.amenitiesFilters);
    },
    setPrice() {
      this.setFilters({ minPrice: this.priceRange[0], maxPrice: this.priceRange[1] });
    },
  },
  created() {
    this.amenitiesFilters = this.activeAmenitiesFilters;
  },
};
</script>

<style>
</style>
<template>
  <v-container>
    <v-card>
      <v-list>
        <v-label class="ml-4 py-2 text-h6"> Price Range</v-label>
        <v-list-item>
          <v-text-field
            v-model.number="minPrice"
            placeholder="price per night"
            variant="outlined"
            prepend-inner-icon="mdi-currency-eur"
            @keypress.enter="setMinPrice"
            @blur="setMinPrice"
            ><template #prepend><label>from</label></template></v-text-field
          >
        </v-list-item>
        <v-list-item>
          <v-text-field
            v-model.number="maxPrice"
            placeholder="price per night"
            variant="outlined"
            prepend-inner-icon="mdi-currency-eur"
            @keypress.enter="setMaxPrice"
            @blur="setMaxPrice"
            ><template #prepend><label>to</label></template></v-text-field
          >
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
    setMinPrice() {
      this.setFilters({ minPrice: this.minPrice });
    },
    setMaxPrice() {
      this.setFilters({ maxPrice: this.maxPrice });
    },
  },
  created() {
    this.amenitiesFilters = this.activeAmenitiesFilters;
  },
};
</script>

<style>
</style>
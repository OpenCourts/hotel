<template>
  <v-autocomplete
    :items="hotels"
    v-model="selectedHotelId"
    :item-title="(h) => getHotelSearchString(h)"
    :item-value="(h) => h.id"
    label="Where to go?"
    variant="outlined"
    @update:modelValue="updateHotel"
  >
    <template v-slot:item="{ item, props }">
      <v-list-item
        class="py-2"
        v-bind="props"
        prepend-icon="mdi-map-marker"
        :title="item?.raw?.location"
        :subtitle="item?.raw?.name"
      ></v-list-item>
    </template>
    <template #prepend-inner>
      <v-icon class="mr-2"> mdi-bed </v-icon>
    </template>
  </v-autocomplete>
</template>

<script>
import { mapMutations, mapState } from "vuex";
export default {
  name: "hotel-picker",
  data() {
    return {
      selectedHotelId: null,
    };
  },
  computed: {
    ...mapState("hotel", ["hotels"]),
    ...mapState("roomType", ["apiFilters"]),
    locations() {
      return this.hotels.map((h) => h.location);
    },
    selectedHotel() {
      return this.hotels.find((h) => h.id == this.selectedHotelId);
    },
  },
  methods: {
    ...mapMutations("roomType", ["setApiFilters"]),
    getHotelSearchString(hotel) {
      return `${hotel.location}, ${hotel.name}`;
    },
    updateHotel() {
      this.setApiFilters({ hotelId: this.selectedHotelId });
    },
  },
  created() {
    this.selectedHotelId = this.apiFilters.hotelId;
  },
};
</script>

<style>
</style>
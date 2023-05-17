<template>
  <base-view>
    <v-container>
      <v-row>
        <v-col cols="4">
          <hotel-picker />
        </v-col>
        <v-col cols="3"> <number-of-persons-picker /> </v-col>
        <v-col cols="3"> <date-range-picker /></v-col>
        <v-col cols="2">
          <v-btn
            :disabled="!allApiFiltersSet"
            @click="searchRoomTypes"
            style="height: 4em; width: 100%"
            >Search</v-btn
          ></v-col
        >
      </v-row>
      <v-row>
        <v-col cols="4"><filter-list /></v-col>
        <v-col cols="8"><result-list v-if="hasClickedSearch || roomTypes.length > 0" /></v-col>
      </v-row>
    </v-container>
  </base-view>
</template>

<script>
import { mapActions, mapState } from "vuex";
import BaseView from "../components/BaseView.vue";
import DateRangePicker from "../components/roomSearch/DateRangePicker.vue";
import FilterList from "../components/roomSearch/FilterList.vue";
import HotelPicker from "../components/roomSearch/HotelPicker.vue";
import NumberOfPersonsPicker from "../components/roomSearch/NumberOfPersonsPicker.vue";
import ResultList from "../components/roomSearch/ResultList.vue";
export default {
  components: {
    BaseView,
    HotelPicker,
    NumberOfPersonsPicker,
    DateRangePicker,
    FilterList,
    ResultList,
  },
  name: "room-search-page",
  data() {
    return {
      hasClickedSearch: false,
    };
  },
  computed: {
    ...mapState("roomType", ["apiFilters", "roomTypes"]),
    allApiFiltersSet() {
      return (
        Object.values(this.apiFilters).filter((v) => v == null).length == 0
      );
    },
  },
  methods: {
    ...mapActions("roomType", ["loadRoomTypes"]),
    async searchRoomTypes() {
      await this.loadRoomTypes();
      this.hasClickedSearch = true;
    },
  },
};
</script>

<style>
</style>
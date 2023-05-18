<template>
  <base-view>
    <v-container v-if="!isLoading">
      <v-row>
        <v-col cols="4">
          <hotel-picker :disabled="loadingRoomTypes" />
        </v-col>
        <v-col cols="3">
          <number-of-persons-picker :disabled="loadingRoomTypes" />
        </v-col>
        <v-col cols="3">
          <date-range-picker :disabled="loadingRoomTypes"
        /></v-col>
        <v-col cols="2">
          <v-btn
            :disabled="!allApiFiltersSet || loadingRoomTypes"
            @click="searchRoomTypes"
            style="height: 4em; width: 100%"
            >Search</v-btn
          ></v-col
        >
      </v-row>
      <v-row>
        <v-col cols="4"><filter-list /></v-col>
        <v-col cols="8">
          <div
            :style="`opacity: ${
              hasClickedSearch && loadingRoomTypes ? 0.5 : 1
            }; transition: all .1s`"
          >
            <base-spinner v-if="loadingRoomTypes && !hasClickedSearch" />
            <result-list
              v-else-if="hasClickedSearch || roomTypes.length > 0"
            /></div
        ></v-col>
      </v-row>
    </v-container>
    <v-container class="fill-height" v-else>
      <v-row justify="center" align="center">
        <v-col class="text-center">
          <base-spinner />
        </v-col>
      </v-row>
    </v-container>
  </base-view>
</template>

<script>
import { mapActions, mapState } from "vuex";
import BaseSpinner from "../components/BaseSpinner.vue";
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
    BaseSpinner,
  },
  name: "room-search-page",
  data() {
    return {
      hasClickedSearch: false,
      isLoading: false,
      loadingRoomTypes: false,
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
    ...mapActions("hotel", ["loadHotels"]),
    async searchHotels() {
      this.isLoading = true;
      try {
        await this.loadHotels();
      } catch (e) {
        console.error(e);
      } finally {
        this.isLoading = false;
      }
    },
    async searchRoomTypes() {
      this.loadingRoomTypes = true;
      try {
        await this.loadRoomTypes();
      } catch (e) {
        console.error(e);
      } finally {
        this.hasClickedSearch = true;
        this.loadingRoomTypes = false;
      }
    },
  },
  async created() {
    await this.searchHotels();
  },
};
</script>

<style>
</style>
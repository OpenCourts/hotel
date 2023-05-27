<template>
  <base-view>
    <v-container v-if="!hasClickedSearch && roomTypes.length == 0">
      <v-row class="mt-10">
        <v-col class="text-center">
          <base-logo />
        </v-col>
      </v-row>
      <v-row>
        <v-col cols="12" class="text-center">
          <span class="text-overline" style="font-size: 2em !important"
            >Search</span
          >
          <v-icon style="font-size: 5em" class="ml-2 mr-0">mdi-magnify</v-icon>
          <span class="text-overline" style="font-size: 2em !important"
            >rooms</span
          >
        </v-col>
      </v-row>
      <v-row>
        <v-col cols="4">
          <hotel-picker />
        </v-col>
        <v-col cols="4">
          <number-of-persons-picker />
        </v-col>
        <v-col cols="4"> <date-range-picker /></v-col>
      </v-row>
    </v-container>
    <v-container fluid v-else-if="!isLoading">
      <div class="px-5">
        <v-row>
          <v-col cols="4">
            <hotel-picker />
          </v-col>
          <v-col cols="4">
            <number-of-persons-picker />
          </v-col>
          <v-col cols="4"> <date-range-picker /></v-col>
        </v-row>
        <v-row>
          <v-col cols="4"><filter-list /> </v-col>
          <v-col cols="8">
            <div
              :style="`opacity: ${
                hasClickedSearch && loadingRoomTypes ? 0.5 : 1
              }; transition: all .1s`"
            >
              <div :class="loadingRoomTypes ? 'overlay' : ''">
                <result-list />
              </div></div
          ></v-col>
        </v-row>
      </div>
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
import BaseLogo from "../components/BaseLogo.vue";
export default {
  components: {
    BaseView,
    HotelPicker,
    NumberOfPersonsPicker,
    DateRangePicker,
    FilterList,
    ResultList,
    BaseSpinner,
    BaseLogo,
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
  watch: {
    apiFilters: {
      deep: true,
      async handler() {
        await this.searchRoomTypes();
      },
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
      if (!this.allApiFiltersSet) return;
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
.overlay {
  opacity: 0.6;
}
</style>
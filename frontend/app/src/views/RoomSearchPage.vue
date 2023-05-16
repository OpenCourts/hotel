<template>
  <base-view>
    <v-container>
      <v-row>
        <v-col cols="3">
          <hotel-picker />
        </v-col>
        <v-col cols="4"> <number-of-persons-picker /> </v-col>
        <v-col cols="3"> <date-range-picker /></v-col>
        <v-col cols="2"> <v-btn style="height: 4em">Search</v-btn></v-col>
      </v-row>
    </v-container>
  </base-view>
</template>

<script>
import BaseView from "../components/BaseView.vue";
import DateRangePicker from '../components/roomSearch/DateRangePicker.vue';
import HotelPicker from "../components/roomSearch/HotelPicker.vue";
import NumberOfPersonsPicker from '../components/roomSearch/NumberOfPersonsPicker.vue';
export default {
  components: { BaseView, HotelPicker, NumberOfPersonsPicker, DateRangePicker },
  name: "room-search-page",
};
</script>

<style>
</style>
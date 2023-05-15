<template>
  <base-view>
    <v-container>
      <v-row>
        <v-col>
          <hotel-picker />
        </v-col>
        <v-col> </v-col>
      </v-row>
    </v-container>
  </base-view>
</template>

<script>
import BaseView from "../components/BaseView.vue";
import HotelPicker from "../components/roomSearch/HotelPicker.vue";
export default {
  components: { BaseView, HotelPicker },
  name: "room-search-page",
};
</script>

<style>
</style>
<template>
  <div>
    <v-container class="pb-0">
      <v-row align="center" justify="center">
        <v-col>
          <p class="text-h3 mt-7">Great choice!</p>
          <p class="text-h6">You're about to book the following apartment:</p>
        </v-col>
      </v-row>
    </v-container>
    <base-view> <room-type-result :roomType="selectedRoomType" /> </base-view>
  </div>
</template>

<script>
import { mapState } from "vuex";
import BaseView from "../components/BaseView.vue";
import RoomTypeResult from "../components/roomSearch/RoomTypeResult.vue";
export default {
  components: { BaseView, RoomTypeResult },
  name: "room-book-page",
  computed: {
    ...mapState("roomType", ["roomTypes"]),
    ...mapState("booking", ["roomTypeId"]),
    selectedRoomType() {
      return this.roomTypes.find((rt) => rt.id == this.roomTypeId);
    },
  },
};
</script>

<style>
#back-button {
  position: absolute;
}
</style>
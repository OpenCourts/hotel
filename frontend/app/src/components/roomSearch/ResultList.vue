<template>
  <v-container v-if="filteredRoomTypes.length > 0">
    <v-row v-for="roomType in filteredRoomTypes" :key="roomType.id">
      <v-col>
        <room-type-result :roomType="roomType" @goToBooking="goToBooking" />
      </v-col>
    </v-row>
  </v-container>
  <v-container v-else class="fill-height">
    <v-row justify="center" align="center">
      <v-col class="text-center">
        <p class="mb-2">
          <v-icon size="15vh" style="opacity: 0.3"
            >mdi-emoticon-sad-outline</v-icon
          >
        </p>
        <v-label style="font-size: 2.5vh"
          >Sorry, there are no rooms that match your search criteria</v-label
        >
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import { mapGetters } from "vuex";
import RoomTypeResult from "./RoomTypeResult.vue";
export default {
  components: { RoomTypeResult },
  name: "result-list",
  computed: {
    ...mapGetters("roomType", ["filteredRoomTypes"]),
  },
  methods: {
    goToBooking() {
      this.$router.push({ name: "roomBook" });
    },
  },
};
</script>

<style>
</style>
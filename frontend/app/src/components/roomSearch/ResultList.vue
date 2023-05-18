<template>
  <v-container v-if="filteredRoomTypes.length > 0">
    <v-row v-for="roomType in filteredRoomTypes" :key="roomType.id">
      <v-col>
        <v-hover>
          <template v-slot:default="{ isHovering, props }">
            <room-type-result
              :roomType="roomType"
              @goToBooking="goToBooking"
              v-bind="props"
            >
              <template #overlay>
                <v-overlay
                  @click="goToBooking(roomType)"
                  :model-value="isHovering"
                  contained
                  scrim="black"
                  class="align-center justify-end"
                  style="cursor: pointer"
                >
                  <v-btn
                    @click="goToBooking(roomType)"
                    variant="outlined"
                    class="mr-5"
                    >Book now</v-btn
                  >
                </v-overlay>
              </template>
            </room-type-result>
          </template>
        </v-hover>
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
import { mapGetters, mapMutations } from "vuex";
import RoomTypeResult from "./RoomTypeResult.vue";
export default {
  components: { RoomTypeResult },
  name: "result-list",
  computed: {
    ...mapGetters("roomType", ["filteredRoomTypes"]),
  },
  methods: {
    ...mapMutations("booking", ["setBookingInformation"]),
    goToBooking(roomType) {
      this.setBookingInformation({ roomTypeId: roomType.id });
      this.$router.push({ name: "roomBook" });
    },
  },
};
</script>

<style>
</style>
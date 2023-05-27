<template>
  <v-container>
    <v-row>
      <label class="text-h4 ml-3">{{ selectedHotel.name }}</label>
    </v-row>
    <v-row>
      <time-range-overview />
    </v-row>
  </v-container>
  <v-container
    v-if="filteredRoomTypes.length > 0"
    class="overflow-y-auto mb-5"
    style="height: 60vh"
  >
    <v-row v-if="availableRoomTypes.length > 0">
      <v-col>
        <v-label>
          {{ availableRoomTypes.length }} room type{{
            availableRoomTypes.length == 1 ? "" : "s"
          }}
          available
        </v-label>
      </v-col>
    </v-row>
    <v-row v-else>
      <v-col>
        <v-label>
          Sorry, no free rooms available!
        </v-label>
      </v-col>
    </v-row>
    <v-row v-for="roomType in availableRoomTypes" :key="roomType.id">
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
    <v-divider class="my-8" v-if="unavailableRoomTypes.length > 0" />
    <v-row v-for="roomType in unavailableRoomTypes" :key="roomType.id">
      <v-col>
        <room-type-result :roomType="roomType" disabled> </room-type-result>
      </v-col>
    </v-row>
  </v-container>
  <v-container v-else class="fill-height">
    <v-row justify="center" align="center">
      <v-col class="text-left">
        <p class="mb-2">
          <v-icon size="10vh" style="opacity: 0.3"
            >mdi-emoticon-sad-outline</v-icon
          >
        </p>
        <v-label style="font-size: 2vh"
          >Sorry, there are no available rooms that match your search
          criteria</v-label
        >
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import { mapGetters, mapMutations } from "vuex";
import TimeRangeOverview from "../roomBooking/TimeRangeOverview.vue";
import RoomTypeResult from "./RoomTypeResult.vue";
export default {
  components: { RoomTypeResult, TimeRangeOverview },
  name: "result-list",
  computed: {
    ...mapGetters("roomType", ["filteredRoomTypes", "selectedHotel"]),
    availableRoomTypes() {
      return this.filteredRoomTypes.filter((rt) => rt.availableRooms > 0);
    },
    unavailableRoomTypes() {
      return this.filteredRoomTypes.filter((rt) => rt.availableRooms == 0);
    },
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
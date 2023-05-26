<template>
  <base-view>
    <v-container class="mb-2">
      <v-row align="center" justify="center">
        <v-col>
          <p class="text-h3 mt-7">Great choice!</p>
          <p>You're about to book the following apartment:</p>
        </v-col>
      </v-row>
      <v-row>
        <v-container>
          <v-row>
            <v-col cols="6">
              <room-type-result
                class="fill-height"
                cover
                :roomType="selectedRoomType"
              />
            </v-col>
            <v-col>
              <v-card class="fill-height px-3 py-4">
                <v-container class="ml-3">
                  <v-row>
                    <v-col>
                      <p class="text-h4">{{ selectedHotel.name }}</p>
                    </v-col>
                  </v-row>
                  <v-row align="center">
                    <v-col cols="2" xl="1"
                      ><v-icon style="font-size: 3em">mdi-map-marker</v-icon>
                    </v-col>
                    <v-col cols="10" xl="11">
                      <p>{{ selectedHotel.street }}</p>
                      <p>{{ selectedHotel.city }}</p>
                      <p>{{ selectedHotel.country }}</p>
                    </v-col>
                  </v-row>
                  <v-row align="center">
                    <v-col cols="2" xl="1"
                      ><v-icon style="font-size: 3em">mdi-calendar</v-icon>
                    </v-col>
                    <v-col>
                      <time-range-overview space />
                    </v-col>
                  </v-row>
                </v-container>
              </v-card>
            </v-col>
          </v-row>
        </v-container>
      </v-row>
      <v-row align="center" justify="center">
        <v-col>
          <p class="text-h4 mt-7">Booking information</p>
          <p>Tell us something about you!</p>
        </v-col>
      </v-row>
      <v-row align="center" justify="center">
        <v-col>
          <guest-data-form-list />
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <v-btn
            color="secondary"
            size="x-large"
            class="py-3 mb-10 mt-3 px-5"
            :disabled="guestDataMissing"
            ><v-icon class="mr-3">mdi-home-edit-outline</v-icon>Book bindingly now</v-btn
          >
        </v-col>
      </v-row>
    </v-container>
  </base-view>
</template>

<script>
import { mapGetters, mapMutations, mapState } from "vuex";
import BaseView from "../components/BaseView.vue";
import GuestDataFormList from "../components/roomBooking/GuestDataFormList.vue";
import TimeRangeOverview from "../components/roomBooking/TimeRangeOverview.vue";
import RoomTypeResult from "../components/roomSearch/RoomTypeResult.vue";
export default {
  components: {
    RoomTypeResult,
    TimeRangeOverview,
    BaseView,
    GuestDataFormList,
  },
  name: "room-book-page",
  computed: {
    ...mapState("roomType", ["roomTypes", "apiFilters"]),
    ...mapState("booking", ["roomTypeId", "guests"]),
    ...mapGetters("roomType", ["selectedHotel"]),
    selectedRoomType() {
      return this.roomTypes.find((rt) => rt.id == this.roomTypeId);
    },
    guestDataMissing() {
      return this.guests.length != this.apiFilters.guests;
    },
  },
  methods: {
    ...mapMutations("booking", ["setBookingInformation"]),
  },
  created() {
    this.setBookingInformation({
      startDate: this.apiFilters.startDate,
      endDate: this.apiFilters.endDate,
      hotelId: this.apiFilters.hotelId,
    });
  },
};
</script>

<style>
#back-button {
  position: absolute;
}
</style>
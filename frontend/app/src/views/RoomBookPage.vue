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
            @click="book"
            color="secondary"
            size="x-large"
            class="py-3 mb-10 mt-3 px-5"
            :disabled="guestDataMissing"
            ><v-icon class="mr-3">mdi-home-edit-outline</v-icon>Book bindingly
            now</v-btn
          >
        </v-col>
      </v-row>
    </v-container>
    <v-dialog v-model="isBooking" persistent width="40%" min-width="600px">
      <v-card class="pa-4">
        <v-card-title class="text-center">
          <v-label class="text-h5">{{ progressTitle }}</v-label>
        </v-card-title>
        <v-card-text>
          <v-container class="pt-0">
            <v-row>
              <v-progress-linear
                model-value="100"
                :indeterminate="bookingIsLoading"
                :color="progressColor"
              ></v-progress-linear>
            </v-row>
            <v-row class="mt-4">
              <v-col>
                <div v-if="bookingFailed">
                  <p class="text-center">
                    Something went wrong while processing your request. <br />
                    Please try again in a few minutes.
                  </p>
                </div>
                <div v-else-if="bookingSuccessful">
                  <p class="text-center">
                    Congratulations! <br />
                    Your booking was successful. <br />
                    You will shortly receive a booking confirmation via e-mail
                    to
                    <span style="color: orange"
                      ><v-icon class="mr-1">mdi-email</v-icon
                      >{{ guests[0]?.email ?? "blabla@gmail.com" }}</span
                    >.<br />
                    We're looking forward to welcoming you at
                    <span style="color: orange">{{ selectedHotel.name }}</span
                    >.
                  </p>
                </div>
              </v-col>
            </v-row>
            <v-row>
              <v-col class="text-center">
                <v-btn color="#333" @click="close"> Close </v-btn>
              </v-col>
            </v-row>
          </v-container>
        </v-card-text>
      </v-card>
    </v-dialog>
  </base-view>
</template>

<script>
import { mapActions, mapGetters, mapMutations, mapState } from "vuex";
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
  data() {
    return {
      isBooking: false,
      bookingSuccessful: false,
      bookingFailed: false,
      bookingIsLoading: false
    };
  },
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
    progressColor() {
      if (this.bookingFailed) return "red";
      if (this.bookingSuccessful) return "green";
      return "default";
    },
    progressTitle() {
      if (this.bookingFailed) return "Booking failed";
      if (this.bookingSuccessful) return "Booking successful";
      return "Booking in progress...";
    },
  },
  methods: {
    ...mapMutations("booking", [
      "setBookingInformation",
      "clearBookingInformation",
    ]),
    ...mapActions("booking", ["submitBooking"]),
    async book() {
      this.isBooking = true;
      this.bookingIsLoading = true;
      try {
        await this.submitBooking();
        this.bookingSuccessful = true;
      } catch (e) {
        console.error(e);
        this.bookingSuccessful = false;
        this.bookingFailed = true;
      } finally {
        this.bookingIsLoading = false;
      }
    },
    close() {
      this.isBooking = false;
      if (this.bookingSuccessful) {
        this.clearBookingInformation();
        this.$router.push({ name: "welcome" });
      }
      this.bookingSuccessful = false;
      this.bookingFailed = false;
    },
  },
  // beforeRouteLeave(_, _2, next) {
  //   if (this.bookingSuccessful) return next();
  //   const answer = window.confirm(
  //     "Do you really want to cancel this booking?"
  //   );
  //   if (answer) {
  //     next();
  //   } else {
  //     next(false);
  //   }
  // },
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
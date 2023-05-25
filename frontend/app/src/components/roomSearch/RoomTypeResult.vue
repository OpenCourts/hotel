<template>
  <v-card
    :style="`background: linear-gradient(
      to right,
      rgba(${bgCol}, ${bgCol / 3 + 20}, 30, 1),
      rgba(${bgCol}, ${bgCol / 3 + 20}, 30, 1),
      rgba(${bgCol}, ${bgCol / 3 + 20}, 30, 1),
      rgba(${bgCol}, ${bgCol / 3 + 20}, 30, 0.8),
      rgba(${bgCol}, ${bgCol / 3 + 20}, 30, 0)
    ),
    url(${imageUrl})
      center center / cover no-repeat;
    transition: all 0.1s;`"
  >
    <slot name="overlay" />
    <v-container>
      <v-row dense align="center" justify="center">
        <v-col>
          <span class="text-h4">
            {{ roomType.name }}
          </span>
        </v-col>
      </v-row>
      <v-row dense>
        <v-col
          ><v-icon class="mr-2">mdi-account</v-icon
          >{{ roomType.capacity }} persons capacity
        </v-col>
      </v-row>
      <v-row dense>
        <v-col
          ><v-chip
            v-for="a in roomType.amenities"
            :key="a"
            color="secondary"
            class="mr-2"
            ><span class="text-button">{{ a }}</span></v-chip
          >
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <p style="width: 60%; opacity: 0.7">{{ roomType.description }}</p>
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <v-chip prepend-icon="mdi-currency-eur">
            <span justify="center" align="center" style="font-size: 1.5em">{{
              roomType.pricePerNight
            }}</span></v-chip
          >
          <v-label
            style="
              font-size: 0.8em;
              position: relative;
              top: 0.4em;
              left: 0.2em;
            "
          >
            / night
          </v-label>
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <v-label :style="`color: ${availabeRoomsTextColor}`"
            ><v-icon class="mr-2">{{ availabeRoomsIcon }}</v-icon
            >{{ availableRoomsText }}</v-label
          >
        </v-col>
      </v-row>
      <div style="position: absolute; top: 0; right: 0">
        <v-col>
          <v-chip
            prepend-icon="mdi-currency-eur"
            size="x-large"
            color="white"
            style="background-color: rgba(0, 0, 0, 0.8); pointer-events: none"
          >
            <span style="font-size: 1.8em">{{ priceTotal }}</span></v-chip
          >
        </v-col>
      </div>
    </v-container>
  </v-card>
</template>

<script>
import { mapGetters } from "vuex";
export default {
  name: "room-type-result",
  emits: ["goToBooking"],
  props: {
    roomType: {
      type: Object,
      required: true,
    },
  },
  data() {
    return {
      warningLimit: 1,
    };
  },
  computed: {
    ...mapGetters("roomType", ["dateRange"]),
    imageUrl() {
      return (
        this.roomType.image ??
        "https://www.parkhotelleipzig.de/wp-content/uploads/2016/07/park-hotel-fassade-jugendstil.jpg"
      );
    },
    priceTotal() {
      return this.roomType.pricePerNight * this.dateRange;
    },
    availableRoomsText() {
      if (this.roomType.availableRooms == 0) {
        return "No more rooms available for your selected period";
      }
      return `${
        this.roomType.availableRooms <= this.warningLimit ? "Only" : ""
      } ${this.roomType.availableRooms} room${
        this.roomType.availableRooms == 1 ? "" : "s"
      } vailable for your selected period`;
    },
    availabeRoomsTextColor() {
      if (this.roomType.availableRooms == 0) return "red";
      return this.roomType.availableRooms <= this.warningLimit
        ? "orange"
        : "inherit";
    },
    availabeRoomsIcon() {
      if (this.roomType.availableRooms == 0) return "mdi-cross";
      return this.roomType.availableRooms <= this.warningLimit
        ? "mdi-alert"
        : "mdi-check";
    },
    bgCol() {
      return this.roomType.availableRooms > 0 &&
        this.roomType.availableRooms <= this.warningLimit
        ? 45
        : 30;
    },
  },
};
</script>

<style>
</style>
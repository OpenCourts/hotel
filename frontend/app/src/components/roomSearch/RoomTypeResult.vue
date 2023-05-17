<template>
  <v-hover>
    <template v-slot:default="{ isHovering, props }">
      <v-card v-bind="props" class="roomType" @click="goToBooking">
        <v-overlay
          :model-value="isHovering"
          contained
          scrim="black"
          class="align-center justify-end"
        >
          <v-btn @click="goToBooking" variant="outlined" class="mr-5"
            >Book now</v-btn
          >
        </v-overlay>
        <v-container>
          <v-row dense>
            <v-col>
              <p class="text-h5">{{ roomType.name }}</p>
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
                <span
                  justify="center"
                  align="center"
                  size="large"
                  style="font-size: 1.8em"
                  >{{ roomType.pricePerNight }}</span
                ></v-chip
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
        </v-container>
      </v-card>
    </template>
  </v-hover>
</template>

<script>
export default {
  name: "room-type-result",
  emits: ["goToBooking"],
  props: {
    roomType: {
      type: Object,
      required: true,
    },
  },
  methods: {
    goToBooking() {
      this.$emit("goToBooking", this.roomType.id);
    },
  },
};
</script>

<style>
.roomType {
  background: linear-gradient(
      to right,
      rgba(30, 30, 30, 1),
      rgba(30, 30, 30, 1),
      rgba(30, 30, 30, 1),
      rgba(30, 30, 30, 0.8),
      rgba(30, 30, 30, 0)
    ),
    url(https://www.parkhotelleipzig.de/wp-content/uploads/2016/07/park-hotel-fassade-jugendstil.jpg)
      center center / cover no-repeat;
  transition: all 0.1s;
}
</style>
<template>
  <v-app-bar
    class="py-2"
    color="#444"
    style="
      background: linear-gradient(to right, rgba(0, 0, 0, 0.1), rgba(0, 0, 0, 0.8)),
        url(https://www.parkhotelleipzig.de/wp-content/uploads/2016/07/park-hotel-fassade-jugendstil.jpg)
          center center / cover no-repeat;
    "
  >
    <v-container>
      <v-row no-gutters class="mt-2">
        <v-col cols="2" class="text-center">
          <p class="text-h5">H O T E L S</p>
        </v-col>
      </v-row>
      <v-row no-gutters class="mb-2">
        <v-col cols="2" class="text-center">
          <p class="mx-3">
            <v-icon>mdi-star</v-icon><v-icon>mdi-star</v-icon
            ><v-icon>mdi-star</v-icon><v-icon>mdi-star</v-icon
            ><v-icon>mdi-star</v-icon><v-icon>mdi-star</v-icon>
          </p>
        </v-col>
      </v-row>
    </v-container>
    <the-navigation-bar />
  </v-app-bar>
</template>

<script>
import TheNavigationBar from "./TheNavigationBar.vue";
export default {
  components: { TheNavigationBar },
  name: "the-header",
};
</script>
<style>
</style>
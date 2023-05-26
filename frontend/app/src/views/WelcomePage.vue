<template>
  <base-view>
    <image-carousel />
    <v-container class="mb-10">
      <v-row no-gutters>
        <v-col class="text-center">
          <p
            class="welcome-text"
            style="letter-spacing: 13px; padding-left: 13px; top: 25px"
          >
            SIX STAR
          </p>
        </v-col>
      </v-row>
      <v-row no-gutters>
        <v-col class="text-center">
          <v-rating :length="6" size="large" readonly :model-value="6" />
        </v-col>
      </v-row>
      <v-row no-gutters>
        <v-col class="text-center">
          <p
            class="welcome-text"
            style="letter-spacing: 23px; padding-left: 23px; bottom: 30px"
          >
            HOTELS
          </p>
        </v-col>
      </v-row>
      <v-row>
        <p class="text-justify paragraph">
          Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam
          nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat,
          sed diam voluptua. At vero eos et accusam et justo duo dolores et ea
          rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem
          ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur
          sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et
          dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam
          et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea
          takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit
          amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor
          invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.
        </p>
      </v-row>
      <v-row>
        <v-col class="text-center my-4">
          <v-btn color="#862" :to="{ name: 'roomSearch' }" size="x-large">Book now! </v-btn>
        </v-col>
      </v-row>
      <v-row>
        <p class="text-justify paragraph">
          At vero eos et accusam et justo duo dolores et ea rebum. Stet clita
          kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit
          amet. Duis autem vel eum iriure dolor in hendrerit in vulputate velit
          esse molestie consequat, vel illum dolore eu feugiat nulla facilisis
          at vero eros et accumsan et iusto odio dignissim qui blandit praesent
          luptatum zzril delenit augue duis dolore te feugait nulla facilisi.
          Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam
          nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat
          volutpat. At vero eos et accusam et justo duo dolores et ea rebum.
          Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum
          dolor sit amet. Duis autem vel eum iriure dolor in hendrerit in
          vulputate velit esse molestie consequat, vel illum dolore eu feugiat
          nulla facilisis at vero eros et accumsan et iusto odio dignissim qui
          blandit praesent luptatum zzril delenit augue duis dolore te feugait
          nulla facilisi. Lorem ipsum dolor sit amet, consectetuer adipiscing
          elit, sed diam nonummy nibh euismod tincidunt ut laoreet dolore magna
          aliquam erat volutpat.
        </p>
      </v-row>
    </v-container>
  </base-view>
</template>

<script>
import BaseView from "../components/BaseView.vue";
import ImageCarousel from "../components/welcome/ImageCarousel.vue";
export default {
  components: { BaseView, ImageCarousel },
  name: "welcome-page",
};
</script>

<style>
.welcome-text {
  font-family: "Lucida Sans", "Lucida Sans Regular", "Lucida Grande",
    "Lucida Sans Unicode", Geneva, Verdana, sans-serif;
  font-size: 3em;
  position: relative;
}

.paragraph {
    font-size: 1.4em
}
</style>
<template>
  <Flicking :options="{ circular: true }" :plugins="plugins">
    <div class="card-panel" v-for="(image, i) in images" :key="i">
      <div class="img-wrapper">
        <img class="img" :src="image" draggable="false" />
      </div>
    </div>
  </Flicking>
</template>

<script>
import Flicking from "@egjs/vue3-flicking";
import { AutoPlay, Fade } from "@egjs/flicking-plugins";
import "@egjs/vue3-flicking/dist/flicking.css";
import "@egjs/flicking-plugins/dist/flicking-plugins.css";
import "@egjs/flicking-plugins/dist/arrow.css";

const plugins = [
  new AutoPlay({
    duration: 2000,
    direction: "NEXT",
    stopOnHover: false,
    delayAfterHover: true,
  }),
  new Fade(),
];

export default {
  components: {
    Flicking,
  },
  name: "image-carousel",
  data() {
    return {
      images: [
        "https://pix10.agoda.net/hotelImages/124/1246280/1246280_16061017110043391702.jpg?ca=6&ce=1&s=1024x768",
        "https://media-cdn.tripadvisor.com/media/photo-s/29/0c/e6/3a/entrance-hotel-artemide.jpg",
        "https://cf.bstatic.com/xdata/images/xphoto/540x405/12097111.jpg?k=ab8d257011c475ed67db40f6ad3dc6339a0ca882f606e5585365ab16411b00d1&o=",
        "https://www.lopesan.com/img/hotels/5867/600-1000-imagen-emblematica-hotel-corallium-beach-by-lopesan-hotels-san-agustin-gran-canaria.jpg",
        "https://www.klosterhof.de/_Resources/Persistent/6985d76f35063ff61c45c3ed21e252c34e176500/klosterhof-aussenansicht-nacht-terasse-neubau-1920x1000.jpg",
        "https://www.umweltzeichen-hotels.at/wp-content/uploads/2023/12/103-0861-LR_kirchheimerhof_look%C2%A9gert_perauer.jpg",
      ],
      plugins: plugins,
    };
  },
};
</script>

<style scoped>
.img-wrapper img {
  height: 20em;
  margin: 0 1px;
  object-fit: cover;
  width: 30vw;
}

.img {
  transform: skewX(10deg) !important;
}

.card-panel {
  transform: skewX(-10deg) !important;
  width: 30vw;
}

/* body {
  height: 100vh;
  width: 100vw;
  background-color: black;
  margin: 0rem;
  overflow: hidden;
} */

/* #image-track {
  display: flex;
  gap: 4vmin;
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(0%, -50%);
  user-select: none; /* -- Prevent image highlighting --
} */

/* #image-track > .image {
  width: 40vmin;
  height: 56vmin;
  object-fit: cover;
  object-position: 100% center;
} */
</style>
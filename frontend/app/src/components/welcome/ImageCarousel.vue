<template>
  <Flicking :options="{ circular: true }" :plugins="plugins">
    <div class="card-panel" v-for="(image, i) in images" :key="i">
      <img :src="image" draggable="false" />
    </div>
  </Flicking>
</template>

<script>
import Flicking from "@egjs/vue3-flicking";
import { AutoPlay, Parallax } from "@egjs/flicking-plugins";
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
  new Parallax("img", 1),
];

export default {
  components: {
    Flicking,
  },
  name: "image-carousel",
  data() {
    return {
      images: [
        "https://images.unsplash.com/photo-1524781289445-ddf8f5695861?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1770&q=80",
        "https://img.welt.de/img/vermischtes/bilder-des-tages/mobile243173239/9812507287-ci102l-w1024/Knowsley-Safari-recycelt-alte-Weihnachtsbaeume.jpg",
        "https://assets.pixolum.com/blog/wp-content/uploads/2016/10/focus-stacking-scharfe-bilder-1.jpg",
        "https://picsum.photos/id/41/600/400/",
        "https://images.unsplash.com/photo-1524781289445-ddf8f5695861?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1770&q=80",
        "https://img.welt.de/img/vermischtes/bilder-des-tages/mobile243173239/9812507287-ci102l-w1024/Knowsley-Safari-recycelt-alte-Weihnachtsbaeume.jpg",
        "https://assets.pixolum.com/blog/wp-content/uploads/2016/10/focus-stacking-scharfe-bilder-1.jpg",
        "https://picsum.photos/id/41/600/400/",
      ],
      plugins: plugins,
    };
  },
};
</script>

<style scoped>
img {
  height: 20em;
  margin: 0 1px;
  object-fit: cover;
  min-width: 20vw;
}

.card-panel {
  width: 20vw;
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
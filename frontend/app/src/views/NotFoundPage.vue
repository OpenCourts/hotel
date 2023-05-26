<template>
  <base-view>
    <div>
      <v-container
        @keydown.enter="goDown"
        @keydown.up="goUp"
        @keyup="stop"
        class="d-none d-lg-block"
      >
        <v-row align="center" justify="center">
          <v-col cols="5" align="right">
            <router-link :to="{ path: '/' }">
              <v-img :src="logo" width="50%" class="mt-8"></v-img>
            </router-link>
          </v-col>
          <v-divider vertical class="mt-7 mb-4 mx-5" />
          <v-col cols="5"> <v-text class="text-h1">404</v-text><br /> </v-col>
        </v-row>
        <v-row>
          <v-col class="text-center">
            <v-text>The requested page does not exist.</v-text></v-col
          >
        </v-row>
        <v-row>
          <v-col class="text-center">
            <v-btn :to="{ path: '/' }" replace
              ><v-icon class="mr-2">mdi-open-in-new</v-icon>Homepage</v-btn
            >
          </v-col>
        </v-row>
        <v-row class="mt-8">
          <v-col class="text-center">
            <v-text class="key-text">W</v-text
            ><v-text class="text-button">UP</v-text
            ><v-divider vertical class="mx-5" /><v-text class="key-text"
              >S</v-text
            ><v-text class="text-button"> DOWN </v-text></v-col
          >
        </v-row>
        <v-row
          align="center"
          justify="center"
          align-content="center"
          class="d-none d-md-block mt-3 mb-0 pa-0"
        >
          <v-col class="text-center" align="center" justify="center">
            <div id="pongWrapper" style="margin: auto">
              <div class="align-center">
                <div id="court">
                  <span class="scoreSpanLeft">
                    {{ playerScore.toString().padStart(2, "0") }} </span
                  ><v-divider
                    :vertical="true"
                    :thickness="5"
                    color="black"
                  /><span class="scoreSpanRight">
                    {{ enemyScore.toString().padStart(2, "0") }}
                  </span>
                  <div
                    class="player"
                    id="player1"
                    ref="player"
                    :style="`top: ${yPlayer / 10}vh`"
                  ></div>
                  <div
                    class="ball"
                    :style="`top: ${yBall / 10}vh; left: ${xBall / 12.5}vw`"
                  ></div>
                  <div
                    class="player"
                    id="player2"
                    :style="`top: ${yEnemy / 10}vh`"
                  ></div>
                </div>
              </div>
            </div>
          </v-col>
        </v-row>
      </v-container>

      <!-- mobile -->
      <v-container class="d-block d-lg-none">
        <v-row align="end" justify="center">
          <v-col cols="12">
            <router-link :to="{ path: '/' }">
              <v-img :src="logo" width="50%" class="mt-8"></v-img>
            </router-link>
          </v-col>
          <v-col cols="12" class="text-center">
            <v-text class="text-h1">404</v-text><br />
          </v-col>
        </v-row>
        <v-divider class="mt-3 mb-5 mx-5" />
        <v-row>
          <v-col class="text-center">
            <v-text>The requested page does not exist.</v-text></v-col
          >
        </v-row>
        <v-row>
          <v-col class="text-center">
            <v-btn :to="{ path: '/' }" replace
              ><v-icon class="mr-2">mdi-open-in-new</v-icon>Homepage</v-btn
            >
          </v-col>
        </v-row>
      </v-container>
    </div>
  </base-view>
</template>

<script>
import BaseView from "../components/BaseView.vue";
export default {
  components: { BaseView },
  name: "not-found-page",
  data() {
    return {
      direction: 0,
      yPlayer: 0,
      yEnemy: 0,
      xBall: 500,
      yBall: 250,
      ballMomentum: {
        x: 5,
        y: 5,
      },
      playerScore: 0,
      enemyScore: 0,
    };
  },
  computed: {
    logo() {
      return "https://distributed.opencourts.net/images/logo.png";
    },
  },
  methods: {
    goUp() {
      this.direction = 5;
    },
    goDown() {
      this.direction = -5;
    },
    stop() {
      this.direction = 0;
    },
    move() {
      {
        // move player and ball
        this.yPlayer += this.direction;
        this.xBall += this.ballMomentum.x;
        this.yBall += this.ballMomentum.y;

        // move enemy
        if (this.yBall - 20 > this.yEnemy && this.ballMomentum.x > 0) {
          this.yEnemy += Math.min(5, this.yBall - 20 - this.yEnemy);
        }
        if (this.yBall - 20 < this.yEnemy && this.ballMomentum.x > 0) {
          this.yEnemy -= Math.min(5, this.yEnemy - this.yBall + 20);
        }

        // player cannot move towards edges
        if (this.yPlayer < 0) this.yPlayer = 0;
        if (this.yPlayer > 440) this.yPlayer = 440;

        if (this.yEnemy < 0) this.yEnemy = 0;
        if (this.yEnemy > 440) this.yEnemy = 440;

        // ball bounces
        if (this.yBall < 0) {
          this.yBall = 0;
          this.ballMomentum.y *= -1;
        }
        if (this.yBall > 480) {
          this.yBall = 480;
          this.ballMomentum.y *= -1;
        }

        // player hits ball
        if (
          this.xBall <= 30 &&
          this.xBall > 25 &&
          this.yPlayer >= this.yBall - 50 &&
          this.yPlayer <= this.yBall
        ) {
          this.xBall = 30;
          this.ballMomentum.x = Math.abs(this.ballMomentum.x);
          this.ballMomentum.y += parseInt(Math.random() * 10) - 5;
        }

        // enemy hits ball
        if (
          this.xBall <= 950 &&
          this.xBall > 945 &&
          this.yEnemy >= this.yBall - 50 &&
          this.yEnemy <= this.yBall
        ) {
          this.xBall = 945;
          this.ballMomentum.x *= -1;
          this.ballMomentum.y += parseInt(Math.random() * 10) - 5;
          this.ballMomentum.y = Math.min(this.ballMomentum.y, 6);
          this.ballMomentum.y = Math.max(this.ballMomentum.y, -6);
        }

        // enemy scores
        if (this.xBall < 0) {
          this.enemyScore++;
          this.xBall = 500;
          this.yBall = 250;
          this.ballMomentum = {
            x: 5,
            y: 5,
          };
        }

        // player scores
        if (this.xBall > 950) {
          this.playerScore++;
          this.xBall = 500;
          this.yBall = 250;
          this.ballMomentum = {
            x: -5,
            y: 5,
          };
          this.ballMomentum.x *= -1;
          this.ballMomentum.y += parseInt(Math.random() * 10) - 5;
          this.ballMomentum.y = Math.min(this.ballMomentum.y, 6);
          this.ballMomentum.y = Math.max(this.ballMomentum.y, -6);
        }
        setTimeout(this.move, 15);
      }
    },
  },
  created() {
    window.addEventListener("keypress", (e) => {
      switch (e.keyCode) {
        case 115:
          this.goUp();
          break;
        case 119:
          this.goDown();
          break;
      }
    });
    window.addEventListener("keyup", () => {
      this.stop();
    });
    this.move();
  },
};
</script>

<style scoped>
#court {
  width: 80vw;
  height: 50vh;
  opacity: 0.3;
  border: 5px solid black;
  background-color: gray;
  font-size: 30vh;
  color: #999;
  text-align: center;
  position: fixed;
  right: 10vw;
}

.player {
  width: 0.8vw;
  height: 5vh;
  background-color: white;
  position: absolute;
}

#player1 {
  left: 2vw;
  top: 0;
}

#player2 {
  right: 2vw;
  top: 0;
}

.ball {
  position: absolute;
  background-color: white;
  height: 1vh;
  width: 0.8vw;
}

.scoreSpanLeft {
  position: relative;
  bottom: 6vh;
  right: 8vw;
}

.scoreSpanRight {
  position: relative;
  bottom: 6vh;
  left: 8vw;
}

#pongWrapper {
  position: absolute;
  width: inherit;
}

.key-text {
  background-color: gray;
  padding: 8px 12px;
  border: 2px solid darkgray;
  border-radius: 5px;
  font-family: monospace;
  margin-right: 5px;
}
</style>
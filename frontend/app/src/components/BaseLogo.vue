<template>
  <v-container>
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
  </v-container>
</template>

<script>
export default {
  name: "base-logo",
};
</script>

<style>
</style>
<template>
  <v-text-field
    readonly
    :model-value="numberOfPersonsString"
    variant="outlined"
    class="pointer"
  >
    <template #prepend-inner>
      <v-icon class="mr-2"> mdi-account-multiple </v-icon>
    </template>
    <v-menu activator="parent" :close-on-content-click="false">
      <v-list>
        <v-list-item>
          <v-row justify="center" align="center">
            <v-col cols="4" class="text-overline text-right"> Adults </v-col>
            <v-col>
              <v-btn-group elevation="2">
                <v-btn size="small"
                  :disabled="valuePersons == minPersons || valueAdults == 0"
                  @click="valueAdults--"
                  ><v-icon>mdi-minus</v-icon></v-btn
                >
                <v-label class="px-5">{{ valueAdults }}</v-label>
                <v-btn size="small"
                  :disabled="valuePersons == maxPersons"
                  @click="valueAdults++"
                  ><v-icon>mdi-plus</v-icon></v-btn
                >
              </v-btn-group>
            </v-col>
          </v-row>
        </v-list-item>
        <v-list-item>
          <v-row justify="center" align="center">
            <v-col cols="4" class="text-overline text-right"> Children </v-col>
            <v-col>
              <v-btn-group elevation="2">
                <v-btn size="small"
                  :disabled="valuePersons == minPersons || valueChildren == 0"
                  @click="valueChildren--"
                  ><v-icon>mdi-minus</v-icon></v-btn
                >
                <v-label class="px-5">{{ valueChildren }}</v-label>
                <v-btn size="small"
                  :disabled="valuePersons == maxPersons"
                  @click="valueChildren++"
                  ><v-icon>mdi-plus</v-icon></v-btn
                >
              </v-btn-group>
            </v-col>
          </v-row>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-text-field>
</template>

<script>
export default {
  name: "number-of-persons-picker",
  data() {
    return {
      minPersons: 1,
      maxPersons: 8,
      valueAdults: 1,
      valueChildren: 0,
      items: ["Adults", "Children"],
    };
  },
  computed: {
    valuePersons() {
      return this.valueAdults + this.valueChildren;
    },
    numberOfPersonsString() {
      return `${this.valuePersons} person${
        this.valuePersons == 1 ? "" : "s"
      } (${this.valueAdults} adult${this.valueAdults == 1 ? "" : "s"}, ${
        this.valueChildren
      } child${this.valueChildren == 1 ? "" : "ren"})`;
    },
  },
  methods: {
    getHotelSearchString(hotel) {
      return `${hotel.location}, ${hotel.name}`;
    },
  },
};
</script>

<style>
.pointer, .pointer * {
  cursor: pointer !important;
}
</style>
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
    <v-menu
      activator="parent"
      :close-on-content-click="false"
    >
      <v-list>
        <v-list-item>
          <v-row justify="center" align="center">
            <v-col cols="4" class="text-overline text-right"> Adults </v-col>
            <v-col>
              <v-btn-group elevation="2">
                <v-btn
                  size="small"
                  :disabled="valuePersons == minPersons || valueAdults == 0"
                  @click="valueAdults--"
                  ><v-icon>mdi-minus</v-icon></v-btn
                >
                <v-label class="px-5">{{ valueAdults }}</v-label>
                <v-btn
                  size="small"
                  :disabled="valuePersons == maxPersons"
                  @click="valueAdults++"
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
import { mapMutations, mapState } from "vuex";
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
    ...mapState("roomType", ["apiFilters"]),
    valuePersons() {
      return this.valueAdults + this.valueChildren;
    },
    numberOfPersonsString() {
      return `${this.valuePersons} person${this.valuePersons == 1 ? "" : "s"}`;
    },
  },
  watch: {
    valuePersons: {
      handler: function () {
        this.updateGuests();
      },
    },
  },
  methods: {
    ...mapMutations("roomType", ["setApiFilters"]),
    updateGuests() {
      this.setApiFilters({ guests: this.valuePersons });
    },
  },
  created() {
    this.valueAdults = this.apiFilters.guests ?? 1;
    this.updateGuests();
  },
};
</script>

<style>
.pointer,
.pointer * {
  cursor: pointer !important;
}
</style>
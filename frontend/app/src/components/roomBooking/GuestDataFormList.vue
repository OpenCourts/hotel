<template>
  <v-expansion-panels v-model="panels">
    <v-expansion-panel
      v-for="i in [...Array(apiFilters.guests).keys()]"
      :key="i"
    >
      <v-expansion-panel-title :style="`background-color: ${getColor(i)}`"
        ><v-icon class="my-1 mr-3">{{ getIcon(i) }}</v-icon> {{ getName(i)
        }}<v-btn
          v-if="i == 0"
          icon
          plain
          fab
          elevation="0"
          :ripple="false"
          color="transparent"
          class="ml-1"
          ><v-tooltip activator="parent" location="end"
            >This guest is the main guest and will receive a booking
            confirmation.</v-tooltip
          ><v-icon>mdi-crown </v-icon>
        </v-btn>
      </v-expansion-panel-title>
      <v-expansion-panel-text>
        <guest-data-form
          :guestData="guestData[i]"
          :guestNumber="i"
          @input="updateGuest"
        />
      </v-expansion-panel-text>
    </v-expansion-panel>
  </v-expansion-panels>
</template>

<script>
import { mapMutations, mapState } from "vuex";
import GuestDataForm from "./GuestDataForm.vue";
export default {
  components: { GuestDataForm },
  name: "guest-data-form-list",
  data() {
    return {
      panels: 0,
      guestData: [],
      valid: [],
    };
  },
  computed: {
    ...mapState("roomType", ["apiFilters"]),
    ...mapState("booking", ["guests"])
  },
  methods: {
    ...mapMutations("booking", ["setBookingInformation"]),
    updateGuest({ guestData, guestNumber, valid }) {
      this.guestData[guestNumber] = guestData;
      this.valid[guestNumber] = valid;
      if (this.valid.filter((v) => !v).length == 0) {
        this.setBookingInformation({ guests: this.guestData });
      }
    },
    getIcon(guestNumber) {
      return this.valid[guestNumber] ? "mdi-check" : "mdi-pencil";
    },
    getName(guestNumber) {
      return this.valid[guestNumber]
        ? `${this.guestData[guestNumber].firstName} ${this.guestData[guestNumber].lastName}`
        : `Guest ${guestNumber + 1}`;
    },
    getColor(guestNumber) {
      return this.valid[guestNumber] ? "#020" : "inherit";
    },
  },
  created() {
    this.guestData = this.guests
    this.valid = [...Array(this.apiFilters.guests)];
  },
};
</script>
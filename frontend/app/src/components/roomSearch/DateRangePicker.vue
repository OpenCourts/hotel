<template>
  <VueDatePicker
    v-model="dates"
    multi-calendars
    range
    dark
    :enable-time-picker="false"
  />
</template>

<script>
import VueDatePicker from "@vuepic/vue-datepicker";
import "@vuepic/vue-datepicker/dist/main.css";

export default {
  components: { VueDatePicker },
  name: "date-range-picker",
  data() {
    return {
      dates: { value: [] },
    };
  },
  methods: {
    activatePicker() {
      this.$refs.datepicker.$el.click();
    },
  },
};
</script>

<style>
.noEvent {
  pointer-events: none;
  z-index: 10;
}

.dp__input {
  background-color: inherit;
  border-color: #777;
  height: 100%;
  line-height: 2.6em;
}

.dp__input:hover {
  border-color: #fff;
}
</style>
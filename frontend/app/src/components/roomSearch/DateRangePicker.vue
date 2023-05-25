<template>
  <VueDatePicker
    v-model="dates"
    multi-calendars
    range
    dark
    :enable-time-picker="false"
    :min-date="minDate"
    model-type="yyyy-MM-dd"
    auto-apply 
    :close-on-auto-apply="false"
    disable-year-select 
    placeholder="When are you travelling?"
  />
</template>

<script>
import VueDatePicker from "@vuepic/vue-datepicker";
import "@vuepic/vue-datepicker/dist/main.css";
import { mapMutations, mapState } from "vuex";

export default {
  components: { VueDatePicker },
  name: "date-range-picker",
  data() {
    return {
      dates: { value: [] },
      minDate: new Date(),
    };
  },
  computed: {
    ...mapState("roomType", ["apiFilters"]),
  },
  watch: {
    dates: {
      deep: true,
      handler: function (newDates) {
        this.updateDates(newDates);
      },
    },
  },
  methods: {
    ...mapMutations("roomType", ["setApiFilters"]),
    activatePicker() {
      this.$refs.datepicker.$el.click();
    },
    updateDates() {
      const newDates =
        this.dates == null || this.dates.length != 2
          ? { startDate: null, endDate: null }
          : { startDate: this.dates[0], endDate: this.dates[1] };
      this.setApiFilters(newDates);
    },
  },
  created() {
    if (!this.apiFilters.startDate || !this.apiFilters.endDate) return;
    this.dates = [this.apiFilters.startDate, this.apiFilters.endDate];
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

.dateError {
  border-color: red;
}
</style>
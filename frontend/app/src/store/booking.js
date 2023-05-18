import { makeJsonRequest } from "@/helpers/requestHelper"

const bookingModule = {
    namespaced: true,

    state: {
        startDate: null,
        endDate: null,
        guests: [],
        roomTypeId: null,
    },

    getters: {
        roomType(state, rootState) {
            return rootState.roomTypes.find(rt => rt.id == state.roomTypeId)
        },
    },

    mutations: {
        setBookingInformation(state, bookingInformation) {
            state.startDate = bookingInformation.startDate ? bookingInformation.startDate : state.startDate
            state.endDate = bookingInformation.endDate ? bookingInformation.endDate : state.endDate
            state.guests = bookingInformation.guests ? bookingInformation.guests : state.guests
            state.roomTypeId = bookingInformation.roomTypeId ? bookingInformation.roomTypeId : state.roomTypeId
        },
    },

    actions: {
        async sendBooking() {
            makeJsonRequest("/book", "POST");
        }
    }
}

export default bookingModule
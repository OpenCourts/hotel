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
        setBooking(state, booking) {
            state.startDate = booking.startDate
            state.endDate = booking.endDate
            state.guests = booking.guests
            state.roomTypeId = booking.roomTypeId
        },
    },

    actions: {
        async sendBooking() {
            makeJsonRequest("/book", "POST");
        }
    }
}

export default bookingModule
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
            state.startDate = bookingInformation.startDate ?? state.startDate
            state.endDate = bookingInformation.endDate ?? state.endDate
            state.guests = bookingInformation.guests ?? state.guests
            state.roomTypeId = bookingInformation.roomTypeId ?? state.roomTypeId
            state.hotelId = bookingInformation.hotelId ?? state.hotelId
        },
    },

    actions: {
        async sendBooking(state) {
            makeJsonRequest("/book", "POST", { from_date: state.startDate, end_date: state.endDate, guests: state.guests, room_type: state.roomTypeId });
        }
    }
}

export default bookingModule
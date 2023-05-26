

const bookingModule = {
    namespaced: true,

    state: {
        startDate: null,
        endDate: null,
        guests: [],
        roomTypeId: null,
        hotelId: null
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
        clearBookingInformation(state) {
            state.startDate = null
            state.endDate = null
            state.guests = []
            state.roomTypeId = null
            state.hotelId = null
        }
    },

    actions: {
        async submitBooking({ state }) {
            const guestsForRequest = []
            for (const g of state.guests) {
                guestsForRequest.push({
                    name: `${g.firstName} ${g.lastName}`,
                    email: g.email,
                    phone_number: g.phoneNumber,
                    address: g.address,
                    passport_number: g.passportNumber
                })
            }
            const request = {
                guests: guestsForRequest,
                room_type_id: state.roomTypeId,
                check_in_date: state.startDate,
                check_out_date: state.endDate,
                hotel_id: state.hotelId
            }
            console.log(request)
            const response = await fetch(process.env.VUE_APP_HOTELS_API_LOCATION + "/booking", { method: "POST", body: JSON.stringify(request) })
            console.log(response)
        }
    }
}

export default bookingModule
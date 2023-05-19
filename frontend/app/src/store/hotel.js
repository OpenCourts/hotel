import { makeJsonRequest } from "@/helpers/requestHelper"

const hotelModule = {
    namespaced: true,

    state: {
        hotels: [],
        hotel: null
    },

    getters: {

    },

    mutations: {
        setHotels(state, hotels) {
            state.hotels = hotels
        },
        setHotel(state, hotel) {
            state.hotel = hotel
        },
    },

    actions: {
        async loadHotels({ commit }) {
            const hotels = await makeJsonRequest("/hotels", "GET");
            commit("setHotels", hotels ?? [])
        }
    }
}

export default hotelModule
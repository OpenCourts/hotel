import { makeJsonRequest } from "@/helpers/requestHelper"

const hotelModule = {
    namespaced: true,

    state: {
        hotels: [
            {
                id: 1,
                location: "Berlin",
                picture: "https://images.unsplash.com/photo-1524781289445-ddf8f5695861?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1770&q=80"
            },
            {
                id: 2,
                location: "London",
                picture: "https://assets.pixolum.com/blog/wp-content/uploads/2016/10/focus-stacking-scharfe-bilder-1.jpg"
            },
            {
                id: 3,
                location: "Paris",
                picture: "https://picsum.photos/id/41/600/400/"
            },
        ],
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
            const hotels = makeJsonRequest("/hotels", "GET");
            commit("setHotels", hotels ?? [123, 456])
        }
    }
}

export default hotelModule
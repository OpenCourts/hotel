import { makeJsonRequest } from "@/helpers/requestHelper"

const roomTypeModule = {
    namespaced: true,

    state: {
        roomTypes: [],
    },

    getters: {
        availableTypes(state) {
            return state.roomTypes.filter(rt => rt.isAvailable)
        },
        nonAvailableTypes(state) {
            return state.roomTypes.filter(rt => !rt.isAvailable)
        },
    },

    mutations: {
        setRoomTypes(state, roomTypes) {
            state.roomTypes = roomTypes
        },
        setRoomType(state, hotel) {
            state.hotel = hotel
        },
    },

    actions: {
        async loadRoomTypes({ commit }) {
            const roomTypes = makeJsonRequest("/search", "GET");
            commit("setRoomTypes", roomTypes ?? [123, 456])
        }
    }
}

export default roomTypeModule
import { makeJsonRequest } from "@/helpers/requestHelper"

const roomTypeModule = {
    namespaced: true,

    state: {
        roomTypes: [
        ],
        activeAmenitiesFilters: [],
        apiFilters: {
            startDate: null,
            endDate: null,
            guests: null,
            hotelId: null
        }
    },

    getters: {
        availableTypes(state) {
            return state.roomTypes.filter(rt => rt.isAvailable)
        },
        nonAvailableTypes(state) {
            return state.roomTypes.filter(rt => !rt.isAvailable)
        },
        availableFilters(state) {
            if (!state.roomTypes) return []
            const roomTypeAmenities = state.roomTypes.map(rt => rt.amenities).flat(1)
            return [...new Set(roomTypeAmenities)]
        },
        filteredRoomTypes(state) {
            return state.roomTypes.filter(rt => {
                for (const activeFilter of state.activeAmenitiesFilters) {
                    if (!rt.amenities.includes(activeFilter)) { return false }
                }
                return true
            })
        }
    },

    mutations: {
        setRoomTypes(state, roomTypes) {
            state.roomTypes = roomTypes
        },
        setRoomType(state, roomType) {
            state.roomType = roomType
        },
        setActiveAmenitiesFilters(state, activeAmenitiesFilters) {
            state.activeAmenitiesFilters = activeAmenitiesFilters
        },
        setApiFilters(state, filters) {
            state.apiFilters.startDate = filters.startDate ?? state.apiFilters.startDate
            state.apiFilters.endDate = filters.endDate ?? state.apiFilters.endDate
            state.apiFilters.guests = filters.guests ?? state.apiFilters.guests
            state.apiFilters.hotelId = filters.hotelId ?? state.apiFilters.hotelId
        }
    },

    actions: {
        async loadRoomTypes({ commit }) {
            // const roomTypes = makeJsonRequest("/search", "GET");
            await makeJsonRequest("/hotels", "GET")
            commit("setRoomTypes", [
                {
                    id: 1,
                    name: 'Comfort Apartment',
                    description: 'text text text text text text text text text text text text text text text text text text ',
                    size: 50,
                    capacity: 8,
                    amenities: ['tv', 'beach', 'whirlpool'],
                    pricePerNight: 59,
                    availableRooms: 12,
                    image: null
                },
                {
                    id: 2,
                    name: 'Comfort Apartment',
                    description: 'text text text text text text text text text text text text text text text text text text ',
                    size: 50,
                    capacity: 8,
                    amenities: ['tv', 'kitchen'],
                    pricePerNight: 59,
                    availableRooms: 12,
                    image: null
                }
            ])
        }
    }
}

export default roomTypeModule
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
            hotelId: null,
        },
        filters: {
            minPrice: null,
            maxPrice: null
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
                if (
                    (state.filters.maxPrice && rt.pricePerNight > state.filters.maxPrice)
                    || (state.filters.minPrice && rt.pricePerNight < state.filters.minPrice)
                ) {
                    return false
                }
                return true
            })
        },
        startDate(state) {
            const parts = state.apiFilters.startDate.split("-")
            return new Date(parts[0], parts[1] - 1, parts[2])
        },
        endDate(state) {
            const parts = state.apiFilters.endDate.split("-")
            return new Date(parts[0], parts[1] - 1, parts[2])
        },
        dateRange(_, getters) {
            const diff = getters.endDate - getters.startDate
            return Math.ceil(diff / (1000 * 60 * 60 * 24));
        },
        selectedHotel(state, _, rootState) {
            return rootState.hotel.hotels.find(h => h.id == state.apiFilters.hotelId)
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
        },
        setFilters(state, filters) {
            state.filters.minPrice = filters.minPrice ?? state.filters.minPrice
            state.filters.maxPrice = filters.maxPrice ?? state.filters.maxPrice
        }
    },

    actions: {
        async loadRoomTypes({ commit, state }) {
            const searchString = `/search?from_date=${state.apiFilters.startDate}&to_date=${state.apiFilters.endDate}&capacity=${state.apiFilters.guests}&hotel_id=${state.apiFilters.hotelId}`
            const roomTypes = await makeJsonRequest(searchString, "GET");
            roomTypes.forEach((rt) => {
                rt.pricePerNight = rt.price_per_night;
                rt.amenities = rt.amenities.split(", ");
                rt.availableRooms = rt.room_available_count ?? 2;
                rt.image = process.env.VUE_APP_HOTELS_SERVER_LOCATION + rt.image_url
            })
            const existingRoomTypes = roomTypes.filter(rt => rt.room_count > 0)
            commit("setRoomTypes", existingRoomTypes)
        },
    }
}

export default roomTypeModule
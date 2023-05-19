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
        },
        startDate(state) {
            const parts = state.apiFilters.startDate.split(".")
            return new Date(parts[0], parts[1] - 1, parts[2])
        },
        endDate(state) {
            const parts = state.apiFilters.endDate.split(".")
            return new Date(parts[0], parts[1] - 1, parts[2])
        },
        dateRange(_, getters) {
            const diff = getters.endDate - getters.startDate
            console.log(getters)
            console.log(diff)
            return Math.ceil(diff / (1000 * 60 * 60 * 24));
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
                    image: null,
                    rooms: 12
                },
                {
                    id: 2,
                    name: 'Luxus Apartment',
                    description: 'text text text text text text text text text text text text text text text text text text ',
                    size: 50,
                    capacity: 4,
                    amenities: ['tv', 'kitchen'],
                    pricePerNight: 219,
                    availableRooms: 12,
                    rooms: 4,
                    image: "https://cf.bstatic.com/xdata/images/hotel/max1024x768/301944448.jpg?k=9bb7ff454f4298bc2a036b8eda69cc6cc11c6de05d9ce4d92c664a281b83cec7&o=&hp=1"
                }
            ])
        }
    }
}

export default roomTypeModule
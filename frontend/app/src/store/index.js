import { createStore } from 'vuex'
import bookingModule from './booking'
import hotelModule from './hotel'
import roomTypeModule from './roomType'

export default createStore({
  state: {
  },
  getters: {
  },
  mutations: {
  },
  actions: {
  },
  modules: {
    hotel: hotelModule,
    booking: bookingModule,
    roomType: roomTypeModule
  }
})

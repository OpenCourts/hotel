import { createRouter, createWebHistory } from 'vue-router'
import WelcomePage from "@/views/WelcomePage"
import RoomSearchPage from "@/views/RoomSearchPage"
import RoomBookPage from "@/views/RoomBookPage"
import BookingSuccessPage from "@/views/BookingSuccessPage"
import NotFoundPage from "@/views/NotFoundPage"
import store from '@/store'

const routes = [
  {
    path: '/',
    name: 'welcome',
    component: WelcomePage
  },
  {
    path: '/search',
    name: 'roomSearch',
    component: RoomSearchPage
  },
  {
    path: '/book',
    name: 'roomBook',
    component: RoomBookPage,
    meta: { requiresRoomType: true }
  },
  {
    path: '/success',
    name: 'bookingSuccess',
    component: BookingSuccessPage
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'notFound',
    component: NotFoundPage
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

router.beforeEach((to, _, next) => {
  console.log(store.state.booking.roomTypeId)
  console.log(store.state["booking/roomTypeId"])
  if (to.meta.requiresRoomType) {
    if (!store.state.booking.roomTypeId) {
      next({ name: "roomSearch" })
      return
    }
    next()
    return
  }
  next()
})

export default router

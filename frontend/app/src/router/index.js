import { createRouter, createWebHistory } from 'vue-router'
import WelcomePage from "@/views/WelcomePage"
import RoomSearchPage from "@/views/RoomSearchPage"
import RoomBookPage from "@/views/RoomBookPage"
import BookingSuccessPage from "@/views/BookingSuccessPage"
import NotFoundPage from "@/views/NotFoundPage"

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
    component: RoomBookPage
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

export default router

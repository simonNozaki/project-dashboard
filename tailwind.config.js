const { resolve } = require('path')
const colorsPath = resolve(__dirname, 'src/styles/colors.ts')
const colors = require(colorsPath)

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./components/**/*.{js,vue,ts}",
    "./layouts/**/*.vue",
    "./pages/**/*.vue",
    "./plugins/**/*.{js,ts}",
    "./nuxt.config.{js,ts}",
    "./app.vue",
  ],
  theme: {
    extend: {
      colors
    },
  },
  plugins: [],
}


/** @type {import("tailwindcss").Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  plugins: [
    require("daisyui")
  ],
  daisyui: {
    themes: [{
      dark: {
        ...require("daisyui/src/theming/themes")["[data-theme=dark]"],
        "primary": "#9141ac"
      }
    }]
  },
}

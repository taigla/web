/** @type {import("tailwindcss").Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        accent: "#9141ac"
      }
    },
  },
  plugins: [
    require("@sira-ui/tailwind")({
      themes: [
        {
          name: "dark",
          colorScheme: "dark",
          prefersColorScheme: true,
          colors: {
            primary: "#9141ac",
          }
        },
        {
          name: "light",
          colorScheme: "light",
          colors: {
            primary: "#9141ac",
          }
        }
      ]
    })
  ],
}

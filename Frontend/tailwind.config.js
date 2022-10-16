const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./src/**/*.{html,js,rs,css}", "*.{html, js, rs, css}"],
  theme: {
    extend: {
      spacing: {
        "8xl": "96rem",
        "9xl": "128rem",
      },
      borderRadius: {
        "4xl": "2rem",
      },
    },
  },
};

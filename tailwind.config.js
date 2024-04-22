/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      backgroundImage: {
        wood: "url('https://s3-us-west-2.amazonaws.com/s.cdpn.io/225363/Melamine-wood-001.png')",
      },
    },
  },
  plugins: [],
};

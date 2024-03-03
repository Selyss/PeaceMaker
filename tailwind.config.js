/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,rs}"],
  theme: {
    extend: {
      backgroundImage: {
        wood: "url('https://s3-us-west-2.amazonaws.com/s.cdpn.io/225363/Melamine-wood-001.png')",
      },
    },
  },
  plugins: [],
};

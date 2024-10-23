const {nextui} = require('@nextui-org/theme');
module.exports = {
  darkMode: "class",
  plugins: [nextui()],
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    "./node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}"
  ],
  theme: {
    extend: {},
  },
};

/** @type {import('tailwindcss').Config} */
module.exports = {
  presets: [require('./theme/tailwind.preset.js')],
  content: [
    "./packages/**/*.rs",
    "./playground/**/*.rs",
    "./playground/index.html",
  ],
  plugins: [],
};

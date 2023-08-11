/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      colors: {
        heiwa: {
          50: '#FFFFFF',
          100: '#C9D1D9',
          200: '#8B949E',
          300: '#30363D',
          400: '#161B22',
          500: '#0D1117',
          600: '#238636',
          700: '#207C32',
        },
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
  ],
}


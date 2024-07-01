/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{js,ts,jsx,tsx}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: 'var(--color-primary)',
        secondary: 'var(--color-secondary)',
        bgPrimary: 'var(--color-bg-primary)',
        tBase: 'var(--color-text-base)'
      }
    }
  },
  plugins: [require('@tailwindcss/typography')]
}

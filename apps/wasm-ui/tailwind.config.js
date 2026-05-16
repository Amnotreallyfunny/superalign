/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        scientific: {
          emerald: '#10b981',
          amber: '#f59e0b',
          crimson: '#ef4444',
          indigo: '#312e81',
          cyan: '#22d3ee',
          slate: '#1e293b',
          violet: '#8b5cf6',
        }
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      }
    },
  },
  plugins: [],
}

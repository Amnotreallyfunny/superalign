/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        bio: {
          950: '#020617',
          900: '#0f172a',
          800: '#1e293b',
          base: '#050a14',
          card: 'rgba(15, 23, 42, 0.6)',
          accent: '#10b981', // Emerald
          glow: '#2dd4bf',   // Teal
          hazard: '#ef4444', // Crimson
          process: '#8b5cf6' // Violet
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      backgroundImage: {
        'grid-pattern': "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='100' height='100' viewBox='0 0 100 100'%3E%3Cg fill-rule='evenodd'%3E%3Cg fill='%231e293b' fill-opacity='0.1'%3E%3Cpath d='M98 0a2 2 0 1 0 0 4 2 2 0 0 0 0-4zM0 98a2 2 0 1 0 0 4 2 2 0 0 0 0-4zm98 98a2 2 0 1 0 0 4 2 2 0 0 0 0-4zM0 0a2 2 0 1 0 0 4 2 2 0 0 0 0-4z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\")",
      }
    },
  },
  plugins: [],
}

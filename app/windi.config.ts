import { defineConfig } from 'windicss/helpers';

export default defineConfig({
  theme: {
    container: {
      padding: {
        DEFAULT: '0.5rem',
        sm: '1rem',
        lg: '2rem',
        xl: '2.5rem',
      },
    },
  },
  extract: {
    include: ['**/*.{tsx,jsx,js,css,scss,html}'],
    exclude: ['node_modules', '.git', '.next', 'out'],
  },
  plugins: [require('daisyui')],
  // daisyUI config (optional)
  daisyui: {
    themes: false,
  },
});

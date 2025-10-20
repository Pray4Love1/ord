import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  server: {
    port: 5173,
    host: '127.0.0.1',
    proxy: {
      '/mirror': {
        target: 'http://127.0.0.1:8787',
        changeOrigin: true
      }
    }
  },
  plugins: [react()]
});

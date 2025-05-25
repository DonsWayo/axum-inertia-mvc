import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { resolve } from 'path';

export default defineConfig({
  plugins: [
    react(),
    tailwindcss(),
  ],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      // Only watch the views directory
      ignored: [
        '**/node_modules/**',
        '**/dist/**',
        '**/target/**',
        '**/src/routes/**',
        '**/src/models/**',
        '**/src/main.rs',
        '**/src/pages/**',
        '**/*.toml',
        '**/*.lock'
      ]
    },
  },
  build: {
    outDir: 'dist',
    manifest: true,
  },
});

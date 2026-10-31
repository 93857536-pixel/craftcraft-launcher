import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    // Bind IPv4 loopback explicitly. Vite's default ("localhost") resolves to ::1
    // on this machine, which makes http://127.0.0.1:5173 (and any tooling that
    // probes IPv4) fail with "connection refused".
    host: '127.0.0.1',
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
});

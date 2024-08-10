import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    host: true,
    port: 8081,
    watch: {
      usePolling: true,
    },
    hmr: {
      clientPort: 80,
    },
  },
  resolve: {
    alias: {
      "@": "/src",
    },
  },
});

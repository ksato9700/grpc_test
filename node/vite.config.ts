import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    ssr: true,
    lib: {
      entry: ['src/server.ts', 'src/client.ts'],
      formats: ['es'],
    },
    rollupOptions: {
      external: [/^@grpc\//, 'path', 'fs', 'url'],
    },
  },
});

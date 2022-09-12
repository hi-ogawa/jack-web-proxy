import { defineConfig } from "vite";
import path from "path";
import react from "@vitejs/plugin-react";
import pages from "vite-plugin-react-pages";
import windicss from "vite-plugin-windicss";

module.exports = defineConfig({
  plugins: [
    windicss(),
    react(),
    pages({
      pagesDir: path.resolve(__dirname, "src", "pages"),
    }),
  ],
});

import path from "path";
import { defineConfig, normalizePath } from "vite";
// import { createHtmlPlugin } from 'vite-plugin-html'
import virtualHtmlTemplate from "vite-plugin-virtual-html-template";
import react from "@vitejs/plugin-react";
import autoprefixer from 'autoprefixer';
import eslint from "vite-plugin-eslint";
import { readdirSync, accessSync, constants } from "fs";

const variablePath = normalizePath(path.resolve("src/global/common.less"));

const title = {
  edit: '编辑',
}

const pages = readdirSync(path.resolve(__dirname, "src/page")).reduce(
  (pre, cur) => {
    try {
      accessSync(path.resolve(__dirname, `./src/page/${cur}/index.tsx`), constants.F_OK)
      pre[0][cur] = {
        template: 'public/index.html',
        title: title[cur] ?? cur,
        entry: `src/page/${cur}/index.tsx`,
      };
      pre[1][cur] = {
        [cur]: path.resolve(__dirname, `${cur}.html`),
      };

    } catch (error) {
    }
    return pre;
    
  },
  [{}, {}]
);

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    // eslint(),
    react({ fastRefresh: false }),
    virtualHtmlTemplate({
      pages: pages[0],
    }),
  ],
  css: {
    preprocessorOptions: {
      less: {
        additionalData: `@import "${variablePath}";`,
      },
    },
    postcss: {
      plugins: [
        autoprefixer({
          // 指定目标浏览器
          overrideBrowserslist: ["Chrome > 40", "ff > 31", "ie 11"],
        }),
      ],
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM === "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    emptyOutDir: true,
    rollupOptions: {
      input: pages[1],
    },
  },
}));

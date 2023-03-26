// import { readdirSync } from 'fs'
import path from 'path'
import { defineConfig, normalizePath } from 'vite'
// import { createHtmlPlugin } from 'vite-plugin-html'
import virtualHtmlTemplate from 'vite-plugin-virtual-html-template'
import react from '@vitejs/plugin-react'
import autoprefixer from 'autoprefixer'
import eslint from 'vite-plugin-eslint'

const variablePath = normalizePath(path.resolve('src/global/common.less'))

// const pages = readdirSync(path.resolve(__dirname, "src/page")).map((pageName) => {
//   return {
//     // entry: path.resolve(__dirname, `../../src/page/${pageName}/index.tsx`),
//     entry: `src/page/${pageName}/index.tsx`,
//     template: "public/index.html",
//     filename: `${pageName}.html`,
//     injectOptions: {
//       data: {
//         title: pageName,
//       },
//     },
//   }
// })
// console.log(pages)

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    eslint(),
    react({ fastRefresh: false }),
    virtualHtmlTemplate({
      pages: {
        index: {
          template: 'public/index.html',
          title: 'main',
          entry: 'src/page/index/index.tsx',
        },
        edit: {
          template: 'public/index.html',
          title: 'edit',
          entry: 'src/page/edit/index.tsx',
        },
      },
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
          overrideBrowserslist: ['Chrome > 40', 'ff > 31', 'ie 11'],
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
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: path.resolve(__dirname, 'index.html'),
        edit: path.resolve(__dirname, 'edit.html'),
      },
    },
  },
}))

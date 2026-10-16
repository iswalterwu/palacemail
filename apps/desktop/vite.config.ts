import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Tauri 在移动端或远程调试时通过此环境变量指定宿主；桌面开发为空。
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],

  // 以下选项为 Tauri 定制，只在 `tauri dev` / `tauri build` 时有意义。
  // 1. 不清屏，避免遮住 Rust 侧的错误输出。
  clearScreen: false,
  // 2. Tauri 需要固定端口：端口被占用时直接失败，而非自动换端口。
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. 不监听 src-tauri，Rust 侧的改动由 cargo 负责。
      ignored: ["**/src-tauri/**"],
    },
  },
});

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type VersionState =
  | { status: "loading" }
  | { status: "ready"; version: string }
  | { status: "error"; message: string };

/** 启动画面：显示产品名与内核版本号，是目前唯一的界面。 */
export default function App() {
  const [state, setState] = useState<VersionState>({ status: "loading" });

  useEffect(() => {
    let cancelled = false;
    // 示例 IPC：向 Rust 侧请求内核版本号。
    invoke<string>("kernel_version")
      .then((version) => {
        if (!cancelled) setState({ status: "ready", version });
      })
      .catch((error: unknown) => {
        if (!cancelled) setState({ status: "error", message: String(error) });
      });
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <main className="splash">
      <h1 className="splash__title">PalaceMail</h1>
      <p className="splash__tagline">以 AI 为内核，以人为界面的邮箱</p>
      <p className="splash__version">
        {state.status === "loading" && "正在读取内核版本…"}
        {state.status === "ready" && `内核版本 ${state.version}`}
        {state.status === "error" && `无法读取内核版本：${state.message}`}
      </p>
    </main>
  );
}

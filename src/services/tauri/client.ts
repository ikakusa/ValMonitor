import { invoke } from "@tauri-apps/api/core";

import type { AppError } from "@/types/riot";

export async function callCommand<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    // Tauri 通信をこの関数に閉じ込めることで、command 名変更や error DTO 変換の影響範囲を固定する。
    return await invoke<T>(command, args);
  } catch (error) {
    throw normalizeTauriError(error);
  }
}

function normalizeTauriError(error: unknown): AppError {
  if (typeof error === "object" && error !== null && "message" in error) {
    const candidate = error as Partial<AppError>;
    return {
      kind: candidate.kind ?? "unknown",
      message: candidate.message ?? "Unknown Tauri error",
    };
  }

  return {
    kind: "unknown",
    message: String(error),
  };
}

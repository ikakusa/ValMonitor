import { create } from "zustand";

export type AppPage = "account" | "live" | "settings";

type UiState = {
  page: AppPage;
  setPage: (page: AppPage) => void;
};

export const useUiStore = create<UiState>((set) => ({
  page: "account",
  // Layout 専用の UI 状態を feature から分離して、画面追加時に account logic を汚さない。
  setPage: (page) => set({ page }),
}));

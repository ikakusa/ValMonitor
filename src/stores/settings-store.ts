import { create } from "zustand";
import { persist } from "zustand/middleware";

type SettingsState = {
  language: "ja" | "en";
  refreshInterval: number;
  connectionRefreshInterval: number;
  rankRefreshInterval: number;
  discordRpcEnabled: boolean;
  discordRpcShowRank: boolean;
  discordRpcShowParty: boolean;
  compactMode: boolean;
  setLanguage: (language: "ja" | "en") => void;
  setRefreshInterval: (interval: number) => void;
  setConnectionRefreshInterval: (interval: number) => void;
  setRankRefreshInterval: (interval: number) => void;
  setDiscordRpcEnabled: (enabled: boolean) => void;
  setDiscordRpcShowRank: (enabled: boolean) => void;
  setDiscordRpcShowParty: (enabled: boolean) => void;
  setCompactMode: (enabled: boolean) => void;
};

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      language: "ja",
      refreshInterval: 2500,
      connectionRefreshInterval: 5000,
      rankRefreshInterval: 60_000,
      discordRpcEnabled: false,
      discordRpcShowRank: true,
      discordRpcShowParty: false,
      compactMode: false,
      // UI 言語は i18next と同期するため store に保存し、再起動後も同じ表示言語で開けるようにする。
      setLanguage: (language) => set({ language }),
      // Presence は UX に必要な範囲で短く保つが、1 秒未満の polling は Local API とログを無駄に増やす。
      // 設定画面から調整できるよう store に置き、query hook が直接参照する。
      setRefreshInterval: (refreshInterval) => set({ refreshInterval }),
      // VALORANT 未起動時は初期化確認だけが走るため、接続監視は少し長めを既定値にして無駄な invoke を抑える。
      setConnectionRefreshInterval: (connectionRefreshInterval) => set({ connectionRefreshInterval }),
      // MMR / Rank は変化頻度が低いため Presence と別設定にし、将来 Henrik API と併用しても負荷を分離できるようにする。
      setRankRefreshInterval: (rankRefreshInterval) => set({ rankRefreshInterval }),
      // Discord RPC は外部プロセス連携になるため、将来の Rust 側サービス実装時に明示 opt-in で起動できる状態を作っておく。
      setDiscordRpcEnabled: (discordRpcEnabled) => set({ discordRpcEnabled }),
      setDiscordRpcShowRank: (discordRpcShowRank) => set({ discordRpcShowRank }),
      setDiscordRpcShowParty: (discordRpcShowParty) => set({ discordRpcShowParty }),
      setCompactMode: (compactMode) => set({ compactMode }),
    }),
    {
      name: "valmonitor-settings",
      partialize: (state) => ({
        refreshInterval: state.refreshInterval,
        connectionRefreshInterval: state.connectionRefreshInterval,
        rankRefreshInterval: state.rankRefreshInterval,
        discordRpcEnabled: state.discordRpcEnabled,
        discordRpcShowRank: state.discordRpcShowRank,
        discordRpcShowParty: state.discordRpcShowParty,
        compactMode: state.compactMode,
        language: state.language,
      }),
    },
  ),
);

import { useEffect } from "react";
import { useTranslation } from "react-i18next";

import { DiscordRPC } from "@/services/tauri/discord";
import { useSettingsStore } from "@/stores/settings-store";
import { useUiStore } from "@/stores/ui-store";

const pageLabelKeys = {
  account: "common.account",
  live: "common.liveView",
  settings: "common.settings",
} as const;

export function useDiscordRpc() {
  const { t } = useTranslation();
  const enabled = useSettingsStore((state) => state.discordRpcEnabled);
  const showRank = useSettingsStore((state) => state.discordRpcShowRank);
  const page = useUiStore((state) => state.page);

  useEffect(() => {
    let cancelled = false;

    async function sync() {
      if (!enabled) {
        await DiscordRPC.clear();
        return;
      }

      // Discord IPC は外部プロセスに依存するため、失敗しても UI を止めない。
      // 詳細な状態更新は将来の dedicated service で throttling して行う。
      await DiscordRPC.setActivity({
        details: t("discordPresence.details"),
        state: showRank
          ? t("discordPresence.viewing", { page: t(pageLabelKeys[page]) })
          : t("discordPresence.idle"),
      });
    }

    void sync().catch((error) => {
      if (!cancelled) {
        console.debug("Discord RPC sync failed", error);
      }
    });

    return () => {
      cancelled = true;
    };
  }, [enabled, page, showRank, t]);
}

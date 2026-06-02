import { AnimatePresence, motion } from "motion/react";
import { Activity, Maximize2, Minus, RadioTower, Settings, Square, UserRound, X } from "lucide-react";
import type { ReactNode } from "react";
import { useEffect, useMemo, useState } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { type AppPage, useUiStore } from "@/stores/ui-store";

const navItems: Array<{ page: AppPage; labelKey: string; icon: typeof UserRound }> = [
  { page: "account", labelKey: "common.account", icon: UserRound },
  { page: "live", labelKey: "common.liveView", icon: RadioTower },
  { page: "settings", labelKey: "common.settings", icon: Settings },
];

const pageLabelKeys: Record<AppPage, string> = {
  account: "common.account",
  live: "common.liveView",
  settings: "common.settings",
};

export function AppLayout({ children }: { children: ReactNode }) {
  const page = useUiStore((state) => state.page);

  return (
    <div className="flex h-screen flex-col overflow-hidden bg-background text-foreground">
      <WindowHeader />
      <main className="min-h-0 flex-1 overflow-hidden">
        <AnimatePresence mode="wait">
          <motion.div
            key={page}
            className="h-full"
            initial={{ opacity: 0, y: 8 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -6 }}
            transition={{ duration: 0.16, ease: "easeOut" }}
          >
            {children}
          </motion.div>
        </AnimatePresence>
      </main>
    </div>
  );
}

function WindowHeader() {
  const { t } = useTranslation();
  const page = useUiStore((state) => state.page);
  const setPage = useUiStore((state) => state.setPage);

  return (
    <header
      className="flex h-14 shrink-0 items-center justify-between border-b bg-card/80 pl-4"
      data-tauri-drag-region
    >
      <div className="flex min-w-0 items-center gap-4" data-tauri-drag-region>
        <BrandMark />
        <nav className="flex items-center gap-1 rounded-md border bg-background/70 p-1">
          {navItems.map((item) => {
            const Icon = item.icon;
            const active = item.page === page;

            return (
              <button
                className={cn(
                  "relative flex h-8 items-center gap-2 rounded-sm px-3 text-xs font-medium text-muted-foreground transition-colors hover:text-foreground",
                  active && "text-foreground",
                )}
                key={item.page}
                onClick={() => setPage(item.page)}
                type="button"
              >
                {active && (
                  <motion.span
                    className="absolute inset-0 rounded-sm bg-primary/15 ring-1 ring-primary/25"
                    layoutId="page-active-pill"
                    transition={{ duration: 0.18, ease: "easeOut" }}
                  />
                )}
                <Icon className="relative h-3.5 w-3.5" />
                <span className="relative hidden sm:inline">{t(item.labelKey)}</span>
              </button>
            );
          })}
        </nav>
        <div className="hidden min-w-0 md:block" data-tauri-drag-region>
          <p className="truncate text-sm font-semibold">{t(pageLabelKeys[page])}</p>
          <p className="truncate text-xs text-muted-foreground">{t("layout.pageSubtitle")}</p>
        </div>
      </div>

      <div className="flex h-full items-center">
        <div className="mr-2 hidden items-center gap-2 rounded-md border bg-background/70 px-2.5 py-1 text-xs text-muted-foreground md:flex">
          <Activity className="h-3.5 w-3.5 text-primary" />
          {t("common.desktop")}
        </div>
        <WindowControls />
      </div>
    </header>
  );
}

function BrandMark() {
  const { t } = useTranslation();
  return (
    <div className="flex items-center gap-2" data-tauri-drag-region>
      <div className="grid h-8 w-8 place-items-center rounded-md border bg-background font-mono text-[11px] font-semibold text-primary shadow-sm">
        VM
      </div>
      <div className="hidden leading-tight sm:block" data-tauri-drag-region>
        <p className="text-sm font-semibold">ValMonitor</p>
        <p className="text-[11px] text-muted-foreground">{t("layout.subtitle")}</p>
      </div>
    </div>
  );
}

function WindowControls() {
  const { t } = useTranslation();
  const [isMaximized, setIsMaximized] = useState(false);
  const runningInTauri = isTauri();
  const appWindow = useMemo(() => (runningInTauri ? getCurrentWindow() : null), [runningInTauri]);

  useEffect(() => {
    if (!appWindow) return undefined;

    let unlisten: (() => void) | undefined;

    // カスタムヘッダーは Tauri IPC の権限に依存するため、失敗時は UI 全体を落とさず
    // 通常の Web プレビューだけ継続できるようにしておく。
    void appWindow.isMaximized().then(setIsMaximized).catch(() => setIsMaximized(false));
    void appWindow.onResized(async () => {
      setIsMaximized(await appWindow.isMaximized());
    }).then((handler) => {
      unlisten = handler;
    }).catch(() => {
      unlisten = undefined;
    });

    return () => unlisten?.();
  }, [appWindow]);

  if (!appWindow) return null;

  return (
    <div className="flex h-full">
      <Button
        aria-label={t("layout.minimize")}
        className="h-full w-11 rounded-none"
        onClick={() => void appWindow.minimize()}
        size="icon"
        variant="ghost"
      >
        <Minus className="h-4 w-4" />
      </Button>
      <Button
        aria-label={isMaximized ? t("layout.restore") : t("layout.maximize")}
        className="h-full w-11 rounded-none"
        onClick={async () => {
          await appWindow.toggleMaximize();
          setIsMaximized(await appWindow.isMaximized());
        }}
        size="icon"
        variant="ghost"
      >
        {isMaximized ? <Square className="h-3.5 w-3.5" /> : <Maximize2 className="h-4 w-4" />}
      </Button>
      <Button
        aria-label={t("layout.close")}
        className="h-full w-11 rounded-none hover:bg-destructive hover:text-white"
        onClick={() => void appWindow.close()}
        size="icon"
        variant="ghost"
      >
        <X className="h-4 w-4" />
      </Button>
    </div>
  );
}

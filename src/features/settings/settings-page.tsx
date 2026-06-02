import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Activity, Clock3, FileText, Gamepad2, PlugZap, Settings2, type LucideIcon } from "lucide-react";
import { motion } from "motion/react";
import type { ReactNode } from "react";
import { useState } from "react";
import { useTranslation } from "react-i18next";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Switch } from "@/components/ui/switch";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { HenrikAPI } from "@/services/tauri/henrik";
import { useSettingsStore } from "@/stores/settings-store";
import { cn } from "@/lib/utils";
import type { SupportedLanguage } from "@/app/i18n";

export function SettingsPage() {
  const { i18n, t } = useTranslation();
  const language = useSettingsStore((state) => state.language);
  const refreshInterval = useSettingsStore((state) => state.refreshInterval);
  const connectionRefreshInterval = useSettingsStore((state) => state.connectionRefreshInterval);
  const rankRefreshInterval = useSettingsStore((state) => state.rankRefreshInterval);
  const discordRpcEnabled = useSettingsStore((state) => state.discordRpcEnabled);
  const discordRpcShowRank = useSettingsStore((state) => state.discordRpcShowRank);
  const discordRpcShowParty = useSettingsStore((state) => state.discordRpcShowParty);
  const compactMode = useSettingsStore((state) => state.compactMode);
  const setLanguage = useSettingsStore((state) => state.setLanguage);
  const setRefreshInterval = useSettingsStore((state) => state.setRefreshInterval);
  const setConnectionRefreshInterval = useSettingsStore((state) => state.setConnectionRefreshInterval);
  const setRankRefreshInterval = useSettingsStore((state) => state.setRankRefreshInterval);
  const setDiscordRpcEnabled = useSettingsStore((state) => state.setDiscordRpcEnabled);
  const setDiscordRpcShowRank = useSettingsStore((state) => state.setDiscordRpcShowRank);
  const setDiscordRpcShowParty = useSettingsStore((state) => state.setDiscordRpcShowParty);
  const setCompactMode = useSettingsStore((state) => state.setCompactMode);
  const [apiKey, setApiKey] = useState("");
  const queryClient = useQueryClient();
  const henrikSettings = useQuery({
    queryKey: ["henrik", "settings"],
    queryFn: HenrikAPI.getSettings,
  });
  const saveApiKey = useMutation({
    mutationFn: HenrikAPI.saveApiKey,
    onSuccess: () => {
      setApiKey("");
      void queryClient.invalidateQueries({ queryKey: ["henrik", "settings"] });
    },
  });

  function changeLanguage(nextLanguage: SupportedLanguage) {
    setLanguage(nextLanguage);
    void i18n.changeLanguage(nextLanguage);
  }

  return (
    <div className="h-full overflow-auto p-4 md:p-6">
      <Tabs defaultValue="general">
        <div className="mb-4 flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
          <div className="flex items-center gap-2">
            <Settings2 className="h-4 w-4 text-primary" />
            <h1 className="text-lg font-semibold">{t("settings.title")}</h1>
          </div>
          <TabsList className="w-full justify-start overflow-x-auto lg:w-auto">
            <TabsTrigger value="general">{t("settings.tabs.general")}</TabsTrigger>
            <TabsTrigger value="henrik">{t("settings.tabs.henrik")}</TabsTrigger>
            <TabsTrigger value="discord">{t("settings.tabs.discord")}</TabsTrigger>
            <TabsTrigger value="runtime">{t("settings.tabs.runtime")}</TabsTrigger>
          </TabsList>
        </div>

        <AnimatedTab value="general">
          <div className="grid gap-4 xl:grid-cols-[1.15fr_0.85fr]">
            <Card>
              <CardHeader>
                <CardTitle>{t("settings.refresh.title")}</CardTitle>
                <CardDescription>{t("settings.refresh.description")}</CardDescription>
              </CardHeader>
              <CardContent className="space-y-5">
                <IntervalControl
                  description={t("settings.refresh.presenceDescription")}
                  label={t("settings.refresh.presenceLabel")}
                  max={8000}
                  min={1000}
                  onChange={setRefreshInterval}
                  step={250}
                  value={refreshInterval}
                />
                <IntervalControl
                  description={t("settings.refresh.connectionDescription")}
                  label={t("settings.refresh.connectionLabel")}
                  max={15000}
                  min={3000}
                  onChange={setConnectionRefreshInterval}
                  step={500}
                  value={connectionRefreshInterval}
                />
                <IntervalControl
                  description={t("settings.refresh.rankDescription")}
                  label={t("settings.refresh.rankLabel")}
                  max={180000}
                  min={15000}
                  onChange={setRankRefreshInterval}
                  step={5000}
                  value={rankRefreshInterval}
                />
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>{t("settings.interface.title")}</CardTitle>
                <CardDescription>{t("settings.interface.description")}</CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <SettingSwitch
                  checked={compactMode}
                  description={t("settings.interface.compactDescription")}
                  label={t("settings.interface.compactLabel")}
                  onCheckedChange={setCompactMode}
                />
                <Separator />
                <StatusLine
                  icon={Gamepad2}
                  label={t("settings.interface.startupLabel")}
                  value={t("settings.interface.startupValue")}
                />
                <StatusLine
                  icon={Activity}
                  label={t("settings.interface.windowLabel")}
                  value={t("settings.interface.windowValue")}
                />
              </CardContent>
            </Card>

            <Card className="xl:col-span-2">
              <CardHeader>
                <CardTitle>{t("settings.language.title")}</CardTitle>
                <CardDescription>{t("settings.language.description")}</CardDescription>
              </CardHeader>
              <CardContent>
                <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                  <div>
                    <p className="text-sm font-medium">{t("settings.language.label")}</p>
                    <p className="text-xs text-muted-foreground">
                      {language === "ja" ? t("settings.language.ja") : t("settings.language.en")}
                    </p>
                  </div>
                  <div className="grid grid-cols-2 rounded-xl border bg-background/70 p-1">
                    {(["ja", "en"] as const).map((item) => (
                      <button
                        className={cn(
                          "rounded-lg px-4 py-2 text-sm font-medium text-muted-foreground transition-colors",
                          language === item && "bg-primary/15 text-foreground ring-1 ring-primary/25",
                        )}
                        key={item}
                        onClick={() => changeLanguage(item)}
                        type="button"
                      >
                        {t(`settings.language.${item}`)}
                      </button>
                    ))}
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </AnimatedTab>

        <AnimatedTab value="henrik">
          <Card>
            <CardHeader className="flex-row items-center justify-between space-y-0">
              <div>
                <CardTitle>{t("settings.henrik.title")}</CardTitle>
                <CardDescription>{t("settings.henrik.description")}</CardDescription>
              </div>
              <Badge variant={henrikSettings.data?.hasApiKey ? "success" : "muted"}>
                {henrikSettings.data?.hasApiKey ? t("settings.henrik.keySet") : t("settings.henrik.keyMissing")}
              </Badge>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid gap-2 text-sm text-muted-foreground">
                <PathLine label="settings.json" value={henrikSettings.data?.settingsPath} />
                <PathLine label="api.txt" value={henrikSettings.data?.apiKeyPath} />
                <PathLine label="debug.log" value={henrikSettings.data?.debugLogPath} />
                <StatusLine
                  icon={PlugZap}
                  label={t("settings.henrik.authMode")}
                  value={henrikSettings.data?.authMode ?? "header"}
                />
                <p className="rounded-md border bg-background/60 p-3 text-sm text-muted-foreground">
                  {t("settings.henrik.distributionNote")}
                </p>
              </div>
              <div className="flex flex-col gap-2 sm:flex-row">
                <input
                  className="h-9 flex-1 rounded-md border bg-background px-3 text-sm text-foreground outline-none ring-offset-background focus-visible:ring-2 focus-visible:ring-ring"
                  onChange={(event) => setApiKey(event.target.value)}
                  placeholder={t("settings.henrik.placeholder")}
                  type="password"
                  value={apiKey}
                />
                <Button
                  disabled={!apiKey.trim() || saveApiKey.isPending}
                  onClick={() => saveApiKey.mutate(apiKey)}
                  type="button"
                >
                  {t("common.save")}
                </Button>
              </div>
              {saveApiKey.error && (
                <p className="text-sm text-destructive">{String(saveApiKey.error.message)}</p>
              )}
            </CardContent>
          </Card>
        </AnimatedTab>

        <AnimatedTab value="discord">
          <div className="grid gap-4 xl:grid-cols-[0.9fr_1.1fr]">
            <Card>
              <CardHeader>
                <CardTitle>{t("settings.discord.title")}</CardTitle>
                <CardDescription>{t("settings.discord.description")}</CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <SettingSwitch
                  checked={discordRpcEnabled}
                  description={t("settings.discord.enableDescription")}
                  label={t("settings.discord.enableLabel")}
                  onCheckedChange={setDiscordRpcEnabled}
                />
                <SettingSwitch
                  checked={discordRpcShowRank}
                  description={t("settings.discord.rankDescription")}
                  disabled={!discordRpcEnabled}
                  label={t("settings.discord.rankLabel")}
                  onCheckedChange={setDiscordRpcShowRank}
                />
                <SettingSwitch
                  checked={discordRpcShowParty}
                  description={t("settings.discord.partyDescription")}
                  disabled={!discordRpcEnabled}
                  label={t("settings.discord.partyLabel")}
                  onCheckedChange={setDiscordRpcShowParty}
                />
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>{t("settings.discord.notesTitle")}</CardTitle>
                <CardDescription>{t("settings.discord.notesDescription")}</CardDescription>
              </CardHeader>
              <CardContent className="space-y-3 text-sm text-muted-foreground">
                <p>
                  {t("settings.discord.noteClientId")}
                </p>
                <p>
                  {t("settings.discord.notePrivacy")}
                </p>
              </CardContent>
            </Card>
          </div>
        </AnimatedTab>

        <AnimatedTab value="runtime">
          <div className="grid gap-4 xl:grid-cols-2">
            <Card>
              <CardHeader>
                <CardTitle>{t("settings.runtime.title")}</CardTitle>
                <CardDescription>{t("settings.runtime.description")}</CardDescription>
              </CardHeader>
              <CardContent className="grid gap-2 text-sm text-muted-foreground">
                <StatusLine icon={Activity} label={t("settings.runtime.ui")} value="React / TypeScript / Zustand" />
                <StatusLine icon={Clock3} label={t("settings.runtime.animation")} value="motion/react" />
                <StatusLine icon={FileText} label={t("settings.runtime.logs")} value="%APPDATA%\\ValMonitor\\debug.log" />
              </CardContent>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle>{t("settings.runtime.startupTitle")}</CardTitle>
                <CardDescription>{t("settings.runtime.startupDescription")}</CardDescription>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">
                {t("settings.runtime.startupBody")}
              </CardContent>
            </Card>
          </div>
        </AnimatedTab>
      </Tabs>
    </div>
  );
}

function AnimatedTab({ children, value }: { children: ReactNode; value: string }) {
  return (
    <TabsContent value={value}>
      <motion.div
        initial={{ opacity: 0, y: 8 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.16, ease: "easeOut" }}
      >
        {children}
      </motion.div>
    </TabsContent>
  );
}

function IntervalControl({
  description,
  label,
  max,
  min,
  onChange,
  step,
  value,
}: {
  description: string;
  label: string;
  max: number;
  min: number;
  onChange: (value: number) => void;
  step: number;
  value: number;
}) {
  return (
    <div className="space-y-2">
      <div className="flex items-start justify-between gap-3">
        <div>
          <p className="text-sm font-medium">{label}</p>
          <p className="text-xs text-muted-foreground">{description}</p>
        </div>
        <input
          className="h-8 w-24 rounded-md border bg-background px-2 text-right text-sm outline-none focus-visible:ring-2 focus-visible:ring-ring"
          max={max}
          min={min}
          onChange={(event) => onChange(clamp(Number(event.target.value), min, max))}
          step={step}
          type="number"
          value={value}
        />
      </div>
      <input
        className="h-2 w-full accent-[var(--valorant)]"
        max={max}
        min={min}
        onChange={(event) => onChange(Number(event.target.value))}
        step={step}
        type="range"
        value={value}
      />
    </div>
  );
}

function SettingSwitch({
  checked,
  description,
  disabled,
  label,
  onCheckedChange,
}: {
  checked: boolean;
  description: string;
  disabled?: boolean;
  label: string;
  onCheckedChange: (checked: boolean) => void;
}) {
  return (
    <div className={cn("flex items-center justify-between gap-4", disabled && "opacity-60")}>
      <div className="min-w-0">
        <p className="text-sm font-medium">{label}</p>
        <p className="text-xs text-muted-foreground">{description}</p>
      </div>
      <Switch checked={checked} disabled={disabled} onCheckedChange={onCheckedChange} />
    </div>
  );
}

function PathLine({ label, value }: { label: string; value?: string }) {
  return (
    <div className="grid gap-1 rounded-md border bg-background/60 p-3 sm:grid-cols-[120px_1fr]">
      <span className="text-xs font-medium uppercase text-muted-foreground">{label}</span>
      <span className="break-all font-mono text-xs text-foreground">{value ?? "-"}</span>
    </div>
  );
}

function StatusLine({
  icon: Icon,
  label,
  value,
}: {
  icon: LucideIcon;
  label: string;
  value: string;
}) {
  return (
    <div className="flex items-center justify-between gap-3 rounded-md border bg-background/60 p-3">
      <span className="flex min-w-0 items-center gap-2 text-xs font-medium uppercase text-muted-foreground">
        <Icon className="h-3.5 w-3.5 text-primary" />
        {label}
      </span>
      <span className="min-w-0 break-words text-right text-sm text-foreground">{value}</span>
    </div>
  );
}

function clamp(value: number, min: number, max: number) {
  if (Number.isNaN(value)) return min;
  return Math.min(Math.max(value, min), max);
}

import { useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { Play, Sparkles } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { buildSkinViews } from "@/features/loadout/skin-utils";
import { RiotAPI } from "@/services/tauri/riot";
import { ValorantAssetsAPI } from "@/services/valorant/assets";
import { useSettingsStore } from "@/stores/settings-store";

export function AccountLoadoutSection({ initialized, puuid }: { initialized: boolean; puuid?: string }) {
  const { t } = useTranslation();
  const language = useSettingsStore((state) => state.language);
  const [video, setVideo] = useState<string | undefined>();
  const loadout = useQuery({
    queryKey: ["riot", "loadout", puuid],
    queryFn: () => RiotAPI.getPlayerLoadout(puuid ?? ""),
    enabled: initialized && Boolean(puuid),
    staleTime: 30 * 1000,
    refetchOnWindowFocus: false,
  });
  const weapons = useQuery({
    queryKey: ["valorant", "weapons", language],
    queryFn: () => ValorantAssetsAPI.getWeapons(language),
    enabled: Boolean(loadout.data?.Guns?.length),
    staleTime: 24 * 60 * 60 * 1000,
    refetchOnWindowFocus: false,
  });
  const labels = useMemo(
    () => ({
      unknownOffer: t("common.unknownOffer"),
      unknownSkin: t("common.unknownSkin"),
      unknownWeapon: t("common.unknownWeapon"),
    }),
    [t],
  );
  const skins = useMemo(
    () => buildSkinViews(loadout.data?.Guns, weapons.data, labels),
    [labels, loadout.data, weapons.data],
  );

  return (
    <section className="grid gap-4 xl:grid-cols-[1fr_0.75fr]">
      <Card>
        <CardHeader className="flex-row items-center justify-between space-y-0">
          <div>
            <CardTitle>{t("loadout.title")}</CardTitle>
            <CardDescription>{t("loadout.description")}</CardDescription>
          </div>
          <Badge variant="outline">{t("common.weapons", { count: skins.length })}</Badge>
        </CardHeader>
        <CardContent>
          {!initialized ? (
            <EmptyLoadout text={t("loadout.waiting")} />
          ) : loadout.isLoading || weapons.isLoading ? (
            <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-4">
              {Array.from({ length: 4 }).map((_, index) => (
                <Skeleton className="h-36" key={index} />
              ))}
            </div>
          ) : skins.length > 0 ? (
            <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-4 2xl:grid-cols-5">
              {skins.map((skin) => (
                <button
                  className="rounded-md border bg-background/60 p-3 text-left transition-colors hover:border-primary/60"
                  key={skin.loadoutId}
                  onClick={() => setVideo(skin.video)}
                  type="button"
                >
                  <div className="mb-2 flex h-20 items-center justify-center rounded-md bg-secondary/45">
                    {skin.icon ? (
                      <img alt="" className="max-h-full max-w-full object-contain" src={skin.icon} />
                    ) : (
                      <Sparkles className="h-5 w-5 text-muted-foreground" />
                    )}
                  </div>
                  <p className="truncate text-xs text-muted-foreground">{skin.weaponName}</p>
                  <p className="truncate text-sm font-semibold">{skin.skinName}</p>
                  {skin.video && (
                    <p className="mt-1 flex items-center gap-1 text-xs text-primary">
                      <Play className="h-3 w-3" />
                      {t("common.video")}
                    </p>
                  )}
                </button>
              ))}
            </div>
          ) : (
            <EmptyLoadout text={t("loadout.unavailable")} />
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("loadout.skinVideo")}</CardTitle>
          <CardDescription>{t("loadout.upgradePreview")}</CardDescription>
        </CardHeader>
        <CardContent>
          {video ? (
            <video className="aspect-video w-full rounded-md border bg-black" controls src={video} />
          ) : (
            <div className="flex aspect-video items-center justify-center rounded-md border bg-background text-sm text-muted-foreground">
              {t("loadout.selectVideo")}
            </div>
          )}
        </CardContent>
      </Card>
    </section>
  );
}

function EmptyLoadout({ text }: { text: string }) {
  return (
    <div className="flex min-h-36 items-center justify-center rounded-md border bg-background/60 text-sm text-muted-foreground">
      {text}
    </div>
  );
}

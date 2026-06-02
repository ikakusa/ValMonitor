import { useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { AlertCircle, Play, ShoppingBag } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { buildStoreItems } from "@/features/loadout/skin-utils";
import { RiotAPI } from "@/services/tauri/riot";
import { ValorantAssetsAPI } from "@/services/valorant/assets";
import { useSettingsStore } from "@/stores/settings-store";

export function StoreSection({ initialized, puuid }: { initialized: boolean; puuid?: string }) {
  const { t } = useTranslation();
  const language = useSettingsStore((state) => state.language);
  const [video, setVideo] = useState<string | undefined>();
  const storefront = useQuery({
    queryKey: ["riot", "storefront", puuid],
    queryFn: () => RiotAPI.getStorefront(puuid ?? ""),
    enabled: initialized && Boolean(puuid),
    staleTime: 2 * 60 * 1000,
    refetchOnWindowFocus: false,
  });
  const weapons = useQuery({
    queryKey: ["valorant", "weapons", language],
    queryFn: () => ValorantAssetsAPI.getWeapons(language),
    enabled: Boolean(storefront.data?.SkinsPanelLayout),
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
  const items = useMemo(
    () => buildStoreItems(storefront.data, weapons.data, labels),
    [labels, storefront.data, weapons.data],
  );
  const remaining = storefront.data?.SkinsPanelLayout?.SingleItemOffersRemainingDurationInSeconds;
  const rawOfferCount = storefront.data?.SkinsPanelLayout?.SingleItemOffers?.length ?? 0;

  return (
    <section className="grid gap-4 xl:grid-cols-[1fr_0.75fr]">
      <Card>
        <CardHeader className="flex-row items-center justify-between space-y-0">
          <div>
            <CardTitle>{t("store.title")}</CardTitle>
            <CardDescription>{t("store.description")}</CardDescription>
          </div>
          <Badge variant={initialized ? "success" : "muted"}>
            <ShoppingBag className="mr-1 h-3 w-3" />
            {formatRemaining(remaining, t("common.store"))}
          </Badge>
        </CardHeader>
        <CardContent className="space-y-3">
          {storefront.error && (
            <StoreNotice
              text={storeErrorMessage(storefront.error, t("store.unavailable"), t("common.storefront"))}
            />
          )}
          {weapons.error && <StoreNotice text={`${t("common.assets")}: ${errorMessage(weapons.error)}`} />}
          {!initialized ? (
            <EmptyStore text={t("store.waiting")} />
          ) : storefront.isLoading ? (
            <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-4">
              {Array.from({ length: 4 }).map((_, index) => (
                <Skeleton className="h-48" key={index} />
              ))}
            </div>
          ) : items.length > 0 ? (
            <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-4">
              {items.map((item) => (
                <button
                  className="group overflow-hidden rounded-lg border bg-background text-left transition-colors hover:border-primary/60"
                  key={item.offerId}
                  onClick={() => setVideo(item.video)}
                  type="button"
                >
                  <div className="flex h-32 items-center justify-center bg-secondary/45 p-3">
                    {item.icon ? (
                      <img alt="" className="max-h-full max-w-full object-contain" src={item.icon} />
                    ) : (
                      <ShoppingBag className="h-6 w-6 text-muted-foreground" />
                    )}
                  </div>
                  <div className="space-y-1 p-3">
                    <p className="truncate text-xs text-muted-foreground">{item.weaponName}</p>
                    <p className="truncate text-sm font-semibold">{item.skinName}</p>
                    <div className="flex items-center justify-between gap-2 text-xs text-muted-foreground">
                      <span>{item.cost ? `${item.cost} VP` : t("common.vpUnavailable")}</span>
                      {item.video && (
                        <span className="inline-flex items-center gap-1 text-primary">
                          <Play className="h-3 w-3" />
                          {t("common.video")}
                        </span>
                      )}
                    </div>
                  </div>
                </button>
              ))}
            </div>
          ) : (
            <EmptyStore
              text={
                rawOfferCount > 0
                  ? t("store.mappingFailed")
                  : t("store.unavailable")
              }
            />
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("common.preview")}</CardTitle>
          <CardDescription>{t("store.previewDescription")}</CardDescription>
        </CardHeader>
        <CardContent>
          {video ? (
            <video className="aspect-video w-full rounded-md border bg-black" controls src={video} />
          ) : (
            <div className="flex aspect-video items-center justify-center rounded-md border bg-background text-sm text-muted-foreground">
              {t("store.selectVideo")}
            </div>
          )}
        </CardContent>
      </Card>
    </section>
  );
}

function EmptyStore({ text }: { text: string }) {
  return (
    <div className="flex min-h-40 items-center justify-center rounded-md border bg-background/60 text-sm text-muted-foreground">
      {text}
    </div>
  );
}

function StoreNotice({ text }: { text: string }) {
  return (
    <div className="flex gap-3 rounded-md border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive">
      <AlertCircle className="mt-0.5 h-4 w-4 shrink-0" />
      <span className="min-w-0 break-words">{text}</span>
    </div>
  );
}

function errorMessage(error: unknown) {
  if (!error || typeof error !== "object") return String(error);
  const candidate = error as { message?: string };
  return candidate.message ?? String(error);
}

function storeErrorMessage(error: unknown, fallback: string, prefix: string) {
  if (!error || typeof error !== "object") return fallback;
  const candidate = error as { kind?: string; message?: string };
  if (candidate.kind === "storefrontUnavailable" || candidate.kind === "riotSessionNotReady") {
    return fallback;
  }
  return `${prefix}: ${candidate.message ?? String(error)}`;
}

function formatRemaining(seconds: number | undefined, fallback: string) {
  if (!seconds) return fallback;
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return `${hours}h ${minutes}m`;
}

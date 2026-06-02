import { motion } from "motion/react";
import { AlertCircle, CheckCircle2, CircleOff, MapPin, Shield, UserRound } from "lucide-react";
import { useTranslation } from "react-i18next";

import valorantIcon from "@/Assets/valorant_icon.png";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { RankCard } from "@/features/account/rank-card";
import { AccountLoadoutSection } from "@/features/account/account-loadout-section";
import { StoreSection } from "@/features/account/store-section";
import { useAccountData } from "@/features/account/use-account-data";

export function AccountPage() {
  const { t } = useTranslation();
  const { data, initialized, loading, error } = useAccountData();
  const playerIcon = data.card?.displayIcon ?? valorantIcon;

  return (
    <ScrollArea className="h-[calc(100vh-4rem)]">
      <div className="space-y-5 p-4 md:p-6">
        <motion.section
          className="grid gap-4 lg:grid-cols-[1.2fr_0.8fr]"
          initial={{ opacity: 0, y: 8 }}
          animate={{ opacity: 1, y: 0 }}
        >
          <Card className="overflow-hidden">
            <CardContent className="p-0">
              <div className="flex flex-col gap-5 p-5 sm:flex-row sm:items-center">
                <img
                  alt=""
                  className="h-24 w-24 rounded-lg border bg-muted object-cover"
                  src={playerIcon}
                />
                <div className="min-w-0 flex-1">
                  <div className="mb-2 flex flex-wrap items-center gap-2">
                    <Badge variant={initialized ? "success" : "muted"}>
                      {initialized ? t("common.connected") : t("common.waiting")}
                    </Badge>
                    <Badge variant="outline">{data.region ?? t("common.region")}</Badge>
                  </div>
                  {loading ? (
                    <div className="space-y-2">
                      <Skeleton className="h-8 w-64" />
                      <Skeleton className="h-4 w-80 max-w-full" />
                    </div>
                  ) : (
                    <>
                      <h1 className="truncate text-3xl font-semibold tracking-normal">
                        {data.fullName ?? t("common.riotClient")}
                      </h1>
                      <p className="mt-1 truncate font-mono text-xs text-muted-foreground">
                        {data.puuid ?? t("account.puuidUnavailable")}
                      </p>
                    </>
                  )}
                </div>
              </div>
              <Separator />
              <div className="grid gap-px bg-border sm:grid-cols-3">
                <Metric icon={UserRound} label={t("account.level")} value={data.accountLevel} />
                <Metric icon={Shield} label={t("account.gameState")} value={data.gameState} />
                <Metric icon={MapPin} label={t("common.region")} value={data.region ?? t("common.unknown")} />
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>{t("account.riotConnection")}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center gap-3">
                {initialized ? (
                  <CheckCircle2 className="h-5 w-5 text-emerald-400" />
                ) : (
                  <CircleOff className="h-5 w-5 text-muted-foreground" />
                )}
                <div>
                  <p className="text-sm font-medium">
                    {initialized ? t("account.sessionActive") : t("account.noSession")}
                  </p>
                  <p className="text-xs text-muted-foreground">
                    {initialized ? t("account.sessionActiveDescription") : t("account.noSessionDescription")}
                  </p>
                </div>
              </div>
              {error && (
                <div className="flex gap-3 rounded-md border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive">
                  <AlertCircle className="mt-0.5 h-4 w-4 shrink-0" />
                  <span className="min-w-0 break-words">{String(error.message ?? error)}</span>
                </div>
              )}
            </CardContent>
          </Card>
        </motion.section>

        <section className="grid gap-4 md:grid-cols-3">
          <RankCard title={t("account.currentRank")} rank={data.currentRank} showRr />
          <RankCard title={t("account.seasonPeak")} rank={data.currentSeasonPeak} />
          <RankCard title={t("account.peakRank")} rank={data.peakRank} />
        </section>

        <AccountLoadoutSection initialized={initialized} puuid={data.puuid} />
        <StoreSection initialized={initialized} puuid={data.puuid} />
      </div>
    </ScrollArea>
  );
}

function Metric({
  icon: Icon,
  label,
  value,
}: {
  icon: typeof UserRound;
  label: string;
  value: string | number;
}) {
  return (
    <div className="bg-card p-4">
      <div className="mb-2 flex items-center gap-2 text-xs text-muted-foreground">
        <Icon className="h-3.5 w-3.5 text-primary" />
        {label}
      </div>
      <p className="truncate text-lg font-semibold">{value}</p>
    </div>
  );
}

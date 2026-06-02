import { AnimatePresence, motion } from "motion/react";
import { AlertCircle, Check, ChevronDown, CircleOff, Play, RadioTower, Sparkles } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { cn } from "@/lib/utils";
import { useLiveViewData, type PlayerStats, type SkinView } from "@/features/live/use-live-view-data";

export function LivePage() {
  const { t } = useTranslation();
  const {
    errors,
    gameState,
    hasHenrikApiKey,
    inMatch,
    initialized,
    loading,
    players,
    selectedPlayerPuuid,
    setSelectedPlayerPuuid,
    skins,
    skinsLoading,
    stats,
    statsLoading,
  } = useLiveViewData();
  const [video, setVideo] = useState<string | undefined>();
  const roster = stats.length > 0 ? stats : players;
  const selectedPlayer = useMemo(
    () => roster.find((player) => player.puuid === selectedPlayerPuuid),
    [roster, selectedPlayerPuuid],
  );

  return (
    <ScrollArea className="h-[calc(100vh-4rem)]">
      <div className="space-y-5 p-4 md:p-6">
        <motion.section
          className="grid gap-4 xl:grid-cols-[1.2fr_0.8fr]"
          initial={{ opacity: 0, y: 8 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.16, ease: "easeOut" }}
        >
          <Card>
            <CardHeader className="flex-row items-center justify-between space-y-0">
              <div>
                <CardTitle>{t("live.title")}</CardTitle>
                <CardDescription>{t("live.description")}</CardDescription>
              </div>
              <Badge variant={initialized ? "success" : "muted"}>
                <RadioTower className="mr-1 h-3 w-3" />
                {gameState}
              </Badge>
            </CardHeader>
            <CardContent className="space-y-4">
              {!hasHenrikApiKey && (
                <InfoBox text={t("live.noHenrikKey")} />
              )}
              {errors.riot ? <InfoBox danger text={errorMessage(errors.riot)} /> : null}
              {errors.henrik ? <InfoBox text={`${t("common.henrik")}: ${errorMessage(errors.henrik)}`} /> : null}
              {!inMatch && stats.length === 0 ? (
                <div className="flex min-h-32 items-center justify-center rounded-md border bg-background/60 text-sm text-muted-foreground">
                  {t("live.notInMatch")}
                </div>
              ) : (
                <PlayerStatsTable loading={loading || statsLoading} stats={stats} />
              )}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>{t("live.skinViewer")}</CardTitle>
              <CardDescription>{t("live.skinViewerDescription")}</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <PlayerDropdown
                onChange={(puuid) => {
                  setSelectedPlayerPuuid(puuid);
                  setVideo(undefined);
                }}
                players={roster}
                value={selectedPlayerPuuid}
              />

              <div className="rounded-md border bg-background/60 p-3">
                <p className="text-sm font-medium">{selectedPlayer?.displayName ?? t("live.player")}</p>
                <p className="mt-1 break-all font-mono text-xs text-muted-foreground">
                  {selectedPlayer?.puuid ?? "PUUID unavailable"}
                </p>
              </div>

              {skinsLoading ? (
                <div className="grid gap-3">
                  <Skeleton className="h-28" />
                  <Skeleton className="h-28" />
                </div>
              ) : skins.length > 0 ? (
                <div className="grid max-h-[440px] gap-3 overflow-auto pr-1">
                  {skins.map((skin) => (
                    <SkinRow key={skin.loadoutId} onVideo={setVideo} skin={skin} />
                  ))}
                </div>
              ) : (
                <div className="flex min-h-32 items-center justify-center rounded-md border bg-background/60 text-sm text-muted-foreground">
                  <CircleOff className="mr-2 h-4 w-4" />
                  {t("live.loadoutUnavailable")}
                </div>
              )}

              <Separator />
              {video ? (
                <video className="aspect-video w-full rounded-md border bg-black" controls src={video} />
              ) : (
                <div className="flex aspect-video items-center justify-center rounded-md border bg-background text-sm text-muted-foreground">
                  {t("live.selectSkinVideo")}
                </div>
              )}
            </CardContent>
          </Card>
        </motion.section>
      </div>
    </ScrollArea>
  );
}

function PlayerDropdown({
  onChange,
  players,
  value,
}: {
  onChange: (puuid: string) => void;
  players: PlayerStats[];
  value: string;
}) {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);
  const rootRef = useRef<HTMLDivElement>(null);
  const selected = players.find((player) => player.puuid === value) ?? players[0];

  useEffect(() => {
    if (!open) return;

    function closeOnOutsideClick(event: MouseEvent) {
      // Dropdown は独立した浮動 UI なので、外側クリックで閉じないと Live 操作中に視界を塞ぎ続ける。
      if (!rootRef.current?.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", closeOnOutsideClick);
    return () => document.removeEventListener("mousedown", closeOnOutsideClick);
  }, [open]);

  if (!selected) {
    return (
      <div className="flex h-11 items-center rounded-xl border bg-background/60 px-3 text-sm text-muted-foreground">
        {t("live.playerDataUnavailable")}
      </div>
    );
  }

  return (
    <div className="relative" ref={rootRef}>
      <button
        aria-expanded={open}
        className={cn(
          "flex h-11 w-full items-center gap-3 rounded-xl border bg-background/80 px-3 text-left shadow-sm outline-none transition-colors",
          "hover:border-primary/60 hover:bg-background focus-visible:ring-2 focus-visible:ring-ring",
        )}
        onClick={() => setOpen((current) => !current)}
        onKeyDown={(event) => {
          if (event.key === "Escape") setOpen(false);
          if (event.key === "ArrowDown" || event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            setOpen(true);
          }
        }}
        type="button"
      >
        <PlayerDot player={selected} />
        <div className="min-w-0 flex-1">
          <p className="truncate text-sm font-semibold">
            {selected.isMe ? t("live.selfPrefix") : ""}
            {selected.displayName}
          </p>
          <p className="truncate text-[11px] text-muted-foreground">
            {selected.characterName ?? selected.puuid}
          </p>
        </div>
        <motion.span animate={{ rotate: open ? 180 : 0 }} transition={{ duration: 0.16 }}>
          <ChevronDown className="h-4 w-4 text-muted-foreground" />
        </motion.span>
      </button>

      <AnimatePresence>
        {open && (
          <motion.div
            className="absolute left-0 right-0 top-12 z-30 overflow-hidden rounded-2xl border bg-popover shadow-xl shadow-black/30"
            initial={{ opacity: 0, y: -6, scale: 0.98 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -4, scale: 0.98 }}
            transition={{ duration: 0.14, ease: "easeOut" }}
          >
            <div className="max-h-72 overflow-auto p-1.5">
              {players.map((player, index) => (
                <motion.button
                  className={cn(
                    "flex w-full items-center gap-3 rounded-xl px-3 py-2 text-left outline-none transition-colors",
                    "hover:bg-accent focus-visible:bg-accent",
                    player.puuid === selected.puuid && "bg-primary/10 text-foreground",
                  )}
                  initial={{ opacity: 0, x: -4 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: Math.min(index * 0.015, 0.08), duration: 0.12 }}
                  key={player.puuid}
                  onClick={() => {
                    onChange(player.puuid);
                    setOpen(false);
                  }}
                  type="button"
                >
                  <PlayerDot player={player} />
                  <div className="min-w-0 flex-1">
                    <p className="truncate text-sm font-medium">
                      {player.isMe ? t("live.selfPrefix") : ""}
                      {player.displayName}
                    </p>
                    <p className="truncate text-xs text-muted-foreground">
                      {player.characterName ?? t("live.agentUnknown")} / {player.rankName}
                    </p>
                  </div>
                  {player.puuid === selected.puuid && <Check className="h-4 w-4 text-primary" />}
                </motion.button>
              ))}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

function PlayerDot({ player }: { player: PlayerStats }) {
  const { t } = useTranslation();
  return (
    <span
      className={cn(
        "grid h-8 w-8 shrink-0 place-items-center rounded-full border bg-card text-[11px] font-semibold",
        player.team === "blue" && "border-blue-400/70 text-blue-300",
        player.team === "red" && "border-red-400/70 text-red-300",
        player.team === "unknown" && "border-muted-foreground/50 text-muted-foreground",
      )}
    >
      {player.characterIcon ? (
        <img alt="" className="h-full w-full rounded-full object-cover" src={player.characterIcon} />
      ) : player.isMe ? (
        t("live.me")
      ) : (
        player.displayName.slice(0, 1).toUpperCase()
      )}
    </span>
  );
}

function CharacterAvatar({ player }: { player: PlayerStats }) {
  return (
    <div className="grid h-10 w-10 shrink-0 place-items-center overflow-hidden rounded-lg border bg-secondary/45">
      {player.characterIcon ? (
        <img alt="" className="h-full w-full object-cover" src={player.characterIcon} />
      ) : (
        <Sparkles className="h-4 w-4 text-muted-foreground" />
      )}
    </div>
  );
}

function RankInline({
  detail,
  icon,
  name,
}: {
  detail?: string;
  icon?: string;
  name: string;
}) {
  return (
    <div className="flex min-w-0 items-center gap-2">
      <div className="grid h-7 w-7 shrink-0 place-items-center rounded-md bg-secondary/45">
        {icon ? (
          <img alt="" className="h-6 w-6 object-contain" src={icon} />
        ) : (
          <span className="h-2 w-2 rounded-full bg-muted-foreground/50" />
        )}
      </div>
      <div className="min-w-0">
        <p className="truncate text-sm">{name}</p>
        {detail && <p className="truncate text-[11px] text-muted-foreground">{detail}</p>}
      </div>
    </div>
  );
}

function PlayerStatsTable({ loading, stats }: { loading: boolean; stats: PlayerStats[] }) {
  const { t } = useTranslation();
  const teams = splitTeams(stats);

  if (loading) {
    return (
      <div className="grid gap-4">
        {["blue", "red"].map((team) => (
          <div className="space-y-2 rounded-md border p-3" key={team}>
            <Skeleton className="h-5 w-28" />
            {Array.from({ length: 5 }).map((_, index) => (
              <Skeleton className="h-10" key={index} />
            ))}
          </div>
        ))}
      </div>
    );
  }

  return (
    <div className="grid gap-4">
      <TeamStatsPanel players={teams.blue} team="blue" title={t("live.blueTeam")} />
      <TeamStatsPanel players={teams.red} team="red" title={t("live.redTeam")} />
    </div>
  );
}

function TeamStatsPanel({
  players,
  team,
  title,
}: {
  players: PlayerStats[];
  team: "blue" | "red";
  title: string;
}) {
  const { t } = useTranslation();
  const emptySlots = Math.max(5 - players.length, 0);

  return (
    <div className="overflow-hidden rounded-md border bg-background/40">
      <div className="flex items-center justify-between border-b bg-secondary/40 px-3 py-2">
        <div className="flex items-center gap-2">
          <span className={cn("h-2.5 w-2.5 rounded-full", team === "blue" ? "bg-blue-400" : "bg-red-400")} />
          <p className="text-sm font-semibold">{title}</p>
        </div>
        <Badge variant="outline">{players.length}/5</Badge>
      </div>
      <div className="overflow-auto">
        <table className="w-full min-w-[980px] border-collapse text-sm">
        <thead className="bg-secondary/60 text-xs uppercase text-muted-foreground">
          <tr>
            <th className="px-3 py-2 text-left">{t("live.player")}</th>
            <th className="px-3 py-2 text-left">{t("live.rank")}</th>
            <th className="px-3 py-2 text-left">{t("live.peak")}</th>
            <th className="px-3 py-2 text-right">{t("live.kd")}</th>
            <th className="px-3 py-2 text-right">{t("live.acs")}</th>
            <th className="px-3 py-2 text-right">{t("live.winRate")}</th>
            <th className="px-3 py-2 text-right">{t("live.hs")}</th>
            <th className="px-3 py-2 text-right">{t("live.adr")}</th>
            <th className="px-3 py-2 text-right">{t("live.matches")}</th>
          </tr>
        </thead>
        <tbody>
          {players.map((player, index) => (
            <tr
              className={cn(
                "border-t bg-card/40",
                player.isMe && "bg-primary/10",
              )}
              key={player.puuid}
            >
              <td className="px-3 py-2">
                <div className="flex items-center gap-2">
                  <PartyConnector player={player} position={partyPosition(players, player, index)} />
                  <CharacterAvatar player={player} />
                  <div className="min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="max-w-52 truncate font-medium">{player.displayName}</span>
                      {player.isMe && <Badge variant="outline">{t("live.you")}</Badge>}
                    </div>
                    <p className="truncate text-xs text-muted-foreground">
                      {player.characterName ?? t("live.agentUnknown")}
                    </p>
                  </div>
                </div>
              </td>
              <td className="px-3 py-2">
                <RankInline icon={player.rankIcon} name={player.rankName} />
              </td>
              <td className="px-3 py-2">
                <RankInline detail={player.peakSeasonName} icon={player.peakRankIcon} name={player.peakRankName} />
              </td>
              <td className="px-3 py-2 text-right font-mono">{formatNumber(player.kd, 2)}</td>
              <td className="px-3 py-2 text-right font-mono">{formatNumber(player.acs, 0)}</td>
              <td className="px-3 py-2 text-right font-mono">{formatNumber(player.winRate, 0)}</td>
              <td className="px-3 py-2 text-right font-mono">{formatNumber(player.hsPercent, 0)}</td>
              <td className="px-3 py-2 text-right font-mono">{formatNumber(player.adr, 0)}</td>
              <td className="px-3 py-2 text-right font-mono">{player.totalMatches || "--"}</td>
            </tr>
          ))}
          {Array.from({ length: emptySlots }).map((_, index) => (
            <tr className="border-t bg-card/20 text-muted-foreground" key={`empty-${team}-${index}`}>
              <td className="px-3 py-2" colSpan={9}>
                <div className="flex h-8 items-center gap-2">
                  <span className="h-2.5 w-2.5 rounded-full border border-dashed border-muted-foreground/50" />
                  <span className="text-xs">{t("live.waitingPlayer")}</span>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      </div>
    </div>
  );
}

function PartyConnector({
  player,
  position,
}: {
  player: PlayerStats;
  position: "single" | "first" | "middle" | "last";
}) {
  const connected = position !== "single";

  return (
    <span className="relative grid h-8 w-5 shrink-0 place-items-center">
      {connected && (
        <span
          className={cn(
            "absolute left-1/2 w-px -translate-x-1/2 bg-primary/60",
            position === "first" && "top-1/2 bottom-0",
            position === "middle" && "inset-y-0",
            position === "last" && "top-0 bottom-1/2",
          )}
        />
      )}
      <span
        className={cn(
          "relative h-2.5 w-2.5 rounded-full border-2 bg-card",
          connected ? "border-primary" : teamClass(player.team),
        )}
      />
    </span>
  );
}

function SkinRow({ onVideo, skin }: { onVideo: (video?: string) => void; skin: SkinView }) {
  const { t } = useTranslation();
  return (
    <button
      className="flex items-center gap-3 rounded-md border bg-background/60 p-3 text-left transition-colors hover:border-primary/60"
      onClick={() => onVideo(skin.video)}
      type="button"
    >
      <div className="flex h-16 w-24 shrink-0 items-center justify-center rounded-md bg-secondary/50">
        {skin.icon ? (
          <img alt="" className="max-h-full max-w-full object-contain" src={skin.icon} />
        ) : (
          <Sparkles className="h-5 w-5 text-muted-foreground" />
        )}
      </div>
      <div className="min-w-0 flex-1">
        <p className="truncate text-xs text-muted-foreground">{skin.weaponName}</p>
        <p className="truncate text-sm font-semibold">{skin.skinName}</p>
        <p className="truncate text-xs text-muted-foreground">
          {skin.levelName || t("common.default")} / {skin.chromaName || t("common.default")}
        </p>
      </div>
      {skin.video && (
        <span className="inline-flex items-center gap-1 text-xs text-primary">
          <Play className="h-4 w-4" />
          {t("common.video")}
        </span>
      )}
    </button>
  );
}

function InfoBox({ danger, text }: { danger?: boolean; text: string }) {
  return (
    <div
      className={
        danger
          ? "flex gap-3 rounded-md border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive"
          : "flex gap-3 rounded-md border bg-background/60 p-3 text-sm text-muted-foreground"
      }
    >
      <AlertCircle className="mt-0.5 h-4 w-4 shrink-0" />
      <span className="min-w-0 break-words">{text}</span>
    </div>
  );
}

function teamClass(team: PlayerStats["team"]) {
  if (team === "red") return "border-red-400 text-red-400";
  if (team === "blue") return "border-blue-400 text-blue-400";
  return "border-muted-foreground text-muted-foreground";
}

function splitTeams(stats: PlayerStats[]) {
  const blue = stats.filter((player) => player.team === "blue");
  const red = stats.filter((player) => player.team === "red");
  const unknown = stats.filter((player) => player.team === "unknown");

  for (const player of unknown) {
    if (blue.length <= red.length) {
      blue.push({ ...player, team: "blue" });
    } else {
      red.push({ ...player, team: "red" });
    }
  }

  return {
    blue: orderTeam(blue).slice(0, 5),
    red: orderTeam(red).slice(0, 5),
  };
}

function orderTeam(players: PlayerStats[]) {
  return [...players].sort((a, b) => {
    if (a.isMe !== b.isMe) return a.isMe ? -1 : 1;
    if (a.partyId && b.partyId && a.partyId !== b.partyId) return a.partyId.localeCompare(b.partyId);
    if (a.partyId !== b.partyId) return a.partyId ? -1 : 1;
    return a.displayName.localeCompare(b.displayName);
  });
}

function partyPosition(stats: PlayerStats[], player: PlayerStats, index: number) {
  if (!player.partyId) return "single";
  const partyIndexes = stats
    .map((candidate, candidateIndex) => ({ candidate, candidateIndex }))
    .filter(({ candidate }) => candidate.partyId === player.partyId)
    .map(({ candidateIndex }) => candidateIndex);

  if (partyIndexes.length <= 1) return "single";
  if (index === partyIndexes[0]) return "first";
  if (index === partyIndexes[partyIndexes.length - 1]) return "last";
  return "middle";
}

function formatNumber(value: number | null, digits: number) {
  return value === null ? "--" : value.toFixed(digits);
}

function errorMessage(error: unknown) {
  if (!error || typeof error !== "object") return String(error);
  const candidate = error as { message?: string };
  return candidate.message ?? String(error);
}

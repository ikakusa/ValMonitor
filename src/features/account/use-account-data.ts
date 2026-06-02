import { useQuery } from "@tanstack/react-query";

import { RiotAPI } from "@/services/tauri/riot";
import { ValorantAssetsAPI } from "@/services/valorant/assets";
import { useSettingsStore } from "@/stores/settings-store";
import type {
  CompetitiveTier,
  DashboardData,
  PvpMmrResponse,
  RankView,
  Season,
} from "@/types/riot";

export function useAccountData() {
  const language = useSettingsStore((state) => state.language);
  const refreshInterval = useSettingsStore((state) => state.refreshInterval);
  const connectionRefreshInterval = useSettingsStore((state) => state.connectionRefreshInterval);
  const rankRefreshInterval = useSettingsStore((state) => state.rankRefreshInterval);

  const initialized = useQuery({
    queryKey: ["riot", "initialized"],
    queryFn: RiotAPI.isInitialized,
    refetchInterval: connectionRefreshInterval,
    refetchOnWindowFocus: false,
  });

  const userInfo = useQuery({
    queryKey: ["riot", "userinfo"],
    queryFn: RiotAPI.getUserInfo,
    enabled: initialized.data === true,
    refetchInterval: rankRefreshInterval,
    refetchOnWindowFocus: false,
  });

  const privatePresence = useQuery({
    queryKey: ["riot", "private-presence"],
    queryFn: RiotAPI.getPrivatePresence,
    enabled: initialized.data === true,
    refetchInterval: refreshInterval,
    refetchOnWindowFocus: false,
  });

  const fullName = useQuery({
    queryKey: ["riot", "full-name"],
    queryFn: RiotAPI.getFullUsername,
    enabled: initialized.data === true,
    refetchInterval: rankRefreshInterval,
    refetchOnWindowFocus: false,
  });

  const puuid = useQuery({
    queryKey: ["riot", "puuid"],
    queryFn: RiotAPI.getPuuid,
    enabled: initialized.data === true,
    refetchInterval: rankRefreshInterval,
    refetchOnWindowFocus: false,
  });

  const region = useQuery({
    queryKey: ["riot", "region"],
    queryFn: RiotAPI.getRegion,
    enabled: initialized.data === true,
    refetchInterval: rankRefreshInterval,
    refetchOnWindowFocus: false,
  });

  const cardId = privatePresence.data?.playerPresenceData?.playerCardId;
  const card = useQuery({
    queryKey: ["valorant", "player-card", cardId],
    queryFn: () => RiotAPI.getPlayerCardById(cardId ?? ""),
    enabled: Boolean(cardId),
    staleTime: 30 * 60 * 1000,
  });

  const tiers = useQuery({
    queryKey: ["valorant", "competitive-tiers", language],
    queryFn: () => ValorantAssetsAPI.getCompetitiveTiers(language),
    enabled: initialized.data === true,
    staleTime: 60 * 60 * 1000,
  });

  const seasons = useQuery({
    queryKey: ["valorant", "seasons"],
    queryFn: ValorantAssetsAPI.getSeasons,
    enabled: initialized.data === true,
    staleTime: 60 * 60 * 1000,
  });

  const currentSeason = findCurrentSeason(seasons.data);
  const mmr = useQuery({
    queryKey: ["riot", "mmr", puuid.data],
    queryFn: () => RiotAPI.getPlayerMmr(puuid.data ?? ""),
    enabled: Boolean(puuid.data && tiers.data?.length && currentSeason),
    refetchInterval: rankRefreshInterval,
    refetchOnWindowFocus: false,
  });

  const peakSeasonId = getPeakSeasonId(mmr.data);
  const peakSeason = useQuery({
    queryKey: ["valorant", "season", peakSeasonId],
    queryFn: () => ValorantAssetsAPI.getSeason(peakSeasonId ?? ""),
    enabled: Boolean(peakSeasonId),
    staleTime: 60 * 60 * 1000,
  });

  const ranks = buildRanks(mmr.data, tiers.data, currentSeason, peakSeason.data, language);

  const visibleError =
    initialized.error ??
    privatePresence.error ??
    userInfo.error ??
    puuid.error ??
    region.error;
  const data: DashboardData = {
    userInfo: userInfo.data,
    privatePresence: privatePresence.data,
    fullName: fullName.data,
    puuid: puuid.data,
    region: region.data,
    card: card.data,
    gameState: dataGameState(privatePresence.data),
    accountLevel: privatePresence.data?.playerPresenceData?.accountLevel ?? 0,
    currentSeason,
    ...ranks,
  };

  return {
    data,
    initialized: initialized.data === true,
    loading:
      initialized.isLoading ||
      (initialized.data === true && privatePresence.isLoading),
    error: isExpectedStartupError(visibleError) ? undefined : visibleError,
  };
}

function isExpectedStartupError(error: unknown) {
  if (!error || typeof error !== "object") return false;
  const candidate = error as { kind?: string; message?: string };
  return (
    candidate.kind === "riotSessionNotReady" ||
    candidate.message?.includes("private presence is missing") ||
    candidate.message?.includes("my presence was not found")
  );
}

function dataGameState(privatePresence?: DashboardData["privatePresence"]) {
  return privatePresence?.matchPresenceData?.sessionLoopState ?? "IDLE";
}

function findCurrentSeason(seasons?: Season[]) {
  if (!seasons) return undefined;
  const now = Date.now();
  return seasons.find((season) => {
    const start = new Date(season.startTime).getTime();
    const end = new Date(season.endTime).getTime();
    return now >= start && now <= end && Boolean(season.title);
  });
}

function buildRanks(
  mmr?: PvpMmrResponse,
  tiers?: CompetitiveTier[],
  currentSeason?: Season,
  peakSeason?: Season,
  language: "ja" | "en" = "en",
): Pick<DashboardData, "currentRank" | "currentSeasonPeak" | "peakRank"> {
  const seasons = mmr?.QueueSkills?.competitive?.SeasonalInfoBySeasonID;
  if (!seasons || !tiers?.length || !currentSeason) {
    const unranked = unrankedRank(language);
    return { currentRank: unranked, currentSeasonPeak: unranked, peakRank: unranked };
  }

  const currentCompetitive = seasons[currentSeason.uuid];
  const peakEntry = Object.entries(seasons).sort(
    ([, a], [, b]) => (b.CompetitiveTier ?? 0) - (a.CompetitiveTier ?? 0),
  )[0];

  const currentTier = currentCompetitive?.CompetitiveTier ?? 0;
  const currentPeakTier = currentCompetitive?.Rank ?? 0;
  const [peakSeasonId, peakRank] = peakEntry ?? [undefined, undefined];
  const peakTier = peakRank?.CompetitiveTier ?? 0;

  return {
    currentRank: rankView(tiers, currentTier, language, {
      rr: currentCompetitive?.RankedRating ?? 0,
      seasonId: currentSeason.uuid,
      seasonName: currentSeason.title,
    }),
    currentSeasonPeak: rankView(tiers, currentPeakTier, language, {
      seasonId: currentSeason.uuid,
      seasonName: currentSeason.title,
    }),
    peakRank: rankView(tiers, peakTier, language, {
      seasonId: peakSeasonId,
      seasonName: peakSeason?.title,
    }),
  };
}

function unrankedRank(language: "ja" | "en"): RankView {
  return { tier: 0, name: language === "ja" ? "ランクなし" : "Unranked" };
}

function getPeakSeasonId(mmr?: PvpMmrResponse) {
  const seasons = mmr?.QueueSkills?.competitive?.SeasonalInfoBySeasonID;
  if (!seasons) return undefined;
  return Object.entries(seasons).sort(
    ([, a], [, b]) => (b.CompetitiveTier ?? 0) - (a.CompetitiveTier ?? 0),
  )[0]?.[0];
}

function rankView(
  tiers: CompetitiveTier[],
  tier: number,
  language: "ja" | "en",
  extra: Partial<RankView> = {},
): RankView {
  const rank = tiers[tier];
  return {
    tier,
    name: rank?.tierName ?? unrankedRank(language).name,
    icon: rank?.largeIcon,
    ...extra,
  };
}

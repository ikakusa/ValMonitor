import { useEffect, useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";

import { buildSkinViews, type SkinView } from "@/features/loadout/skin-utils";
import { HenrikAPI } from "@/services/tauri/henrik";
import { RiotAPI } from "@/services/tauri/riot";
import { ValorantAssetsAPI } from "@/services/valorant/assets";
import { useSettingsStore } from "@/stores/settings-store";
import type {
  HenrikAccountResponse,
  HenrikMatchesResponse,
  HenrikMatchData,
  HenrikMatchPlayer,
  HenrikMmrRankEntry,
  HenrikMmrResponse,
} from "@/types/henrik";
import type {
  AgentAsset,
  CompetitiveTier,
  CurrentMatchResponse,
  CurrentMatchPlayer,
  PresenceEntry,
  PrivatePresence,
} from "@/types/riot";

export type LiveTeam = "red" | "blue" | "unknown";

export type PlayerStats = {
  puuid: string;
  displayName: string;
  team: LiveTeam;
  partyId?: string;
  characterId?: string;
  characterName?: string;
  characterIcon?: string;
  isMe: boolean;
  kd: number | null;
  acs: number | null;
  winRate: number | null;
  hsPercent: number | null;
  adr: number | null;
  totalMatches: number;
  currentTier: number;
  rankName: string;
  rankIcon?: string;
  peakTier: number;
  peakRankName: string;
  peakRankIcon?: string;
  peakSeasonName?: string;
};

export function useLiveViewData() {
  const connectionRefreshInterval = useSettingsStore((state) => state.connectionRefreshInterval);
  const language = useSettingsStore((state) => state.language);
  const refreshInterval = useSettingsStore((state) => state.refreshInterval);
  const rankRefreshInterval = useSettingsStore((state) => state.rankRefreshInterval);
  const [selectedPlayerPuuid, setSelectedPlayerPuuid] = useState("");
  const [cachedPlayers, setCachedPlayers] = useState<PlayerStats[]>([]);
  const [cachedStats, setCachedStats] = useState<PlayerStats[]>([]);
  const [activeMatchKey, setActiveMatchKey] = useState("");

  const initialized = useQuery({
    queryKey: ["riot", "initialized"],
    queryFn: RiotAPI.isInitialized,
    refetchInterval: connectionRefreshInterval,
    refetchOnWindowFocus: false,
  });

  const myPuuid = useQuery({
    queryKey: ["riot", "puuid"],
    queryFn: RiotAPI.getPuuid,
    enabled: initialized.data === true,
    staleTime: 5 * 60 * 1000,
    refetchOnWindowFocus: false,
  });

  const region = useQuery({
    queryKey: ["riot", "region"],
    queryFn: RiotAPI.getRegion,
    enabled: initialized.data === true,
    staleTime: 5 * 60 * 1000,
    refetchOnWindowFocus: false,
  });

  const fullName = useQuery({
    queryKey: ["riot", "full-name"],
    queryFn: RiotAPI.getFullUsername,
    enabled: initialized.data === true,
    staleTime: 5 * 60 * 1000,
    refetchOnWindowFocus: false,
  });

  const privatePresence = useQuery({
    queryKey: ["riot", "private-presence"],
    queryFn: RiotAPI.getPrivatePresence,
    enabled: initialized.data === true,
    refetchInterval: refreshInterval,
    refetchOnWindowFocus: false,
  });

  const currentMatch = useQuery({
    queryKey: ["riot", "current-match", myPuuid.data],
    queryFn: () => RiotAPI.getCurrentMatch(myPuuid.data ?? ""),
    enabled: initialized.data === true && Boolean(myPuuid.data),
    refetchInterval: refreshInterval,
    refetchOnWindowFocus: false,
    // current match endpoint はラウンド遷移などで瞬間的に 404/empty になりやすい。
    // 前回成功値を保持して、Live UI が空状態へちらつかないようにする。
    placeholderData: (previous) => previous,
  });

  const allPresences = useQuery({
    queryKey: ["riot", "all-presences"],
    queryFn: RiotAPI.getAllPresences,
    enabled: initialized.data === true,
    refetchInterval: refreshInterval,
    refetchOnWindowFocus: false,
    placeholderData: (previous) => previous,
  });

  const henrikSettings = useQuery({
    queryKey: ["henrik", "settings"],
    queryFn: HenrikAPI.getSettings,
    staleTime: 30 * 1000,
  });

  const players = useMemo(
    () => buildPlayers(currentMatch.data, allPresences.data, myPuuid.data, fullName.data, privatePresence.data, language),
    [allPresences.data, currentMatch.data, fullName.data, language, myPuuid.data, privatePresence.data],
  );
  const matchKey = useMemo(() => currentMatchKey(currentMatch.data), [currentMatch.data]);

  useEffect(() => {
    if (!initialized.data) {
      setCachedPlayers([]);
      setCachedStats([]);
      setActiveMatchKey("");
      return;
    }
    if (players.length === 0) return;

    const currentPuuids = new Set(players.map((player) => player.puuid));

    if (matchKey && matchKey !== activeMatchKey) {
      // MatchID が変わったら前試合の roster を残さない。
      // ここを単純な merge にすると、次の試合へ入るたびに過去の 10 人が増え続けて Live View が壊れる。
      setActiveMatchKey(matchKey);
      setCachedPlayers(players);
      setCachedStats((current) => keepPlayers(current, currentPuuids));
      return;
    }

    setCachedPlayers((current) => mergePlayers(keepPlayers(current, currentPuuids), players));
    setCachedStats((current) => keepPlayers(current, currentPuuids));
  }, [activeMatchKey, initialized.data, matchKey, players]);

  useEffect(() => {
    if (!selectedPlayerPuuid && myPuuid.data) {
      setSelectedPlayerPuuid(myPuuid.data);
    }
  }, [myPuuid.data, selectedPlayerPuuid]);

  useEffect(() => {
    if (cachedPlayers.length === 0) return;
    if (selectedPlayerPuuid && cachedPlayers.some((player) => player.puuid === selectedPlayerPuuid)) return;

    const fallback = cachedPlayers.find((player) => player.puuid === myPuuid.data) ?? cachedPlayers[0];
    setSelectedPlayerPuuid(fallback.puuid);
  }, [cachedPlayers, myPuuid.data, selectedPlayerPuuid]);

  const playerStats = useQuery({
    queryKey: ["live", "player-stats", cachedPlayers.map((player) => player.puuid).join(","), region.data, language],
    queryFn: () => fetchPlayerStats(cachedPlayers, region.data ?? "ap", henrikSettings.data?.hasApiKey === true, language),
    enabled: Boolean(henrikSettings.data?.hasApiKey && cachedPlayers.length && region.data),
    staleTime: 60 * 1000,
    refetchInterval: rankRefreshInterval,
    refetchOnWindowFocus: false,
    placeholderData: (previous) => previous,
  });

  const selectedLoadout = useQuery({
    queryKey: ["riot", "loadout", selectedPlayerPuuid],
    queryFn: () => RiotAPI.getPlayerLoadout(selectedPlayerPuuid),
    enabled: initialized.data === true && Boolean(selectedPlayerPuuid),
    staleTime: 30 * 1000,
    refetchOnWindowFocus: false,
  });

  const weapons = useQuery({
    queryKey: ["valorant", "weapons", language],
    queryFn: () => ValorantAssetsAPI.getWeapons(language),
    enabled: Boolean(selectedLoadout.data?.Guns?.length),
    staleTime: 24 * 60 * 60 * 1000,
    refetchOnWindowFocus: false,
  });

  const agents = useQuery({
    queryKey: ["valorant", "agents", language],
    queryFn: () => ValorantAssetsAPI.getAgents(language),
    enabled: cachedPlayers.some((player) => Boolean(player.characterId)),
    staleTime: 24 * 60 * 60 * 1000,
    refetchOnWindowFocus: false,
  });

  const competitiveTiers = useQuery({
    queryKey: ["valorant", "competitive-tiers", language],
    queryFn: () => ValorantAssetsAPI.getCompetitiveTiers(language),
    enabled: cachedPlayers.length > 0,
    staleTime: 60 * 60 * 1000,
    refetchOnWindowFocus: false,
  });

  useEffect(() => {
    if (playerStats.data?.length) {
      setCachedStats((current) => mergePlayers(current, playerStats.data ?? []));
    }
  }, [playerStats.data]);

  const statsBase = cachedStats.length ? mergePlayers(cachedPlayers, cachedStats) : cachedPlayers.map(emptyStats);
  const stats = useMemo(
    () => enrichPlayerAssets(statsBase, agents.data, competitiveTiers.data),
    [agents.data, competitiveTiers.data, statsBase],
  );
  const skins = useMemo(
    () => buildSkinViews(selectedLoadout.data?.Guns, weapons.data, loadoutLabels(language)),
    [language, selectedLoadout.data?.Guns, weapons.data],
  );

  return {
    initialized: initialized.data === true,
    gameState: dataGameState(privatePresence.data),
    inMatch:
      isMatchState(dataGameState(privatePresence.data)) ||
      Boolean(currentMatch.data) ||
      cachedPlayers.length > 0,
    hasHenrikApiKey: henrikSettings.data?.hasApiKey === true,
    players: cachedPlayers,
    stats,
    skins,
    selectedPlayerPuuid,
    setSelectedPlayerPuuid,
    loading:
      initialized.isLoading ||
      (initialized.data === true &&
        cachedPlayers.length === 0 &&
        (allPresences.isLoading || privatePresence.isLoading || currentMatch.isLoading)),
    statsLoading: playerStats.isLoading && cachedStats.length === 0,
    skinsLoading: selectedLoadout.isLoading || weapons.isLoading,
    errors: {
      riot: firstMeaningfulError([initialized.error, allPresences.error, privatePresence.error, selectedLoadout.error]),
      henrik: playerStats.error,
      assets: firstMeaningfulError([weapons.error, agents.error, competitiveTiers.error]),
    },
  };
}

async function fetchPlayerStats(
  players: PlayerStats[],
  region: string,
  hasApiKey: boolean,
  language: "ja" | "en",
) {
  if (!hasApiKey) return players.map(emptyStats);

  const settled = await Promise.allSettled(
    players.map(async (player) => {
      const [account, matches, mmr] = await Promise.allSettled([
        HenrikAPI.accountByPuuid({ puuid: player.puuid }) as Promise<HenrikAccountResponse>,
        HenrikAPI.matchesByPuuid({ region, platform: "pc", puuid: player.puuid, size: 8 }) as Promise<HenrikMatchesResponse>,
        HenrikAPI.mmrByPuuid({ region, platform: "pc", puuid: player.puuid }) as Promise<HenrikMmrResponse>,
      ]);
      const accountData = account.status === "fulfilled" ? account.value : undefined;
      const matchData = matches.status === "fulfilled" ? matches.value.data ?? [] : [];
      const mmrData = mmr.status === "fulfilled" ? mmr.value : undefined;
      return calcStatsFromMatches(matchData, player, mmrData, accountData, language);
    }),
  );

  return settled.map((result, index) =>
    result.status === "fulfilled" ? result.value : emptyStats(players[index]),
  );
}

function calcStatsFromMatches(
  matches: HenrikMatchData[],
  player: PlayerStats,
  mmr?: HenrikMmrResponse,
  account?: HenrikAccountResponse,
  language: "ja" | "en" = "en",
): PlayerStats {
  let kills = 0;
  let deaths = 0;
  let score = 0;
  let rounds = 0;
  let wins = 0;
  let headshots = 0;
  let bodyshots = 0;
  let legshots = 0;
  let damage = 0;
  let countedMatches = 0;
  const accountDisplayName = displayNameFromAccount(account);
  let displayName = accountDisplayName ?? player.displayName;
  let fallbackTierId = player.currentTier;
  let fallbackRankName = player.rankName;
  let partyId = player.partyId;

  for (const match of matches) {
    const entry = findMatchPlayer(match, player.puuid);
    if (!entry) continue;

    if (!accountDisplayName && entry.name && entry.tag) {
      displayName = `${entry.name}#${entry.tag}`;
    }
    countedMatches += 1;
    const stats = entry.stats ?? {};
    kills += stats.kills ?? 0;
    deaths += stats.deaths ?? 0;
    score += stats.score ?? 0;
    headshots += stats.headshots ?? 0;
    bodyshots += stats.bodyshots ?? 0;
    legshots += stats.legshots ?? 0;
    damage += damageMade(entry);
    rounds += roundsPlayed(match);
    fallbackTierId = entry.tier?.id ?? entry.currenttier ?? fallbackTierId;
    fallbackRankName = entry.tier?.name ?? entry.currenttier_patched ?? fallbackRankName;
    partyId = entry.party_id ?? partyId;
    if (didWin(match, entry)) wins += 1;
  }

  const shots = headshots + bodyshots + legshots;
  const rank = mmr?.data?.current;
  const peak = peakRankFromMmr(mmr, language);
  return {
    ...player,
    displayName,
    kd: countedMatches ? kills / Math.max(deaths, 1) : null,
    acs: rounds ? score / rounds : null,
    winRate: countedMatches ? (wins / countedMatches) * 100 : null,
    hsPercent: shots ? (headshots / shots) * 100 : null,
    adr: rounds ? damage / rounds : null,
    totalMatches: countedMatches,
    currentTier: rank?.tier?.id ?? fallbackTierId,
    rankName: rank?.tier?.name ?? fallbackRankName,
    rankIcon: rank?.images?.large ?? player.rankIcon,
    peakTier: peak?.tier ?? player.peakTier,
    peakRankName: peak?.name ?? player.peakRankName,
    peakRankIcon: peak?.icon ?? player.peakRankIcon,
    peakSeasonName: peak?.seasonName ?? player.peakSeasonName,
    partyId,
  };
}

function displayNameFromAccount(account?: HenrikAccountResponse) {
  const data = account?.data;
  const name = data?.name ?? data?.gameName;
  const tag = data?.tag ?? data?.tagLine;
  return name && tag ? `${name}#${tag}` : undefined;
}

function peakRankFromMmr(mmr: HenrikMmrResponse | undefined, language: "ja" | "en") {
  const peak = rankEntryView(mmr?.data?.peak, language);
  if (peak) return peak;

  const seasonal = mmr?.data?.seasonal ?? [];
  return seasonal
    .map((entry) => rankEntryView(entry, language))
    .filter((entry): entry is NonNullable<ReturnType<typeof rankEntryView>> => Boolean(entry))
    .sort((a, b) => b.tier - a.tier)[0];
}

function rankEntryView(entry: HenrikMmrRankEntry | undefined, language: "ja" | "en") {
  if (!entry) return undefined;
  const tier = entry.tier?.id ?? numberField(entry, "tier") ?? numberField(entry, "rank") ?? 0;
  if (!tier) return undefined;

  return {
    tier,
    name:
      entry.tier?.name ??
      stringField(entry, "currenttier_patched") ??
      stringField(entry, "rank_name") ??
      defaultRankName(language),
    icon: entry.images?.large ?? entry.images?.small,
    seasonName: seasonLabel(entry),
  };
}

function seasonLabel(entry: HenrikMmrRankEntry) {
  const season = entry.season ?? entry.act;
  return (
    season?.short ??
    season?.name ??
    stringField(entry, "season_short") ??
    stringField(entry, "season_name") ??
    stringField(entry, "season_id")
  );
}

function findMatchPlayer(match: HenrikMatchData, puuid: string) {
  return matchPlayers(match).find((player) => player.puuid === puuid);
}

function didWin(match: HenrikMatchData, player: HenrikMatchPlayer) {
  const team = normalizeTeamId(player.team ?? player.team_id);
  if (!team) return false;

  if (Array.isArray(match.teams)) {
    return match.teams.find((candidate) => normalizeTeamId(candidate.team_id) === team)?.won === true;
  }

  if (team === "red") return match.teams?.red?.has_won === true;
  if (team === "blue") return match.teams?.blue?.has_won === true;
  return false;
}

function matchPlayers(match: HenrikMatchData): HenrikMatchPlayer[] {
  if (Array.isArray(match.players)) return match.players;
  return match.players?.all_players ?? [
    ...(match.players?.red ?? []),
    ...(match.players?.blue ?? []),
  ];
}

function damageMade(player: HenrikMatchPlayer) {
  return (
    player.damage_made ??
    player.damage?.made ??
    player.damage?.dealt ??
    player.stats?.damage?.made ??
    player.stats?.damage?.dealt ??
    0
  );
}

function roundsPlayed(match: HenrikMatchData) {
  return match.metadata?.rounds_played ?? match.rounds?.length ?? 0;
}

function normalizeTeamId(team?: string): LiveTeam | undefined {
  const normalized = team?.toLowerCase();
  if (!normalized) return undefined;
  if (normalized.includes("red")) return "red";
  if (normalized.includes("blue")) return "blue";
  return undefined;
}

function buildPlayers(
  currentMatch?: CurrentMatchResponse,
  presences?: PresenceEntry[],
  myPuuid?: string,
  myName?: string,
  privatePresence?: PrivatePresence,
  language: "ja" | "en" = "en",
): PlayerStats[] {
  const matchPlayers = buildPlayersFromCurrentMatch(currentMatch, myPuuid, myName, language);
  if (matchPlayers.length > 0) return matchPlayers;

  const myPrivate = privatePresence?.matchPresenceData;
  const rawPlayers = (presences ?? []).filter((presence) => presence.puuid);
  const unique = new Map<string, PresenceEntry>();
  for (const presence of rawPlayers) {
    unique.set(presence.puuid, presence);
  }
  if (myPuuid && !unique.has(myPuuid)) {
    unique.set(myPuuid, { puuid: myPuuid });
  }

  return Array.from(unique.values())
    .slice(0, 10)
    .map((presence, index) => ({
      puuid: presence.puuid,
      displayName: presence.puuid === myPuuid ? myName ?? defaultSelfName(language) : displayNameFromPresence(presence, index, language),
      team: teamFromPresence(presence, myPuuid, myPrivate),
      isMe: presence.puuid === myPuuid,
      kd: null,
      acs: null,
      winRate: null,
      hsPercent: null,
      adr: null,
      totalMatches: 0,
      currentTier: 0,
      rankName: defaultRankName(language),
      peakTier: 0,
      peakRankName: defaultRankName(language),
    }));
}

function currentMatchKey(currentMatch?: CurrentMatchResponse) {
  return currentMatch?.match?.MatchID ?? currentMatch?.player?.MatchID ?? "";
}

function buildPlayersFromCurrentMatch(
  currentMatch?: CurrentMatchResponse,
  myPuuid?: string,
  myName?: string,
  language: "ja" | "en" = "en",
): PlayerStats[] {
  const players = extractCurrentMatchPlayers(currentMatch);
  if (!players.length) return [];

  return players
    .map((entry, index) => {
      const puuid = entry.player.Subject ?? "";
      return {
        puuid,
        displayName: puuid === myPuuid ? myName ?? defaultSelfName(language) : defaultPlayerName(index, language),
        team: entry.team,
        partyId: entry.player.PartyID,
        isMe: puuid === myPuuid,
        kd: null,
        acs: null,
        winRate: null,
        hsPercent: null,
        adr: null,
        totalMatches: 0,
        currentTier: 0,
        rankName: defaultRankName(language),
        peakTier: 0,
        peakRankName: defaultRankName(language),
        characterId: entry.player.CharacterID,
      };
    })
    .filter((player) => player.puuid);
}

function defaultRankName(language: "ja" | "en") {
  return language === "ja" ? "ランクなし" : "Unranked";
}

function defaultSelfName(language: "ja" | "en") {
  return language === "ja" ? "自分" : "You";
}

function defaultPlayerName(index: number, language: "ja" | "en") {
  const number = index + 1;
  return language === "ja" ? `プレイヤー ${number}` : `Player ${number}`;
}

function loadoutLabels(language: "ja" | "en") {
  return language === "ja"
    ? {
        unknownOffer: "不明なオファー",
        unknownSkin: "不明なスキン",
        unknownWeapon: "武器",
      }
    : {
        unknownOffer: "Unknown Offer",
        unknownSkin: "Unknown Skin",
        unknownWeapon: "Weapon",
      };
}

function extractCurrentMatchPlayers(currentMatch?: CurrentMatchResponse): Array<{
  player: CurrentMatchPlayer;
  team: LiveTeam;
}> {
  const match = currentMatch?.match;
  if (!match) return [];

  if (match.Players?.length) {
    return match.Players.map((player) => ({
      player,
      team: normalizeTeam(player.TeamID),
    }));
  }

  return [
    ...(match.AllyTeam?.Players ?? []).map((player) => ({
      player,
      team: normalizeTeam(match.AllyTeam?.TeamID, "blue"),
    })),
    ...(match.EnemyTeam?.Players ?? []).map((player) => ({
      player,
      team: normalizeTeam(match.EnemyTeam?.TeamID, "red"),
    })),
  ];
}

function displayNameFromPresence(presence: PresenceEntry, index: number, language: "ja" | "en") {
  const gameName = stringField(presence, "game_name") ?? stringField(presence, "gameName");
  const tag = stringField(presence, "tag_line") ?? stringField(presence, "tagLine");
  if (gameName && tag) return `${gameName}#${tag}`;
  return defaultPlayerName(index, language);
}

function teamFromPresence(presence: PresenceEntry, myPuuid?: string, myPrivate?: PrivatePresence["matchPresenceData"]): LiveTeam {
  if (presence.puuid === myPuuid) return "blue";
  const team = stringField(presence, "team")?.toLowerCase();
  if (team === "red" || team === "blue") return team;
  return myPrivate?.sessionLoopState === "PREGAME" || myPrivate?.sessionLoopState === "INGAME" ? "unknown" : "unknown";
}

function normalizeTeam(team?: string, fallback: LiveTeam = "unknown"): LiveTeam {
  const normalized = team?.toLowerCase();
  if (normalized === "red" || normalized === "blue") return normalized;
  return fallback;
}

function emptyStats(player: PlayerStats): PlayerStats {
  return {
    ...player,
    kd: null,
    acs: null,
    winRate: null,
    hsPercent: null,
    adr: null,
    totalMatches: 0,
  };
}

function enrichPlayerAssets(players: PlayerStats[], agents?: AgentAsset[], tiers?: CompetitiveTier[]) {
  const agentsById = new Map((agents ?? []).map((agent) => [agent.uuid.toLowerCase(), agent]));
  const tiersById = new Map((tiers ?? []).map((tier) => [tier.tier, tier]));

  return players.map((player) => {
    const agent = player.characterId ? agentsById.get(player.characterId.toLowerCase()) : undefined;
    const currentTier = tiersById.get(player.currentTier);
    const peakTier = tiersById.get(player.peakTier);

    return {
      ...player,
      characterName: agent?.displayName ?? player.characterName,
      characterIcon: agent?.displayIcon ?? agent?.bustPortrait ?? player.characterIcon,
      rankName: currentTier?.tierName ?? player.rankName,
      rankIcon: player.rankIcon ?? currentTier?.largeIcon,
      peakRankName: peakTier?.tierName ?? player.peakRankName,
      peakRankIcon: player.peakRankIcon ?? peakTier?.largeIcon,
    };
  });
}

function mergePlayers(current: PlayerStats[], incoming: PlayerStats[]) {
  const byPuuid = new Map(current.map((player) => [player.puuid, player]));

  for (const next of incoming) {
    const previous = byPuuid.get(next.puuid);
    byPuuid.set(next.puuid, previous ? mergePlayer(previous, next) : next);
  }

  return Array.from(byPuuid.values()).sort((a, b) => {
    if (a.isMe !== b.isMe) return a.isMe ? -1 : 1;
    const teamDelta = teamSort(a.team) - teamSort(b.team);
    if (teamDelta !== 0) return teamDelta;
    return partySort(a.partyId) - partySort(b.partyId);
  });
}

function keepPlayers(players: PlayerStats[], allowedPuuids: Set<string>) {
  return players.filter((player) => allowedPuuids.has(player.puuid));
}

function mergePlayer(previous: PlayerStats, next: PlayerStats): PlayerStats {
  return {
    ...previous,
    ...next,
    displayName: preferDisplayName(previous.displayName, next.displayName),
    kd: next.kd ?? previous.kd,
    acs: next.acs ?? previous.acs,
    winRate: next.winRate ?? previous.winRate,
    hsPercent: next.hsPercent ?? previous.hsPercent,
    adr: next.adr ?? previous.adr,
    totalMatches: next.totalMatches || previous.totalMatches,
    currentTier: next.currentTier || previous.currentTier,
    rankName: !isDefaultRankName(next.rankName) ? next.rankName : previous.rankName,
    rankIcon: next.rankIcon ?? previous.rankIcon,
    peakTier: next.peakTier || previous.peakTier,
    peakRankName: !isDefaultRankName(next.peakRankName) ? next.peakRankName : previous.peakRankName,
    peakRankIcon: next.peakRankIcon ?? previous.peakRankIcon,
    peakSeasonName: next.peakSeasonName ?? previous.peakSeasonName,
    characterId: next.characterId ?? previous.characterId,
    characterName: next.characterName ?? previous.characterName,
    characterIcon: next.characterIcon ?? previous.characterIcon,
  };
}

function isDefaultRankName(value: string) {
  return value === "Unranked" || value === "ランクなし";
}

function preferDisplayName(previous: string, next: string) {
  if (/^(Player|プレイヤー) \d+$/.test(previous) && !/^(Player|プレイヤー) \d+$/.test(next)) return next;
  if ((previous === "You" || previous === "自分") && next.includes("#")) return next;
  return next || previous;
}

function teamSort(team: LiveTeam) {
  if (team === "blue") return 0;
  if (team === "red") return 1;
  return 2;
}

function partySort(partyId?: string) {
  if (!partyId) return Number.MAX_SAFE_INTEGER;
  let score = 0;
  for (const char of partyId) {
    score += char.charCodeAt(0);
  }
  return score;
}

function dataGameState(privatePresence?: PrivatePresence) {
  return privatePresence?.matchPresenceData?.sessionLoopState ?? "IDLE";
}

function isMatchState(gameState: string) {
  return gameState === "INGAME" || gameState === "PREGAME";
}

function stringField(value: Record<string, unknown>, key: string) {
  const field = value[key];
  return typeof field === "string" ? field : undefined;
}

function numberField(value: Record<string, unknown>, key: string) {
  const field = value[key];
  return typeof field === "number" ? field : undefined;
}

function firstMeaningfulError(errors: unknown[]) {
  return errors.find((error) => {
    if (!error || typeof error !== "object") return false;
    const candidate = error as { kind?: string };
    return candidate.kind !== "riotSessionNotReady" && candidate.kind !== "lockfileNotFound";
  });
}

export type { SkinView };

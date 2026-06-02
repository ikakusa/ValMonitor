export type HenrikRuntimeSettings = {
  settingsPath: string;
  apiKeyPath: string;
  debugLogPath: string;
  baseUrl: string;
  authMode: "header" | "query";
  hasApiKey: boolean;
};

export type HenrikAccountByNameRequest = {
  name: string;
  tag: string;
  force?: boolean;
};

export type HenrikAccountByPuuidRequest = {
  puuid: string;
  force?: boolean;
};

export type HenrikContentRequest = {
  locale?: string;
};

export type HenrikCrosshairRequest = {
  id?: string;
};

export type HenrikEsportsScheduleRequest = {
  region?: string;
  league?: string;
};

export type HenrikLeaderboardRequest = {
  region: string;
  platform: "pc" | "console" | string;
  season?: string;
  size?: number;
  page?: number;
  startIndex?: number;
  name?: string;
  tag?: string;
  puuid?: string;
};

export type HenrikMatchesByNameRequest = {
  region: string;
  platform: "pc" | "console" | string;
  name: string;
  tag: string;
  mode?: string;
  map?: string;
  size?: number;
  start?: number;
};

export type HenrikMatchesByPuuidRequest = {
  region: string;
  platform: "pc" | "console" | string;
  puuid: string;
  mode?: string;
  map?: string;
  size?: number;
  start?: number;
};

export type HenrikMatchRequest = {
  region: string;
  matchId: string;
};

export type HenrikMmrByNameRequest = {
  region: string;
  platform: "pc" | "console" | string;
  name: string;
  tag: string;
};

export type HenrikMmrByPuuidRequest = {
  region: string;
  platform: "pc" | "console" | string;
  puuid: string;
};

export type HenrikMmrHistoryByNameRequest = HenrikMmrByNameRequest;
export type HenrikMmrHistoryByPuuidRequest = HenrikMmrByPuuidRequest;

export type HenrikVlrEntityRequest = {
  id: string;
};

export type HenrikResponse = Record<string, unknown>;

export type HenrikAccountData = {
  puuid?: string;
  name?: string;
  tag?: string;
  gameName?: string;
  tagLine?: string;
  account_level?: number;
  accountLevel?: number;
  card?: {
    small?: string;
    large?: string;
    wide?: string;
    id?: string;
  };
  [key: string]: unknown;
};

export type HenrikAccountResponse = {
  status?: number;
  data?: HenrikAccountData;
  errors?: unknown[];
  error?: unknown;
  _rate_limits?: Record<string, string>;
};

export type HenrikMatchStats = {
  score?: number;
  kills?: number;
  deaths?: number;
  assists?: number;
  headshots?: number;
  bodyshots?: number;
  legshots?: number;
  damage?: {
    made?: number;
    dealt?: number;
    received?: number;
  };
};

export type HenrikMatchPlayer = {
  puuid: string;
  name?: string;
  tag?: string;
  team?: string;
  team_id?: string;
  currenttier?: number;
  currenttier_patched?: string;
  tier?: {
    id?: number;
    name?: string;
  };
  stats?: HenrikMatchStats;
  damage_made?: number;
  damage_received?: number;
  damage?: {
    made?: number;
    dealt?: number;
    received?: number;
  };
  party_id?: string;
};

export type HenrikMatchData = {
  metadata?: {
    matchid?: string;
    match_id?: string;
    rounds_played?: number;
    game_start?: number;
  };
  players?:
    | HenrikMatchPlayer[]
    | {
        red?: HenrikMatchPlayer[];
        blue?: HenrikMatchPlayer[];
        all_players?: HenrikMatchPlayer[];
      };
  teams?:
    | Array<{
        team_id?: string;
        won?: boolean;
        rounds?: { won?: number; lost?: number };
      }>
    | {
        red?: { has_won?: boolean; rounds_won?: number; rounds_lost?: number };
        blue?: { has_won?: boolean; rounds_won?: number; rounds_lost?: number };
      };
  rounds?: unknown[];
};

export type HenrikMatchesResponse = {
  status?: number;
  data?: HenrikMatchData[];
  _rate_limits?: Record<string, string>;
};

export type HenrikMmrResponse = {
  status?: number;
  data?: {
    current?: {
      tier?: { id?: number; name?: string };
      images?: { large?: string; small?: string };
    };
    peak?: HenrikMmrRankEntry;
    seasonal?: HenrikMmrRankEntry[];
  };
  _rate_limits?: Record<string, string>;
};

export type HenrikMmrRankEntry = {
  tier?: { id?: number; name?: string };
  images?: { large?: string; small?: string };
  rr?: number;
  end_rr?: number;
  season?: HenrikMmrSeason;
  act?: HenrikMmrSeason;
  season_id?: string;
  season_short?: string;
  season_name?: string;
  [key: string]: unknown;
};

export type HenrikMmrSeason = {
  id?: string;
  short?: string;
  name?: string;
  [key: string]: unknown;
};

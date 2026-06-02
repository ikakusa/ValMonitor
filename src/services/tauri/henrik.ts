import { callCommand } from "@/services/tauri/client";
import type {
  HenrikAccountByNameRequest,
  HenrikAccountByPuuidRequest,
  HenrikContentRequest,
  HenrikCrosshairRequest,
  HenrikEsportsScheduleRequest,
  HenrikLeaderboardRequest,
  HenrikMatchRequest,
  HenrikMatchesByNameRequest,
  HenrikMatchesByPuuidRequest,
  HenrikMmrByNameRequest,
  HenrikMmrByPuuidRequest,
  HenrikMmrHistoryByNameRequest,
  HenrikMmrHistoryByPuuidRequest,
  HenrikResponse,
  HenrikRuntimeSettings,
  HenrikVlrEntityRequest,
} from "@/types/henrik";

function withInput<TInput extends object>(input: TInput) {
  return { input };
}

export const HenrikAPI = {
  getSettings: () => callCommand<HenrikRuntimeSettings>("henrik_get_settings"),
  saveApiKey: (apiKey: string) =>
    callCommand<HenrikRuntimeSettings>("henrik_save_api_key", { apiKey }),

  accountByName: (input: HenrikAccountByNameRequest) =>
    callCommand<HenrikResponse>("henrik_account_by_name", withInput(input)),
  accountByPuuid: (input: HenrikAccountByPuuidRequest) =>
    callCommand<HenrikResponse>("henrik_account_by_puuid", withInput(input)),
  content: (input: HenrikContentRequest = {}) =>
    callCommand<HenrikResponse>("henrik_content", withInput(input)),
  crosshair: (input: HenrikCrosshairRequest = {}) =>
    callCommand<HenrikResponse>("henrik_crosshair", withInput(input)),
  esportsSchedule: (input: HenrikEsportsScheduleRequest = {}) =>
    callCommand<HenrikResponse>("henrik_esports_schedule", withInput(input)),
  leaderboard: (input: HenrikLeaderboardRequest) =>
    callCommand<HenrikResponse>("henrik_leaderboard", withInput(input)),

  matchesByName: (input: HenrikMatchesByNameRequest) =>
    callCommand<HenrikResponse>("henrik_matches_by_name", withInput(input)),
  matchesByPuuid: (input: HenrikMatchesByPuuidRequest) =>
    callCommand<HenrikResponse>("henrik_matches_by_puuid", withInput(input)),
  matchById: (input: HenrikMatchRequest) =>
    callCommand<HenrikResponse>("henrik_match_by_id", withInput(input)),

  mmrByName: (input: HenrikMmrByNameRequest) =>
    callCommand<HenrikResponse>("henrik_mmr_by_name", withInput(input)),
  mmrByPuuid: (input: HenrikMmrByPuuidRequest) =>
    callCommand<HenrikResponse>("henrik_mmr_by_puuid", withInput(input)),
  mmrHistoryByName: (input: HenrikMmrHistoryByNameRequest) =>
    callCommand<HenrikResponse>("henrik_mmr_history_by_name", withInput(input)),
  mmrHistoryByPuuid: (input: HenrikMmrHistoryByPuuidRequest) =>
    callCommand<HenrikResponse>("henrik_mmr_history_by_puuid", withInput(input)),

  vlrEvents: () => callCommand<HenrikResponse>("henrik_vlr_events"),
  vlrEventMatches: (input: HenrikVlrEntityRequest) =>
    callCommand<HenrikResponse>("henrik_vlr_event_matches", withInput(input)),
  vlrMatch: (input: HenrikVlrEntityRequest) =>
    callCommand<HenrikResponse>("henrik_vlr_match", withInput(input)),
  vlrTeam: (input: HenrikVlrEntityRequest) =>
    callCommand<HenrikResponse>("henrik_vlr_team", withInput(input)),
  vlrTeamMatches: (input: HenrikVlrEntityRequest) =>
    callCommand<HenrikResponse>("henrik_vlr_team_matches", withInput(input)),
  vlrTeamTransactions: (input: HenrikVlrEntityRequest) =>
    callCommand<HenrikResponse>("henrik_vlr_team_transactions", withInput(input)),
  vlrPlayer: (input: HenrikVlrEntityRequest) =>
    callCommand<HenrikResponse>("henrik_vlr_player", withInput(input)),
  vlrPlayerMatches: (input: HenrikVlrEntityRequest) =>
    callCommand<HenrikResponse>("henrik_vlr_player_matches", withInput(input)),
};

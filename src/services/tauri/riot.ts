import { callCommand } from "@/services/tauri/client";
import type {
  PlayerCardData,
  PlayerLoadoutResponse,
  PresenceEntry,
  PrivatePresence,
  PvpMmrResponse,
  StorefrontResponse,
  CurrentMatchResponse,
  UserInfoResponse,
} from "@/types/riot";

function parseJsonString<T>(value: string): T {
  return JSON.parse(value) as T;
}

export const RiotAPI = {
  async getUserInfo(): Promise<UserInfoResponse> {
    const json = await callCommand<string>("get_auth_userinfo");
    return parseJsonString<UserInfoResponse>(json);
  },
  getMyPresence: () => callCommand<PresenceEntry>("get_my_presence"),
  getAllPresences: () => callCommand<PresenceEntry[]>("get_all_presences"),
  getPrivatePresence: () => callCommand<PrivatePresence>("get_private_presence"),
  getGameState: () => callCommand<string>("get_gamestate"),
  isInitialized: () => callCommand<boolean>("is_api_initialized"),
  getFullUsername: () => callCommand<string>("get_full_username"),
  getPuuid: () => callCommand<string>("get_puuid"),
  getRegion: () => callCommand<string>("get_region"),
  getPlayerCardById: (id: string) => callCommand<PlayerCardData>("get_playercard_by_id", { id }),
  getPlayerMmr: (uid: string) => callCommand<PvpMmrResponse>("get_player_mmr", { uid }),
  getPlayerLoadout: (uid: string) => callCommand<PlayerLoadoutResponse>("get_player_loadout", { uid }),
  getStorefront: (uid: string) => callCommand<StorefrontResponse>("get_storefront", { uid }),
  getCurrentMatch: (uid: string) => callCommand<CurrentMatchResponse>("get_current_match", { uid }),
};

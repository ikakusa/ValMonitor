import type {
  AgentAsset,
  AgentsResponse,
  CompetitiveTier,
  CompetitiveTierResponse,
  Season,
  SeasonResponse,
  SeasonsResponse,
  WeaponAsset,
  WeaponsResponse,
} from "@/types/riot";

const VALORANT_API_BASE = "https://valorant-api.com/v1";
type AssetLanguage = "ja" | "en";

function locale(language: AssetLanguage) {
  return language === "ja" ? "ja-JP" : "en-US";
}

async function fetchJson<T>(path: string): Promise<T> {
  const started = performance.now();
  const response = await fetch(`${VALORANT_API_BASE}${path}`);
  const elapsed = Math.round(performance.now() - started);
  console.debug(`HTTP valorant-assets GET ${path} -> ${response.status} in ${elapsed}ms`);
  if (!response.ok) {
    throw new Error(`valorant-api.com request failed: ${response.status}`);
  }
  return (await response.json()) as T;
}

export const ValorantAssetsAPI = {
  async getAgents(language: AssetLanguage = "en"): Promise<AgentAsset[]> {
    const response = await fetchJson<AgentsResponse>(`/agents?isPlayableCharacter=true&language=${locale(language)}`);
    return response.data;
  },
  async getCompetitiveTiers(language: AssetLanguage = "en"): Promise<CompetitiveTier[]> {
    const response = await fetchJson<CompetitiveTierResponse>(`/competitivetiers?language=${locale(language)}`);
    return response.data.at(-1)?.tiers ?? [];
  },
  async getSeasons(): Promise<Season[]> {
    const response = await fetchJson<SeasonsResponse>("/seasons");
    return response.data;
  },
  async getSeason(id: string): Promise<Season> {
    const response = await fetchJson<SeasonResponse>(`/seasons/${id}`);
    return response.data;
  },
  async getWeapons(language: AssetLanguage = "en"): Promise<WeaponAsset[]> {
    const response = await fetchJson<WeaponsResponse>(`/weapons?language=${locale(language)}`);
    return response.data;
  },
};

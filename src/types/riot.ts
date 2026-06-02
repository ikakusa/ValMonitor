export type AppError = {
  kind: string;
  message: string;
};

export type AccountInfo = {
  game_name: string;
  tag_line: string;
};

export type UserInfoResponse = {
  acct: AccountInfo;
};

export type PresenceEntry = {
  puuid: string;
  private?: string;
  [key: string]: unknown;
};

export type PlayerPresenceData = {
  playerCardId?: string;
  accountLevel?: number;
};

export type MatchPresenceData = {
  sessionLoopState?: string;
};

export type PrivatePresence = {
  playerPresenceData?: PlayerPresenceData;
  matchPresenceData?: MatchPresenceData;
  [key: string]: unknown;
};

export type PlayerCardData = {
  uuid?: string;
  displayName?: string;
  displayIcon?: string;
  smallArt?: string;
  wideArt?: string;
  largeArt?: string;
  [key: string]: unknown;
};

export type SeasonalRankInfo = {
  CompetitiveTier?: number;
  RankedRating?: number;
  Rank?: number;
};

export type PvpMmrResponse = {
  QueueSkills?: {
    competitive?: {
      SeasonalInfoBySeasonID?: Record<string, SeasonalRankInfo>;
    };
  };
  [key: string]: unknown;
};

export type PlayerLoadoutGun = {
  ID: string;
  SkinID: string;
  SkinLevelID: string;
  ChromaID: string;
  CharmInstanceID?: string | null;
  CharmID?: string | null;
  CharmLevelID?: string | null;
  Attachments?: unknown[];
};

export type PlayerLoadoutResponse = {
  Subject: string;
  Version: number;
  Guns: PlayerLoadoutGun[];
};

export type StoreReward = {
  ItemTypeID: string;
  ItemID: string;
  Quantity: number;
};

export type StoreOffer = {
  OfferID: string;
  Cost?: Record<string, number>;
  Rewards?: StoreReward[];
};

export type StorefrontResponse = {
  FeaturedBundle?: unknown;
  SkinsPanelLayout?: {
    SingleItemOffers?: string[];
    SingleItemStoreOffers?: StoreOffer[];
    SingleItemOffersRemainingDurationInSeconds?: number;
  };
  BonusStore?: unknown;
};

export type CurrentMatchPlayer = {
  Subject?: string;
  TeamID?: string;
  PartyID?: string;
  CharacterID?: string;
  PlayerIdentity?: {
    AccountLevel?: number;
    PlayerCardID?: string;
    Incognito?: boolean;
    HideAccountLevel?: boolean;
  };
};

export type CurrentMatchResponse = {
  mode: "core-game" | "pregame";
  player?: {
    Subject?: string;
    MatchID?: string;
  };
  match?: {
    MatchID?: string;
    Players?: CurrentMatchPlayer[];
    AllyTeam?: {
      TeamID?: string;
      Players?: CurrentMatchPlayer[];
    };
    EnemyTeam?: {
      TeamID?: string;
      Players?: CurrentMatchPlayer[];
    };
  };
};

export type WeaponSkinLevel = {
  uuid: string;
  displayName?: string;
  displayIcon?: string;
  streamedVideo?: string;
};

export type WeaponSkinChroma = {
  uuid: string;
  displayName?: string;
  displayIcon?: string;
  fullRender?: string;
  swatch?: string;
};

export type WeaponSkin = {
  uuid: string;
  displayName: string;
  displayIcon?: string;
  contentTierUuid?: string;
  levels?: WeaponSkinLevel[];
  chromas?: WeaponSkinChroma[];
};

export type WeaponAsset = {
  uuid: string;
  displayName: string;
  displayIcon?: string;
  killStreamIcon?: string;
  skins: WeaponSkin[];
};

export type WeaponsResponse = {
  data: WeaponAsset[];
};

export type AgentAsset = {
  uuid: string;
  displayName: string;
  displayIcon?: string;
  bustPortrait?: string;
  fullPortrait?: string;
  isPlayableCharacter?: boolean;
};

export type AgentsResponse = {
  data: AgentAsset[];
};

export type CompetitiveTier = {
  tier: number;
  tierName: string;
  largeIcon?: string;
};

export type CompetitiveTierResponse = {
  data: Array<{
    tiers: CompetitiveTier[];
  }>;
};

export type Season = {
  uuid: string;
  title?: string;
  startTime: string;
  endTime: string;
};

export type SeasonsResponse = {
  data: Season[];
};

export type SeasonResponse = {
  data: Season;
};

export type RankView = {
  tier: number;
  name: string;
  icon?: string;
  rr?: number;
  seasonId?: string;
  seasonName?: string;
};

export type DashboardData = {
  userInfo?: UserInfoResponse;
  privatePresence?: PrivatePresence;
  fullName?: string;
  puuid?: string;
  region?: string;
  card?: PlayerCardData;
  gameState: string;
  accountLevel: number;
  currentSeason?: Season;
  currentRank: RankView;
  currentSeasonPeak: RankView;
  peakRank: RankView;
};

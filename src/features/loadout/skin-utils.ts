import type {
  PlayerLoadoutGun,
  StorefrontResponse,
  WeaponAsset,
  WeaponSkin,
  WeaponSkinChroma,
  WeaponSkinLevel,
} from "@/types/riot";

export type SkinView = {
  loadoutId: string;
  weaponId: string;
  weaponName: string;
  skinName: string;
  levelName?: string;
  chromaName?: string;
  icon?: string;
  swatch?: string;
  video?: string;
};

export type StoreItemView = {
  offerId: string;
  itemId: string;
  skinName: string;
  weaponName: string;
  icon?: string;
  video?: string;
  cost?: number;
};

type LoadoutLabels = {
  unknownWeapon: string;
  unknownSkin: string;
  unknownOffer: string;
};

const defaultLabels: LoadoutLabels = {
  unknownWeapon: "Weapon",
  unknownSkin: "Unknown Skin",
  unknownOffer: "Unknown Offer",
};

export function buildSkinViews(
  guns?: PlayerLoadoutGun[],
  weapons?: WeaponAsset[],
  labels: Partial<LoadoutLabels> = {},
): SkinView[] {
  if (!guns?.length || !weapons?.length) return [];

  const text = { ...defaultLabels, ...labels };
  const indexes = buildWeaponIndexes(weapons);
  return guns.map((gun) => {
    const skin = indexes.skinById.get(gun.SkinID);
    const weapon = indexes.weaponBySkinId.get(gun.SkinID);
    const level = skin?.levels?.find((candidate) => candidate.uuid === gun.SkinLevelID);
    const chroma = skin?.chromas?.find((candidate) => candidate.uuid === gun.ChromaID);

    return {
      loadoutId: `${gun.ID}:${gun.SkinID}`,
      weaponId: gun.ID,
      weaponName: weapon?.displayName ?? text.unknownWeapon,
      skinName: skin?.displayName ?? text.unknownSkin,
      levelName: cleanVariantName(level, skin),
      chromaName: cleanVariantName(chroma, skin),
      icon: chroma?.fullRender ?? chroma?.displayIcon ?? skin?.displayIcon ?? level?.displayIcon ?? weapon?.displayIcon,
      swatch: chroma?.swatch,
      video: level?.streamedVideo,
    };
  });
}

export function buildStoreItems(
  storefront?: StorefrontResponse,
  weapons?: WeaponAsset[],
  labels: Partial<LoadoutLabels> = {},
): StoreItemView[] {
  const layout = storefront?.SkinsPanelLayout;
  if (!layout || !weapons?.length) return [];

  const text = { ...defaultLabels, ...labels };
  const indexes = buildWeaponIndexes(weapons);
  const items = (layout.SingleItemStoreOffers ?? []).map((offer) => {
    const reward = offer.Rewards?.[0];
    const cost = Object.values(offer.Cost ?? {})[0];
    return buildStoreItemView({
      indexes,
      itemId: reward?.ItemID ?? offer.OfferID,
      offerId: offer.OfferID,
      cost,
      labels: text,
    });
  });

  const offeredItemIds = new Set(items.map((item) => item.itemId));
  const fallbackItems = (layout.SingleItemOffers ?? [])
    .filter((itemId) => !offeredItemIds.has(itemId))
    .map((itemId) =>
      buildStoreItemView({
        indexes,
        itemId,
        offerId: itemId,
        labels: text,
      }),
    );

  return [...items, ...fallbackItems];
}

function buildWeaponIndexes(weapons: WeaponAsset[]) {
  const skinById = new Map<string, WeaponSkin>();
  const skinByLevelId = new Map<string, WeaponSkin>();
  const weaponBySkinId = new Map<string, WeaponAsset>();

  for (const weapon of weapons) {
    for (const skin of weapon.skins) {
      skinById.set(skin.uuid, skin);
      weaponBySkinId.set(skin.uuid, weapon);
      for (const level of skin.levels ?? []) {
        skinByLevelId.set(level.uuid, skin);
      }
    }
  }

  return { skinById, skinByLevelId, weaponBySkinId };
}

function buildStoreItemView({
  cost,
  indexes,
  itemId,
  labels,
  offerId,
}: {
  cost?: number;
  indexes: ReturnType<typeof buildWeaponIndexes>;
  itemId: string;
  labels: LoadoutLabels;
  offerId: string;
}): StoreItemView {
  const skin = indexes.skinByLevelId.get(itemId) ?? indexes.skinById.get(itemId);
  const weapon = skin ? indexes.weaponBySkinId.get(skin.uuid) : undefined;
  const level = skin?.levels?.find((candidate) => candidate.uuid === itemId) ?? skin?.levels?.[0];

  return {
    offerId,
    itemId,
    skinName: skin?.displayName ?? labels.unknownOffer,
    weaponName: weapon?.displayName ?? labels.unknownWeapon,
    icon: skin?.displayIcon ?? level?.displayIcon ?? weapon?.displayIcon,
    video: level?.streamedVideo,
    cost,
  };
}

function cleanVariantName(
  variant: WeaponSkinLevel | WeaponSkinChroma | undefined,
  skin: WeaponSkin | undefined,
) {
  if (!variant?.displayName || !skin?.displayName) return variant?.displayName;
  return variant.displayName.replace(skin.displayName, "").replace(/^Level\s*/i, "Lv ").trim();
}

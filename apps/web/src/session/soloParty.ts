import type { Character, CharacterId, UserId } from "@tabifuda/ui";

// crates/tabifuda-cli/src/play.rs のソロキャラ構築を踏襲
// (domain-model.md「ソロMVPでの簡略化」。単一ユーザーがPlayer/GM両ロールを兼ねる)。
export const SOLO_ACTOR: UserId = "solo";
export const SOLO_CHARACTER_ID: CharacterId = "hunter";

export function createSoloCharacter(): Character {
  return {
    id: SOLO_CHARACTER_ID,
    name: "旅人",
    stats: {},
    deck: [],
  };
}

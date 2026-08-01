import type { Scenario } from "../core/bindings";
import raw from "../../../../shared/scenarios/simple-hunt.json";

// shared/scenarios/(docs/design/domain-model.md「シナリオファイルの配置」)。
// 特定crate/packageに従属しない共有配置のため、apps/webからは相対importで読む。
export const simpleHunt = raw as Scenario;

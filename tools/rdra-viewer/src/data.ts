// docs/rdra/ のYAMLをビルド時に取り込む(データの正は docs/rdra/。ここには置かない)
import actorsYaml from "../../../docs/rdra/actors.yaml?raw";
import usecasesYaml from "../../../docs/rdra/usecases.yaml?raw";
import { parseModel } from "./model";

export const model = parseModel(actorsYaml, usecasesYaml);

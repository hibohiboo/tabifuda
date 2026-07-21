// docs/rdra/ のYAMLをビルド時に取り込む(データの正は docs/rdra/。ここには置かない)
import actorsYaml from "../../../docs/rdra/actors.yaml?raw";
import usecasesYaml from "../../../docs/rdra/usecases.yaml?raw";
import informationYaml from "../../../docs/rdra/information.yaml?raw";
import statesYaml from "../../../docs/rdra/states.yaml?raw";
import requirementsYaml from "../../../docs/rdra/requirements.yaml?raw";
import businessFlowYaml from "../../../docs/rdra/business-flow.yaml?raw";
import { parseModel } from "./model";

export const model = parseModel({
  actors: actorsYaml,
  usecases: usecasesYaml,
  information: informationYaml,
  states: statesYaml,
  requirements: requirementsYaml,
  businessFlow: businessFlowYaml,
});

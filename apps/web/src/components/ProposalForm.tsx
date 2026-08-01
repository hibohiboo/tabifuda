import { FREE_TEXT_MAX } from "../session/limits";
import { FreeTextInput } from "./FreeTextInput";

export function ProposalForm({ onPropose }: { onPropose: (text: string) => void }) {
  return (
    <FreeTextInput
      maxLength={FREE_TEXT_MAX}
      placeholder="提案する内容(GMに相談したいこと)"
      submitLabel="提案する"
      onSubmit={onPropose}
    />
  );
}

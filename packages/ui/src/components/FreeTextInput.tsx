import { useState } from "react";
import "./FreeTextInput.css";

export function FreeTextInput({
  maxLength,
  placeholder,
  submitLabel,
  onSubmit,
  onCancel,
}: {
  maxLength: number;
  placeholder?: string;
  submitLabel: string;
  onSubmit: (text: string) => void;
  onCancel?: () => void;
}) {
  const [text, setText] = useState("");

  return (
    <div className="tf-free-text">
      <textarea
        className="tf-free-text__textarea"
        value={text}
        maxLength={maxLength}
        placeholder={placeholder}
        onChange={(event) => setText(event.target.value)}
      />
      <div className="tf-free-text__actions">
        <button type="button" onClick={() => onSubmit(text)}>
          {submitLabel}
        </button>
        {onCancel !== undefined && (
          <button type="button" onClick={onCancel}>
            キャンセル
          </button>
        )}
      </div>
    </div>
  );
}

import type React from "react";

import { SearchInput } from "@patternfly/react-core";

import { getString } from "@app/utils/utils";

import type { AnyAutocompleteOptionProps } from "./type-utils";

export interface SearchInputProps {
  id: string;
  placeholderText: string;
  searchInputAriaLabel: string;
  onSearchChange: (value: string) => void;
  onClear: () => void;
  onKeyHandling: (event: React.KeyboardEvent<HTMLInputElement>) => void;
  inputValue: string;
  inputRef: React.RefObject<HTMLDivElement>;
  options: AnyAutocompleteOptionProps[];
}

export const SearchInputComponent: React.FC<SearchInputProps> = ({
  id,
  placeholderText,
  searchInputAriaLabel,
  onSearchChange,
  onClear,
  onKeyHandling,
  options,
  inputValue,
  inputRef,
}) => {
  const getHint = (): string => {
    if (options.length === 0) {
      return "";
    }

    if (options.length === 1 && inputValue) {
      const fullHint = getString(options[0].name);

      if (fullHint.toLowerCase().indexOf(inputValue.toLowerCase()) === 0) {
        return inputValue + fullHint.substring(inputValue.length);
      }
    }

    return "";
  };

  const hint = getHint();

  return (
    <div ref={inputRef}>
      <SearchInput
        id={id}
        value={inputValue}
        hint={hint}
        onChange={(_event, value) => onSearchChange(value)}
        onClear={onClear}
        onFocus={() => onKeyHandling(event as any)}
        onKeyDown={onKeyHandling}
        placeholder={placeholderText}
        aria-label={searchInputAriaLabel}
      />
    </div>
  );
};

import type { Control, FieldValues, Path } from "react-hook-form";

import { HookFormPFGroupController } from "@app/components/HookFormPFFields";

import {
  Autocomplete,
  type AutocompleteOptionProps,
} from "../Autocomplete/Autocomplete";
import {
  GroupedAutocomplete,
  type GroupedAutocompleteOptionProps,
} from "../Autocomplete/GroupedAutocomplete";

// TODO: Does not support select menu selection checkboxes
// TODO: Does not support rendering item labels with item category color
// TODO: Does not support rendering item labels in item category groups

export const HookFormAutocomplete = <FormValues extends FieldValues>({
  items = [],
  groupedItems = [],
  isGrouped = false,
  label,
  fieldId,
  name,
  control,
  noResultsMessage,
  placeholderText,
  searchInputAriaLabel,
  isRequired = false,
}: {
  items?: AutocompleteOptionProps[];
  groupedItems?: GroupedAutocompleteOptionProps[];
  isGrouped?: boolean;
  name: Path<FormValues>;
  control: Control<FormValues>;
  label: string;
  fieldId: string;
  noResultsMessage: string;
  placeholderText: string;
  searchInputAriaLabel: string;
  isRequired?: boolean;
}) => (
  <HookFormPFGroupController
    isRequired={isRequired}
    control={control}
    name={name}
    label={label}
    fieldId={fieldId}
    renderInput={({ field: { value, onChange } }) =>
      isGrouped ? (
        <GroupedAutocomplete
          id={fieldId}
          noResultsMessage={noResultsMessage}
          placeholderText={placeholderText}
          searchInputAriaLabel={searchInputAriaLabel}
          options={groupedItems}
          selections={value}
          onChange={(selection) => {
            onChange(selection);
          }}
        />
      ) : (
        <Autocomplete
          id={fieldId}
          noResultsMessage={noResultsMessage}
          placeholderText={placeholderText}
          searchInputAriaLabel={searchInputAriaLabel}
          options={items}
          selections={value}
          onChange={(selection) => {
            onChange(selection);
          }}
        />
      )
    }
  />
);

export default HookFormAutocomplete;

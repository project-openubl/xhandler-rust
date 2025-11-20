interface BaseOptionProps {
  name: string | (() => string);
  labelName?: string | (() => string);
  tooltip?: string | (() => string);
}

export interface GroupedAutocompleteOptionProps extends BaseOptionProps {
  uniqueId: string;
  group?: string;
}

export interface AutocompleteOptionProps extends BaseOptionProps {
  id: number;
}

// Helper type for use in the hook and components
export type AnyAutocompleteOptionProps =
  | GroupedAutocompleteOptionProps
  | AutocompleteOptionProps;

// Function to get the unique identifier from either type
export const getUniqueId = (
  option: AnyAutocompleteOptionProps,
): string | number => {
  return "uniqueId" in option ? option.uniqueId : option.id;
};

export interface GroupMap {
  [key: string]: AnyAutocompleteOptionProps[];
}

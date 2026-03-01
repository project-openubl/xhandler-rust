import React, { useRef } from "react";

import {
  Divider,
  Flex,
  FlexItem,
  Label,
  type LabelProps,
  Menu,
  MenuContent,
  MenuGroup,
  MenuItem,
  MenuList,
  Popper,
} from "@patternfly/react-core";

import { getString } from "@app/utils/utils";

import { LabelToolip } from "./LabelTooltip";
import { SearchInputComponent } from "./SearchInput";
import { type AnyAutocompleteOptionProps, getUniqueId } from "./type-utils";
import { useAutocompleteHandlers } from "./useAutocompleteHandlers";

export interface AutocompleteOptionProps {
  /** id for the option */
  id: number;

  /** the text to display for the option */
  name: string | (() => string);

  /** the text to display on a label when the option is selected, defaults to `name` if not supplied */
  labelName?: string | (() => string);

  /** the tooltip to display on the Label when the option has been selected */
  tooltip?: string | (() => string);
}

export interface IAutocompleteProps {
  onChange: (selections: AnyAutocompleteOptionProps[]) => void;
  id?: string;

  /** The set of options to use for selection */
  options?: AutocompleteOptionProps[];
  selections?: AutocompleteOptionProps[];

  placeholderText?: string;
  searchString?: string;
  searchInputAriaLabel?: string;
  labelColor?: LabelProps["color"];
  menuHeader?: string;
  noResultsMessage?: string;
}

/**
 * Multiple type-ahead with table complete and selection labels
 */
export const Autocomplete: React.FC<IAutocompleteProps> = ({
  id = "",
  onChange,
  options = [],
  placeholderText = "Search",
  searchString = "",
  searchInputAriaLabel = "Search input",
  labelColor,
  selections = [],
  menuHeader = "",
  noResultsMessage = "No results found",
}) => {
  const menuRef = useRef<HTMLDivElement>(null);
  const searchInputRef = useRef<HTMLInputElement>(null);
  const {
    setInputValue,
    inputValue,
    menuIsOpen,
    groupedFilteredOptions,
    removeSelectionById,
    handleMenuItemOnSelect,
    handleMenuOnKeyDown,
    handleOnDocumentClick,
    handleInputChange,
    handleKeyDown,
    selectedOptions,
  } = useAutocompleteHandlers({
    options,
    searchString,
    selections,
    onChange,
    menuRef,
    searchInputRef,
  });

  const inputGroup = (
    <SearchInputComponent
      id={id}
      placeholderText={placeholderText}
      searchInputAriaLabel={searchInputAriaLabel}
      onSearchChange={handleInputChange}
      onClear={() => setInputValue("")}
      onKeyHandling={handleKeyDown}
      options={options}
      inputValue={inputValue}
      inputRef={searchInputRef}
    />
  );
  const renderMenuItems = () => {
    const allGroups = Object.entries(groupedFilteredOptions);
    if (allGroups.length === 0) {
      return (
        <MenuList>
          <MenuItem isDisabled key="no-options">
            {noResultsMessage || "No options available"}
          </MenuItem>
        </MenuList>
      );
    }

    return allGroups.map(([groupName, groupOptions], index) => (
      <React.Fragment key={groupName || `ungrouped-${index}`}>
        <MenuGroup label={groupName || undefined}>
          <MenuList>
            {groupOptions.length > 0 ? (
              groupOptions.map((option) => (
                <MenuItem
                  key={getUniqueId(option)}
                  itemId={getUniqueId(option)}
                  onClick={(e) => handleMenuItemOnSelect(e, option)}
                >
                  {getString(option.labelName || option.name)}
                </MenuItem>
              ))
            ) : (
              <MenuItem isDisabled key="no result" itemId="-1">
                {noResultsMessage}
              </MenuItem>
            )}
          </MenuList>
        </MenuGroup>
        {index < allGroups.length - 1 && <Divider />}
      </React.Fragment>
    ));
  };

  const menu = (
    <Menu ref={menuRef} onKeyDown={handleMenuOnKeyDown} isScrollable>
      <MenuContent>{renderMenuItems()}</MenuContent>
    </Menu>
  );

  return (
    <Flex direction={{ default: "column" }}>
      <FlexItem key="input">
        <Popper
          trigger={inputGroup}
          triggerRef={searchInputRef}
          popper={menu}
          popperRef={menuRef}
          appendTo={() => searchInputRef.current || document.body}
          isVisible={menuIsOpen}
          onDocumentClick={handleOnDocumentClick}
        />
      </FlexItem>
      <FlexItem key="chips">
        <Flex spaceItems={{ default: "spaceItemsXs" }}>
          {selectedOptions.map((option) => (
            <FlexItem key={getUniqueId(option)}>
              <LabelToolip content={option.tooltip}>
                <Label
                  color={labelColor}
                  onClose={() => removeSelectionById(getUniqueId(option))}
                >
                  {getString(option.labelName || option.name)}
                </Label>
              </LabelToolip>
            </FlexItem>
          ))}
        </Flex>
      </FlexItem>
    </Flex>
  );
};

import * as React from "react";
import {
  SelectOptionProps,
  ToolbarToggleGroup,
  ToolbarItem,
} from "@patternfly/react-core";
import {
  Dropdown,
  DropdownItem,
  DropdownGroup,
  DropdownToggle,
} from "@patternfly/react-core/deprecated";
import FilterIcon from "@patternfly/react-icons/dist/esm/icons/filter-icon";

import { FilterControl } from "./FilterControl";

export enum FilterType {
  select = "select",
  multiselect = "multiselect",
  search = "search",
  numsearch = "numsearch",
}

export type FilterValue = string[] | undefined | null;

export interface OptionPropsWithKey extends SelectOptionProps {
  key: string;
}

export interface IBasicFilterCategory<
  TItem, // The actual API objects we're filtering
  TFilterCategoryKey extends string // Unique identifiers for each filter category (inferred from key properties if possible)
> {
  key: TFilterCategoryKey; // For use in the filterValues state object. Must be unique per category.
  title: string;
  type: FilterType; // If we want to support arbitrary filter types, this could be a React node that consumes context instead of an enum
  filterGroup?: string;
  getItemValue?: (item: TItem) => string | boolean; // For client-side filtering
  serverFilterField?: string; // For server-side filtering, defaults to `key` if omitted. Does not need to be unique if the server supports joining repeated filters.
  getServerFilterValue?: (filterValue: FilterValue) => FilterValue; // For server-side filtering. Defaults to using the UI state's value if omitted.
}

export interface IMultiselectFilterCategory<
  TItem,
  TFilterCategoryKey extends string
> extends IBasicFilterCategory<TItem, TFilterCategoryKey> {
  selectOptions: OptionPropsWithKey[];
  placeholderText?: string;
  logicOperator?: "AND" | "OR";
}

export interface ISelectFilterCategory<TItem, TFilterCategoryKey extends string>
  extends IBasicFilterCategory<TItem, TFilterCategoryKey> {
  selectOptions: OptionPropsWithKey[];
}

export interface ISearchFilterCategory<TItem, TFilterCategoryKey extends string>
  extends IBasicFilterCategory<TItem, TFilterCategoryKey> {
  placeholderText: string;
}

export type FilterCategory<TItem, TFilterCategoryKey extends string> =
  | IMultiselectFilterCategory<TItem, TFilterCategoryKey>
  | ISelectFilterCategory<TItem, TFilterCategoryKey>
  | ISearchFilterCategory<TItem, TFilterCategoryKey>;

export type IFilterValues<TFilterCategoryKey extends string> = Partial<
  Record<TFilterCategoryKey, FilterValue>
>;

export const getFilterLogicOperator = <
  TItem,
  TFilterCategoryKey extends string
>(
  filterCategory?: FilterCategory<TItem, TFilterCategoryKey>,
  defaultOperator: "AND" | "OR" = "OR"
) =>
  (filterCategory &&
    (filterCategory as IMultiselectFilterCategory<TItem, TFilterCategoryKey>)
      .logicOperator) ||
  defaultOperator;

export interface IFilterToolbarProps<TItem, TFilterCategoryKey extends string> {
  filterCategories: FilterCategory<TItem, TFilterCategoryKey>[];
  filterValues: IFilterValues<TFilterCategoryKey>;
  setFilterValues: (values: IFilterValues<TFilterCategoryKey>) => void;
  beginToolbarItems?: JSX.Element;
  endToolbarItems?: JSX.Element;
  pagination?: JSX.Element;
  showFiltersSideBySide?: boolean;
  isDisabled?: boolean;
}

export const FilterToolbar = <TItem, TFilterCategoryKey extends string>({
  filterCategories,
  filterValues,
  setFilterValues,
  pagination,
  showFiltersSideBySide = false,
  isDisabled = false,
}: React.PropsWithChildren<
  IFilterToolbarProps<TItem, TFilterCategoryKey>
>): JSX.Element | null => {
  const [isCategoryDropdownOpen, setIsCategoryDropdownOpen] =
    React.useState(false);
  const [currentFilterCategoryKey, setCurrentFilterCategoryKey] =
    React.useState(filterCategories[0].key);

  const onCategorySelect = (
    category: FilterCategory<TItem, TFilterCategoryKey>
  ) => {
    setCurrentFilterCategoryKey(category.key);
    setIsCategoryDropdownOpen(false);
  };

  const setFilterValue = (
    category: FilterCategory<TItem, TFilterCategoryKey>,
    newValue: FilterValue
  ) => setFilterValues({ ...filterValues, [category.key]: newValue });

  const currentFilterCategory = filterCategories.find(
    (category) => category.key === currentFilterCategoryKey
  );

  const filterGroups = filterCategories.reduce(
    (groups, category) =>
      !category.filterGroup || groups.includes(category.filterGroup)
        ? groups
        : [...groups, category.filterGroup],
    [] as string[]
  );

  const renderDropdownItems = () => {
    if (!!filterGroups.length) {
      return filterGroups.map((filterGroup) => (
        <DropdownGroup label={filterGroup} key={filterGroup}>
          {filterCategories
            .filter(
              (filterCategory) => filterCategory.filterGroup === filterGroup
            )
            .map((filterCategory) => {
              return (
                <DropdownItem
                  id={`filter-category-${filterCategory.key}`}
                  key={filterCategory.key}
                  onClick={() => onCategorySelect(filterCategory)}
                >
                  {filterCategory.title}
                </DropdownItem>
              );
            })}
        </DropdownGroup>
      ));
    } else {
      return filterCategories.map((category) => (
        <DropdownItem
          id={`filter-category-${category.key}`}
          key={category.key}
          onClick={() => onCategorySelect(category)}
        >
          {category.title}
        </DropdownItem>
      ));
    }
  };

  return (
    <>
      <ToolbarToggleGroup
        variant="filter-group"
        toggleIcon={<FilterIcon />}
        breakpoint="2xl"
        spaceItems={
          showFiltersSideBySide ? { default: "spaceItemsMd" } : undefined
        }
      >
        {!showFiltersSideBySide && (
          <ToolbarItem>
            <Dropdown
              isGrouped={!!filterGroups.length}
              toggle={
                <DropdownToggle
                  id="filtered-by"
                  onToggle={() =>
                    setIsCategoryDropdownOpen(!isCategoryDropdownOpen)
                  }
                  isDisabled={isDisabled}
                >
                  <FilterIcon /> {currentFilterCategory?.title}
                </DropdownToggle>
              }
              isOpen={isCategoryDropdownOpen}
              dropdownItems={renderDropdownItems()}
            />
          </ToolbarItem>
        )}

        {filterCategories.map((category) => (
          <FilterControl<TItem, TFilterCategoryKey>
            key={category.key}
            category={category}
            filterValue={filterValues[category.key]}
            setFilterValue={(newValue) => setFilterValue(category, newValue)}
            showToolbarItem={
              showFiltersSideBySide ||
              currentFilterCategory?.key === category.key
            }
            isDisabled={isDisabled}
          />
        ))}
      </ToolbarToggleGroup>
      {pagination ? (
        <ToolbarItem variant="pagination">{pagination}</ToolbarItem>
      ) : null}
    </>
  );
};
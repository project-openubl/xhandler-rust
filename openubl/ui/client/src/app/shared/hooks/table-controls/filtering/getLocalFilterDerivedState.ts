import {
  FilterCategory,
  getFilterLogicOperator,
} from "@app/shared/components/FilterToolbar";
import { objectKeys } from "@app/utils/utils";
import { IFilterState } from "./useFilterState";

export interface ILocalFilterDerivedStateArgs<
  TItem,
  TFilterCategoryKey extends string
> {
  items: TItem[];
  filterCategories?: FilterCategory<TItem, TFilterCategoryKey>[];
}

export const getLocalFilterDerivedState = <
  TItem,
  TFilterCategoryKey extends string
>({
  items,
  filterCategories = [],
  filterState: { filterValues },
}: ILocalFilterDerivedStateArgs<TItem, TFilterCategoryKey> & {
  filterState: IFilterState<TFilterCategoryKey>;
}) => {
  const filteredItems = items.filter((item) =>
    objectKeys(filterValues).every((categoryKey) => {
      const values = filterValues[categoryKey];
      if (!values || values.length === 0) return true;
      const filterCategory = filterCategories.find(
        (category) => category.key === categoryKey
      );
      let itemValue = (item as any)[categoryKey];
      if (filterCategory?.getItemValue) {
        itemValue = filterCategory.getItemValue(item);
      }
      const logicOperator = getFilterLogicOperator(filterCategory);
      return values[logicOperator === "AND" ? "every" : "some"](
        (filterValue) => {
          if (!itemValue) return false;
          const lowerCaseItemValue = String(itemValue).toLowerCase();
          const lowerCaseFilterValue = String(filterValue).toLowerCase();
          return lowerCaseItemValue.indexOf(lowerCaseFilterValue) !== -1;
        }
      );
    })
  );
  return { filteredItems };
};

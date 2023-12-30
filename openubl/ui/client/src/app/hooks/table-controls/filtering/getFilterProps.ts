import {
  FilterCategory,
  IFilterToolbarProps,
} from "@app/components/FilterToolbar";
import { IFilterState } from "./useFilterState";

export interface IFilterPropsArgs<TItem, TFilterCategoryKey extends string> {
  filterState: IFilterState<TFilterCategoryKey>;
  filterCategories?: FilterCategory<TItem, TFilterCategoryKey>[];
}

export const getFilterProps = <TItem, TFilterCategoryKey extends string>({
  filterState: { filterValues, setFilterValues },
  filterCategories = [],
}: IFilterPropsArgs<TItem, TFilterCategoryKey>): IFilterToolbarProps<
  TItem,
  TFilterCategoryKey
> => ({
  filterCategories,
  filterValues,
  setFilterValues,
});

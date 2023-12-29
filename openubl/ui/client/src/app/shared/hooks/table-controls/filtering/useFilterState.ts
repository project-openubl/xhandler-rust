import * as React from "react";
import { useSessionStorage } from "@app/shared/hooks/useStorage";
import {
  FilterCategory,
  FilterValue,
  IFilterValues,
} from "@app/shared/components/FilterToolbar";
import { useUrlParams } from "../../useUrlParams";
import { objectKeys } from "@app/utils/utils";
import { IExtraArgsForURLParamHooks } from "../types";

export interface IFilterState<TFilterCategoryKey extends string> {
  filterValues: IFilterValues<TFilterCategoryKey>;
  setFilterValues: (values: IFilterValues<TFilterCategoryKey>) => void;
}

export interface IFilterStateArgs<TItem, TFilterCategoryKey extends string> {
  filterStorageKey?: string;
  filterCategories?: FilterCategory<TItem, TFilterCategoryKey>[];
}

export const useFilterState = <TItem, TFilterCategoryKey extends string>({
  filterStorageKey,
}: IFilterStateArgs<
  TItem,
  TFilterCategoryKey
>): IFilterState<TFilterCategoryKey> => {
  const state = React.useState<IFilterValues<TFilterCategoryKey>>({});
  const storage = useSessionStorage<IFilterValues<TFilterCategoryKey>>(
    filterStorageKey || "",
    {}
  );
  const [filterValues, setFilterValues] = filterStorageKey ? storage : state;
  return { filterValues, setFilterValues };
};

export const serializeFilterUrlParams = <TFilterCategoryKey extends string>(
  filterValues: IFilterValues<TFilterCategoryKey>
): { filters?: string | null } => {
  // If a filter value is empty/cleared, don't put it in the object in URL params
  const trimmedFilterValues = { ...filterValues };
  objectKeys(trimmedFilterValues).forEach((filterCategoryKey) => {
    if (
      !trimmedFilterValues[filterCategoryKey] ||
      trimmedFilterValues[filterCategoryKey]?.length === 0
    ) {
      delete trimmedFilterValues[filterCategoryKey];
    }
  });
  return {
    filters:
      objectKeys(trimmedFilterValues).length > 0
        ? JSON.stringify(trimmedFilterValues)
        : null, // If there are no filters, remove the filters param from the URL entirely.
  };
};

export const deserializeFilterUrlParams = <
  TFilterCategoryKey extends string
>(serializedParams: {
  filters?: string | null;
}): Partial<Record<TFilterCategoryKey, FilterValue>> => {
  try {
    return JSON.parse(serializedParams.filters || "{}");
  } catch (e) {
    return {};
  }
};

export const useFilterUrlParams = <
  TFilterCategoryKey extends string,
  TURLParamKeyPrefix extends string = string
>({
  urlParamKeyPrefix,
}: IExtraArgsForURLParamHooks<TURLParamKeyPrefix> = {}): IFilterState<TFilterCategoryKey> => {
  const [filterValues, setFilterValues] = useUrlParams({
    keyPrefix: urlParamKeyPrefix,
    keys: ["filters"],
    defaultValue: {} as IFilterValues<TFilterCategoryKey>,
    serialize: serializeFilterUrlParams,
    deserialize: deserializeFilterUrlParams,
  });
  return { filterValues, setFilterValues };
};

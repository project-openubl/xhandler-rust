import { IExtraArgsForURLParamHooks, ITableControlCommonArgs } from "./types";
import { useFilterUrlParams } from "./filtering";
import { useSortUrlParams } from "./sorting";
import { usePaginationUrlParams } from "./pagination";
import { useActiveRowUrlParams } from "./active-row";
import { useExpansionUrlParams } from "./expansion";

export const useTableControlUrlParams = <
  TItem,
  TColumnKey extends string,
  TSortableColumnKey extends TColumnKey,
  TFilterCategoryKey extends string = string,
  TURLParamKeyPrefix extends string = string,
>(
  args: ITableControlCommonArgs<
    TItem,
    TColumnKey,
    TSortableColumnKey,
    TFilterCategoryKey
  > &
    IExtraArgsForURLParamHooks<TURLParamKeyPrefix>
) => {
  // Must pass type params because they can't all be inferred from the required args of useFilterUrlParams
  const filterState = useFilterUrlParams<
    TFilterCategoryKey, // Must pass this because no required args here have categories to infer from
    TURLParamKeyPrefix
  >(args);
  const sortState = useSortUrlParams(args); // Type params inferred from args
  const paginationState = usePaginationUrlParams(args); // Type params inferred from args
  // Must pass type params because they can't all be inferred from the required args of useExpansionUrlParams
  const expansionState = useExpansionUrlParams<TColumnKey, TURLParamKeyPrefix>(
    args
  );
  const activeRowState = useActiveRowUrlParams(args); // Type params inferred from args
  return {
    ...args,
    filterState,
    sortState,
    paginationState,
    expansionState,
    activeRowState,
  };
};

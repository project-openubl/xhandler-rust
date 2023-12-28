import * as React from "react";
import { useUrlParams } from "../../useUrlParams";
import { IExtraArgsForURLParamHooks } from "../types";

export interface IActiveSort<TSortableColumnKey extends string> {
  columnKey: TSortableColumnKey;
  direction: "asc" | "desc";
}

export interface ISortState<TSortableColumnKey extends string> {
  activeSort: IActiveSort<TSortableColumnKey> | null;
  setActiveSort: (sort: IActiveSort<TSortableColumnKey>) => void;
}

export interface ISortStateArgs<TSortableColumnKey extends string> {
  sortableColumns?: TSortableColumnKey[];
  initialSort?: IActiveSort<TSortableColumnKey> | null;
}

const getDefaultSort = <TSortableColumnKey extends string>(
  sortableColumns: TSortableColumnKey[]
): IActiveSort<TSortableColumnKey> | null =>
  sortableColumns[0]
    ? { columnKey: sortableColumns[0], direction: "asc" }
    : null;

export const useSortState = <TSortableColumnKey extends string>({
  sortableColumns = [],
  initialSort = getDefaultSort(sortableColumns),
}: ISortStateArgs<TSortableColumnKey>): ISortState<TSortableColumnKey> => {
  const [activeSort, setActiveSort] = React.useState(initialSort);
  return { activeSort, setActiveSort };
};

export const useSortUrlParams = <
  TSortableColumnKey extends string,
  TURLParamKeyPrefix extends string = string
>({
  sortableColumns = [],
  initialSort = getDefaultSort(sortableColumns),
  urlParamKeyPrefix,
}: ISortStateArgs<TSortableColumnKey> &
  IExtraArgsForURLParamHooks<TURLParamKeyPrefix>): ISortState<TSortableColumnKey> => {
  const [activeSort, setActiveSort] = useUrlParams({
    keyPrefix: urlParamKeyPrefix,
    keys: ["sortColumn", "sortDirection"],
    defaultValue: initialSort,
    serialize: (activeSort) => ({
      sortColumn: activeSort?.columnKey || null,
      sortDirection: activeSort?.direction || null,
    }),
    deserialize: (urlParams) =>
      urlParams.sortColumn && urlParams.sortDirection
        ? {
            columnKey: urlParams.sortColumn as TSortableColumnKey,
            direction: urlParams.sortDirection as "asc" | "desc",
          }
        : null,
  });
  return { activeSort, setActiveSort };
};

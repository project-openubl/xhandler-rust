import { ApiRequestParams } from "@app/api/models";
import { ISortState } from "./useSortState";

export interface IGetSortApiRequestParamsArgs<
  TSortableColumnKey extends string
> {
  sortState?: ISortState<TSortableColumnKey>;
  apiSortFieldKeys?: Record<TSortableColumnKey, string>;
}

export const getSortApiRequestParams = <TSortableColumnKey extends string>({
  sortState,
  apiSortFieldKeys,
}: IGetSortApiRequestParamsArgs<TSortableColumnKey>): Partial<ApiRequestParams> => {
  if (!sortState?.activeSort || !apiSortFieldKeys) return {};
  const { activeSort } = sortState;
  return {
    sort: {
      field: apiSortFieldKeys[activeSort.columnKey],
      direction: activeSort.direction,
    },
  };
};

export const serializeSortRequestParamsForApi = (
  deserializedParams: ApiRequestParams,
  serializedParams: URLSearchParams
) => {
  const { sort } = deserializedParams;
  if (sort) {
    const { field, direction } = sort;
    serializedParams.append("sort", `${direction}:${field}`);
  }
};

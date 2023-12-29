// Api filter/sort/pagination utils
// TODO these could use some unit tests!

import { ApiRequestParams } from "@app/api/models";
import {
  IGetFilterApiRequestParamsArgs,
  getFilterApiRequestParams,
  serializeFilterRequestParamsForApi,
} from "./filtering";
import {
  IGetSortApiRequestParamsArgs,
  getSortApiRequestParams,
  serializeSortRequestParamsForApi,
} from "./sorting";
import {
  IGetPaginationApiRequestParamsArgs,
  getPaginationApiRequestParams,
  serializePaginationRequestParamsForApi,
} from "./pagination";

export const getApiRequestParams = <
  TItem,
  TSortableColumnKey extends string,
  TFilterCategoryKey extends string = string
>(
  args: IGetFilterApiRequestParamsArgs<TItem, TFilterCategoryKey> &
    IGetSortApiRequestParamsArgs<TSortableColumnKey> &
    IGetPaginationApiRequestParamsArgs
): ApiRequestParams => ({
  ...getFilterApiRequestParams(args),
  ...getSortApiRequestParams(args),
  ...getPaginationApiRequestParams(args),
});

export const serializeRequestParamsForApi = (
  deserializedParams: ApiRequestParams
): URLSearchParams => {
  const serializedParams = new URLSearchParams();
  serializeFilterRequestParamsForApi(deserializedParams, serializedParams);
  serializeSortRequestParamsForApi(deserializedParams, serializedParams);
  serializePaginationRequestParamsForApi(deserializedParams, serializedParams);
  return serializedParams;
};

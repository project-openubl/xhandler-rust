import { ApiRequestParams } from "@app/api/models";
import { IPaginationState } from "./usePaginationState";

export interface IGetPaginationApiRequestParamsArgs {
  paginationState?: IPaginationState;
}

export const getPaginationApiRequestParams = ({
  paginationState,
}: IGetPaginationApiRequestParamsArgs): Partial<ApiRequestParams> => {
  if (!paginationState) return {};
  const { pageNumber, itemsPerPage } = paginationState;
  return { page: { pageNumber, itemsPerPage } };
};

export const serializePaginationRequestParamsForApi = (
  deserializedParams: ApiRequestParams,
  serializedParams: URLSearchParams
) => {
  const { page } = deserializedParams;
  if (page) {
    const { pageNumber, itemsPerPage } = page;
    serializedParams.append("limit", String(itemsPerPage));
    serializedParams.append("offset", String((pageNumber - 1) * itemsPerPage));
  }
};

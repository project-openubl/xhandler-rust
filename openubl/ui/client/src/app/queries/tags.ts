import { AxiosError } from "axios";
import { useMutation, useQuery } from "@tanstack/react-query";

import { ApiRequestParams, Tag } from "@app/api/models";
import { getRepositories, getTags, uploadTag } from "@app/api/rest";
import { serializeRequestParamsForApi } from "@app/shared/hooks/table-controls";

export interface ITagsFetchState {
  result: Tag[];
  isFetching: boolean;
  fetchError: unknown;
  refetch: () => void;
}

export const TagsQueryKey = "tags";

export const useFetchTags = (repositoryId?: number) => {
  const { data, isLoading, error, refetch } = useQuery({
    enabled: repositoryId !== undefined,
    queryKey: [TagsQueryKey, repositoryId],
    queryFn: () => getTags(repositoryId),
    onError: (error) => console.log("error, ", error),
    keepPreviousData: true,
  });
  return {
    result: data?.data || [],
    isFetching: isLoading,
    fetchError: error,
    refetch,
  };
};

export const useCreateTagMutation = (
  successCallback?: (res: any) => void,
  errorCallback?: (err: AxiosError) => void
) => {
  return useMutation({
    mutationFn: uploadTag,
    mutationKey: ["upload"],
    onSuccess: (res) => {
      successCallback && successCallback(res);
    },
    onError: (err: AxiosError) => {
      errorCallback && errorCallback(err);
    },
  });
};

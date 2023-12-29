import { AxiosError } from "axios";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { ApiRequestParams, Repository } from "@app/api/models";
import {
  createRepository,
  deleteRepository,
  getRepositoryById,
  getRepositories,
  updateRepository,
} from "@app/api/rest";
import { serializeRequestParamsForApi } from "@app/shared/hooks/table-controls";

export interface IRepositoriesFetchState {
  result: Repository[];
  isFetching: boolean;
  fetchError: unknown;
  refetch: () => void;
}

export const RepositoriesQueryKey = "repositories";

export const useFetchRepositories = (params: ApiRequestParams = {}) => {
  const { data, isLoading, error, refetch } = useQuery({
    queryKey: [
      RepositoriesQueryKey,
      serializeRequestParamsForApi(params).toString(),
    ],
    queryFn: async () => await getRepositories(params),
    onError: (error) => console.log("error, ", error),
    keepPreviousData: true,
  });
  return {
    result: data || { data: [], total: 0, params },
    isFetching: isLoading,
    fetchError: error,
    refetch,
  };
};

export const useRepositoryById = (id: number | string) => {
  const { data, isLoading, error } = useQuery(
    [RepositoriesQueryKey, id],
    async () => (await getRepositoryById(id)).data,
    { onError: (error) => console.log(error) }
  );

  return {
    result: data,
    isFetching: isLoading,
    fetchError: error as AxiosError,
  };
};

export const useCreateRepositoryMutation = (
  onSuccess: (res: any) => void,
  onError: (err: AxiosError) => void
) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createRepository,
    onSuccess: (res) => {
      onSuccess(res);
      queryClient.invalidateQueries([RepositoriesQueryKey]);
    },
    onError,
  });
};

export const useUpdateRepositoryMutation = (
  onSuccess: (res: any) => void,
  onError: (err: AxiosError) => void
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: updateRepository,
    onSuccess: (res) => {
      onSuccess(res);
      queryClient.invalidateQueries([RepositoriesQueryKey]);
    },
    onError: onError,
  });
};

export const useDeleteRepositoryMutation = (
  onSuccess: (res: any) => void,
  onError: (err: AxiosError) => void
) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteRepository,
    onSuccess: (res) => {
      onSuccess(res);
      queryClient.invalidateQueries([RepositoriesQueryKey]);
    },
    onError: onError,
  });
};

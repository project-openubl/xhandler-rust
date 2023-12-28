import { AxiosError } from "axios";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { Organization } from "@app/api/models";
import {
  createOrganization,
  deleteOrganization,
  getOrganizationById,
  getOrganizations,
  updateOrganization,
} from "@app/api/rest";

export interface IProductsFetchState {
  result: Organization[];
  isFetching: boolean;
  fetchError: unknown;
  refetch: () => void;
}

export const OrganizationsQueryKey = "organizations";

export const useFetchOrganizations = () => {
  const { data, isLoading, error, refetch } = useQuery({
    queryKey: [OrganizationsQueryKey],
    queryFn: async () => await getOrganizations(),
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

export const useOrganizationById = (id: number) => {
  const { data, isLoading, error } = useQuery(
    [OrganizationsQueryKey, id],
    async () => (await getOrganizationById(id)).data,
    { onError: (error) => console.log(error) }
  );

  return {
    result: data,
    isFetching: isLoading,
    fetchError: error as AxiosError,
  };
};

export const useCreateOrganizationMutation = (
  onSuccess: (res: any) => void,
  onError: (err: AxiosError) => void
) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createOrganization,
    onSuccess: (res) => {
      onSuccess(res);
      queryClient.invalidateQueries([OrganizationsQueryKey]);
    },
    onError,
  });
};

export const useUpdateOrganizationMutation = (
  onSuccess: (res: any) => void,
  onError: (err: AxiosError) => void
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: updateOrganization,
    onSuccess: (res) => {
      onSuccess(res);
      queryClient.invalidateQueries([OrganizationsQueryKey]);
    },
    onError: onError,
  });
};

export const useDeleteOrganizationMutation = (
  onSuccess: (res: any) => void,
  onError: (err: AxiosError) => void
) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteOrganization,
    onSuccess: (res) => {
      onSuccess(res);
      queryClient.invalidateQueries([OrganizationsQueryKey]);
    },
    onError: onError,
  });
};

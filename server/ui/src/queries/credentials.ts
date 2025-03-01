import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { AxiosError } from "axios";

import type { New } from "@client/helpers";
import type { Credentials } from "@client/models";
import {
  createCredentials,
  deleteCredentials,
  getCredentials,
  getCredentialsById,
  updateCredentials,
} from "@client/rest";

export const CredentialsQueryKey = "credentials";

export const useFetchCredentials = () => {
  const { data, isLoading, error, refetch } = useQuery({
    queryKey: [CredentialsQueryKey],
    queryFn: () => getCredentials(),
  });
  return {
    credentials: data ?? [],
    isFetching: isLoading,
    fetchError: error,
    refetch,
  };
};

export const useFetchCredentialsById = (id?: number | string) => {
  const { data, isLoading, error } = useQuery({
    queryKey: [CredentialsQueryKey, id],
    queryFn: () =>
      id === undefined ? Promise.resolve(undefined) : getCredentialsById(id),
    enabled: id !== undefined,
  });

  return {
    credentials: data,
    isFetching: isLoading,
    fetchError: error as AxiosError,
  };
};

export const useCreateCredentialsMutation = (
  onSuccess: (res: Credentials) => void,
  onError: (err: AxiosError, payload: New<Credentials>) => void,
) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (obj) => createCredentials(obj),
    onSuccess: async (res) => {
      onSuccess(res.data);
      await queryClient.invalidateQueries({ queryKey: [CredentialsQueryKey] });
    },
    onError,
  });
};

export const useUpdateCredentialsMutation = (
  onSuccess: (payload: Credentials) => void,
  onError: (err: AxiosError, payload: Credentials) => void,
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (obj) => updateCredentials(obj),
    onSuccess: async (_res, payload) => {
      onSuccess(payload);
      await queryClient.invalidateQueries({ queryKey: [CredentialsQueryKey] });
    },
    onError: onError,
  });
};

export const useDeleteCredentialsMutation = (
  onSuccess: (id: number | string) => void,
  onError: (err: AxiosError, id: number | string) => void,
) => {
  const queryClient = useQueryClient();

  const { isPending, mutate, error } = useMutation({
    mutationFn: ({ id }: { id: string | number }) => deleteCredentials(id),
    onSuccess: async (_res, { id }) => {
      onSuccess(id);
      await queryClient.invalidateQueries({ queryKey: [CredentialsQueryKey] });
    },
    onError: async (err: AxiosError, { id }) => {
      onError(err, id);
      await queryClient.invalidateQueries({ queryKey: [CredentialsQueryKey] });
    },
  });

  return {
    mutate,
    isPending,
    error,
  };
};

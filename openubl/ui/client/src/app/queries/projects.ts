import { AxiosError } from "axios";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { New, Project } from "@app/api/models";
import {
  createProject,
  deleteProject,
  getProjectById,
  getProjects,
  updateProject as updateProject,
} from "@app/api/rest";

export const ProjectsQueryKey = "projects";

export const useFetchProjects = () => {
  const { data, isLoading, error, refetch } = useQuery({
    queryKey: [ProjectsQueryKey],
    queryFn: getProjects,
    onError: (error: AxiosError) => console.log("error, ", error),
    keepPreviousData: true,
  });
  return {
    projects: data || [],
    isFetching: isLoading,
    fetchError: error,
    refetch,
  };
};

export const useFetchProjectById = (id?: number | string) => {
  const { data, isLoading, error } = useQuery({
    queryKey: [ProjectsQueryKey, id],
    queryFn: () =>
      id === undefined ? Promise.resolve(undefined) : getProjectById(id),
    onError: (error: AxiosError) => console.log("error", error),
    enabled: id !== undefined,
  });

  return {
    project: data,
    isFetching: isLoading,
    fetchError: error as AxiosError,
  };
};

export const useCreateProjectMutation = (
  onSuccess: (res: Project) => void,
  onError: (err: AxiosError, payload: New<Project>) => void
) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createProject,
    onSuccess: ({ data }, _payload) => {
      onSuccess(data);
      queryClient.invalidateQueries([ProjectsQueryKey]);
    },
    onError,
  });
};

export const useUpdateProjectMutation = (
  onSuccess: (payload: Project) => void,
  onError: (err: AxiosError, payload: Project) => void
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: updateProject,
    onSuccess: (_res, payload) => {
      onSuccess(payload);
      queryClient.invalidateQueries([ProjectsQueryKey]);
    },
    onError: onError,
  });
};

export const useDeleteProjectMutation = (
  onSuccess: (id: number | string) => void,
  onError: (err: AxiosError, id: number | string) => void
) => {
  const queryClient = useQueryClient();

  const { isLoading, mutate, error } = useMutation({
    mutationFn: deleteProject,
    onSuccess: (_res, id) => {
      onSuccess(id);
      queryClient.invalidateQueries([ProjectsQueryKey]);
    },
    onError: (err: AxiosError, id) => {
      onError(err, id);
      queryClient.invalidateQueries([ProjectsQueryKey]);
    },
  });

  return {
    mutate,
    isLoading,
    error,
  };
};

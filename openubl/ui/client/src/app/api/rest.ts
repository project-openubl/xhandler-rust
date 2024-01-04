import axios, { AxiosRequestConfig } from "axios";
import { HubPaginatedResult, HubRequestParams, New, Project } from "./models";
import { serializeRequestParamsForHub } from "@app/hooks/table-controls";

const HUB = "/hub";

export const PROJECTS = HUB + "/projects";

interface ApiSearchResult<T> {
  total: number;
  data: T[];
}

export const getApiPaginatedResult = <T>(
  url: string,
  params: HubRequestParams = {}
): Promise<HubPaginatedResult<T>> =>
  axios
    .get<ApiSearchResult<T>>(url, {
      params: serializeRequestParamsForHub(params),
    })
    .then(({ data }) => ({
      data: data.data,
      total: data.total,
      params,
    }));

export const getProjects = () => {
  return axios.get<Project[]>(PROJECTS).then((response) => response.data);
};

export const getProjectById = (id: number | string) => {
  return axios
    .get<Project>(`${PROJECTS}/${id}`)
    .then((response) => response.data);
};

export const createProject = (obj: New<Project>) =>
  axios.post<Project>(PROJECTS, obj);

export const updateProject = (obj: Project) =>
  axios.put<Project>(`${PROJECTS}/${obj.id}`, obj);

export const deleteProject = (id: number | string) =>
  axios.delete<void>(`${PROJECTS}/${id}`);

export const uploadFile = (
  projectId: number | string,
  formData: FormData,
  config?: AxiosRequestConfig
) => axios.post<void>(`${PROJECTS}/${projectId}/files`, formData, config);

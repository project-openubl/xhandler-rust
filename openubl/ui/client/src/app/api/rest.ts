import axios, { AxiosRequestConfig } from "axios";
import {
  HubPaginatedResult,
  HubRequestParams,
  New,
  Project,
  UblDocument,
} from "./models";
import { serializeRequestParamsForHub } from "@app/hooks/table-controls";

const HUB = "/hub";

export const PROJECTS = HUB + "/projects";

interface ApiSearchResult<T> {
  total: number;
  data: T[];
}

export const getHubPaginatedResult = <T>(
  url: string,
  params: HubRequestParams = {}
): Promise<HubPaginatedResult<T>> =>
  axios
    .get<T[]>(url, {
      params: serializeRequestParamsForHub(params),
    })
    .then(({ data, headers }) => ({
      data,
      total: headers["x-total"] ? parseInt(headers["x-total"], 10) : 0,
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
) =>
  axios.post<UblDocument>(`${PROJECTS}/${projectId}/files`, formData, config);

export const getUblDocuments = (
  projectId?: number | string,
  params: HubRequestParams = {}
) => {
  return getHubPaginatedResult<UblDocument>(
    `${PROJECTS}/${projectId}/documents`,
    params
  );
};

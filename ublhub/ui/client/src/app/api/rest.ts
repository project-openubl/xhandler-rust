import axios from "axios";
import {
  Advisory,
  AdvisoryDetails,
  ApiPaginatedResult,
  ApiRequestParams,
  Package,
  PackageDetails,
  Organization,
  Repository,
  New,
  Tag,
} from "./models";
import { serializeRequestParamsForApi } from "@app/shared/hooks/table-controls";

const API = "/api";

const ORGANIZATIONS = API + "/organizations";
const REPOSITORIES = API + "/repositories";

const ADVISORY = API + "/advisory";
const ADVISORY_SEARCH = API + "/advisory/search";

const PACKAGE = API + "/package";
const PACKAGE_SEARCH = API + "/package/search";

const formHeaders = { headers: { Accept: "multipart/form-data" } };

interface ApiSearchResult<T> {
  total: number;
  data: T[];
}

export const getApiPaginatedResult = <T>(
  url: string,
  params: ApiRequestParams = {}
): Promise<ApiPaginatedResult<T>> =>
  axios
    .get<ApiSearchResult<T>>(url, {
      params: serializeRequestParamsForApi(params),
    })
    .then(({ data }) => ({
      data: data.data,
      total: data.total,
      params,
    }));

export const getOrganizations = () => {
  return axios.get<Organization[]>(ORGANIZATIONS);
};

export const getOrganizationById = (id: number) => {
  return axios.get<Organization>(`${ORGANIZATIONS}/${id}`);
};

export const createOrganization = (
  obj: New<Organization>
): Promise<Organization> => axios.post(ORGANIZATIONS, obj);

export const updateOrganization = (obj: Organization): Promise<Organization> =>
  axios.put(`${ORGANIZATIONS}/${obj.id}`, obj);

export const deleteOrganization = (id: number): Promise<Organization> =>
  axios.delete(`${ORGANIZATIONS}/${id}`);

// Repositories

export const getRepositories = (params: ApiRequestParams = {}) => {
  return getApiPaginatedResult<Repository>(REPOSITORIES, params);
};

export const getRepositoryById = (id: number | string) => {
  return axios.get<Repository>(`${REPOSITORIES}/${id}`);
};

export const createRepository = (obj: New<Repository>): Promise<Repository> =>
  axios.post(REPOSITORIES, obj);

export const updateRepository = (obj: Repository): Promise<Repository> =>
  axios.put(`${REPOSITORIES}/${obj.id}`, obj);

export const deleteRepository = (id: number): Promise<void> =>
  axios.delete(`${REPOSITORIES}/${id}`);

//

export const getTags = (repositoryId?: number) => {
  return repositoryId
    ? axios.get<Tag[]>(`${REPOSITORIES}/${repositoryId}/tags`)
    : Promise.reject();
};

export const uploadTag = ({
  repositoryId,
  formData,
}: {
  repositoryId: number;
  formData: any;
}) => {
  return axios.post<Tag>(`${REPOSITORIES}/${repositoryId}/tags`, formData);
};

//

export const getAdvisories = (params: ApiRequestParams = {}) => {
  return getApiPaginatedResult<Advisory>(ADVISORY_SEARCH, params);
};

export const getAdvisoryById = (id: string) => {
  return axios.get<AdvisoryDetails>(`${ADVISORY}?id=${id}`);
};

export const getpackages = (params: ApiRequestParams = {}) => {
  return getApiPaginatedResult<Package>(PACKAGE_SEARCH, params);
};

export const getPackageById = (id: string) => {
  return axios.get<PackageDetails>(`${PACKAGE}?id=${id}`);
};

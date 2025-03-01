import axios from "axios";

import type { New } from "./helpers.ts";
import type { Credentials } from "./models";

const API = "/api";

export const CREDENTIALS = `${API}/credentials`;

export const getCredentials = () => {
  return axios
    .get<Credentials[]>(CREDENTIALS)
    .then((response) => response.data);
};

export const getCredentialsById = (id: number | string) => {
  return axios
    .get<Credentials>(`${CREDENTIALS}/${id}`)
    .then((response) => response.data);
};

export const createCredentials = (obj: New<Credentials>) =>
  axios.post<Credentials>(`${CREDENTIALS}/credentials`, obj);

export const updateCredentials = (obj: Credentials) =>
  axios.put<Credentials>(`${CREDENTIALS}/${obj.id}`, obj);

export const deleteCredentials = (id: number | string) =>
  axios.delete<void>(`${CREDENTIALS}/${id}`);

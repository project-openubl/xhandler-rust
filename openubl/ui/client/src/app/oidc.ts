import { OidcClientSettings, User } from "oidc-client-ts";
import { ENV } from "./env";

export const oidcClientSettings: OidcClientSettings = {
  authority: "/auth",
  client_id: ENV.OIDC_CLIENT_ID || "openubl-ui",
  redirect_uri: window.location.href,
};

export const getUser = () => {
  const oidcStorage = localStorage.getItem(
    `oidc.user:${oidcClientSettings.authority}:${oidcClientSettings.client_id}`
  );
  if (!oidcStorage) {
    return null;
  }

  return User.fromStorageString(oidcStorage);
};

/** Define process.env to contain `UblhubEnvType` */
declare global {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace NodeJS {
    interface ProcessEnv extends Partial<Readonly<OpenublEnvType>> {}
  }
}

/**
 * The set of environment variables used by `@konveyor-ui` packages.
 */
export type OpenublEnvType = {
  NODE_ENV: "development" | "production" | "test";
  VERSION: string;

  /** Controls how mock data is injected on the client */
  MOCK: string;

  /** SSO / Keycloak realm */
  KEYCLOAK_REALM: string;

  /** SSO / Keycloak client id */
  KEYCLOAK_CLIENT_ID: string;

  /** Branding to apply to the UI */
  PROFILE: "openubl" | "mta";

  /** UI upload file size limit in megabytes (MB), suffixed with "m" */
  UI_INGRESS_PROXY_BODY_SIZE: string;

  /** The listen port for the UI's server */
  PORT?: string;

  /** Target URL for the UI server's `/auth` proxy */
  KEYCLOAK_SERVER_URL?: string;

  /** Target URL for the UI server's `/hub` proxy */
  OPENUBL_HUB_URL?: string;
};

/**
 * Keys in `KonveyorEnvType` that are only used on the server and therefore do not
 * need to be sent to the client.
 */
export const SERVER_ENV_KEYS = [
  "PORT",
  "KEYCLOAK_SERVER_URL",
  "OPENUBL_HUB_URL",
];

/**
 * Create a `KonveyorEnv` from a partial `KonveyorEnv` with a set of default values.
 */
export const buildOpenublEnv = ({
  NODE_ENV = "production",
  PORT,
  VERSION = "99.0.0",
  MOCK = "off",

  KEYCLOAK_SERVER_URL,
  KEYCLOAK_REALM = "openubl",
  KEYCLOAK_CLIENT_ID = "openubl-ui",

  PROFILE = "openubl",
  UI_INGRESS_PROXY_BODY_SIZE = "500m",
  OPENUBL_HUB_URL,
}: Partial<OpenublEnvType> = {}): OpenublEnvType => ({
  NODE_ENV,
  PORT,
  VERSION,
  MOCK,

  KEYCLOAK_SERVER_URL,
  KEYCLOAK_REALM,
  KEYCLOAK_CLIENT_ID,

  PROFILE,
  UI_INGRESS_PROXY_BODY_SIZE,
  OPENUBL_HUB_URL,
});

/**
 * Default values for `OpenublEnvType`.
 */
export const OPENUBL_ENV_DEFAULTS = buildOpenublEnv();

/**
 * Current `@openubl-ui` environment configurations from `process.env`.
 */
export const OPENUBL_ENV = buildOpenublEnv(process.env);

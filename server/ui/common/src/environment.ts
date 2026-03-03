/** Define process.env to contain `OpenublEnvType` */
declare global {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace NodeJS {
    interface ProcessEnv extends Partial<Readonly<OpenublEnvType>> {}
  }
}

/**
 * The set of environment variables used by `@openubl-ui` packages.
 */
export type OpenublEnvType = {
  NODE_ENV: "development" | "production" | "test";
  VERSION: string;

  /** Enable RBAC authentication/authorization */
  AUTH_REQUIRED: "true" | "false";

  /** SSO / Oidc client id */
  OIDC_CLIENT_ID?: string;

  /** SSO / Oidc scope */
  OIDC_SCOPE?: string;

  /** UI upload file size limit in megabytes (MB), suffixed with "m" */
  UI_INGRESS_PROXY_BODY_SIZE: string;

  /** The listen port for the UI's server */
  PORT?: string;

  /** Target URL for the UI server's `/auth` proxy */
  OIDC_SERVER_URL?: string;

  /** Whether or not `/auth` proxy will be enabled */
  OIDC_SERVER_IS_EMBEDDED?: "true" | "false";

  /** The Keycloak Realm */
  OIDC_SERVER_EMBEDDED_PATH?: string;

  /** Target URL for the UI server's `/api` proxy */
  OPENUBL_API_URL?: string;

  /** Location of branding files (relative paths computed from the project source root) */
  BRANDING?: string;
};

/**
 * Keys in `OpenublEnv` that are only used on the server and therefore do not
 * need to be sent to the client.
 */
export const SERVER_ENV_KEYS = ["PORT", "OPENUBL_API_URL", "BRANDING"];

/**
 * Create a `OpenublEnv` from a partial `OpenublEnv` with a set of default values.
 */
export const buildOpenublEnv = ({
  NODE_ENV = "production",
  PORT,
  VERSION = "99.0.0",

  OIDC_SERVER_URL,
  OIDC_SERVER_IS_EMBEDDED = "false",
  OIDC_SERVER_EMBEDDED_PATH,
  AUTH_REQUIRED = "true",
  OIDC_CLIENT_ID,
  OIDC_SCOPE,

  UI_INGRESS_PROXY_BODY_SIZE = "500m",
  OPENUBL_API_URL,
  BRANDING,
}: Partial<OpenublEnvType> = {}): OpenublEnvType => ({
  NODE_ENV,
  PORT,
  VERSION,

  OIDC_SERVER_URL,
  OIDC_SERVER_IS_EMBEDDED,
  OIDC_SERVER_EMBEDDED_PATH,
  AUTH_REQUIRED,
  OIDC_CLIENT_ID,
  OIDC_SCOPE,

  UI_INGRESS_PROXY_BODY_SIZE,
  OPENUBL_API_URL,
  BRANDING,
});

/**
 * Default values for `OpenublEnvType`.
 */
export const OPENUBL_ENV_DEFAULTS = buildOpenublEnv();

/**
 * Current `@openubl-ui` environment configurations from `process.env`.
 */
export const OPENUBL_ENV = buildOpenublEnv(process.env);

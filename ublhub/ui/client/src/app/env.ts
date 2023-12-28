/**
 * The set of environment variables used by spog-ui.  This list
 * needs to be kept in sync with `client/config/envLookup.js`.
 */
const ENV_VAR_DEFAULTS = {
  PROFILE: "spog",
  VERSION: "99.0.0",
} as const;

type EnvVarKey = keyof typeof ENV_VAR_DEFAULTS;
const ENV_VAR_KEYS = Object.keys(ENV_VAR_DEFAULTS) as EnvVarKey[];
export type EnvVars = Record<EnvVarKey, string>;

declare global {
  interface Window {
    _env: EnvVars;
  }
}
export const ENV: EnvVars = { ...window._env } || {};

ENV_VAR_KEYS.forEach((key) => {
  if (!ENV[key]) ENV[key] = ENV_VAR_DEFAULTS[key];
});

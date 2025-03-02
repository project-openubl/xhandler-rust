import * as fs from "node:fs";
import * as path from "node:path";

import { type RsbuildPlugin, defineConfig } from "@rsbuild/core";
import { pluginReact } from "@rsbuild/plugin-react";
import { TanStackRouterRspack } from "@tanstack/router-plugin/rspack";

/**
 * Return a base64 encoded JSON string containing the given `env` object.
 */
export const encodeEnv = (env: object, exclude?: string[]): string => {
  const filtered = exclude
    ? Object.fromEntries(
        Object.entries(env).filter(([key]) => !exclude.includes(key)),
      )
    : env;

  return btoa(JSON.stringify(filtered));
};

export const copyIndex = (): RsbuildPlugin => ({
  name: "CopyIndex",
  setup(api) {
    if (process.env.NODE_ENV === "production") {
      api.onAfterBuild(() => {
        const distDir = path.resolve(__dirname, "dist");
        const src = path.join(distDir, "index.html");
        const dest = path.join(distDir, "index.html.ejs");

        if (fs.existsSync(src)) {
          fs.renameSync(src, dest);
        }
      });
    }
  },
});

export default defineConfig({
  plugins: [pluginReact(), copyIndex()],
  html: {
    template: path.join(__dirname, "index.html"),
    templateParameters: {
      ...(process.env.NODE_ENV === "development" && {
        _env: encodeEnv({ version: "99.0.0" }),
      }),
    },
    title: "Openubl",
    favicon: "./src/assets/favicon.svg",
  },
  tools: {
    rspack(_config, { addRules, appendPlugins }) {
      addRules([
        ...(process.env.NODE_ENV === "production"
          ? [
              {
                test: /\.html$/,
                use: "raw-loader", // Ensures HTML remains unprocessed
              },
            ]
          : []),
      ]);
      appendPlugins([
        TanStackRouterRspack({ target: "react", autoCodeSplitting: true }),
      ]);
    },
  },
  output: {
    manifest: true,
  },
});

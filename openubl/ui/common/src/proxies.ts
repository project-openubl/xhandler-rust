import type { Options } from "http-proxy-middleware";
import { OPENUBL_ENV } from "./environment.js";

export const proxyMap: Record<string, Options> = {
  "/auth": {
    target: OPENUBL_ENV.OIDC_SERVER_URL || "http://localhost:9001",
    logLevel: process.env.DEBUG ? "debug" : "info",

    changeOrigin: true,

    onProxyReq: (proxyReq, req, _res) => {
      // Keycloak needs these header set so we can function in Kubernetes (non-OpenShift)
      // https://www.keycloak.org/server/reverseproxy
      //
      // Note, on OpenShift, this works as the haproxy implementation
      // for the OpenShift route is setting these for us automatically
      //
      // We saw problems with including the below broke the OpenShift route
      //  {"X-Forwarded-Proto", req.protocol} broke the OpenShift
      //  {"X-Forwarded-Port", req.socket.localPort}
      //  {"Forwarded", `for=${req.socket.remoteAddress};proto=${req.protocol};host=${req.headers.host}`}
      // so we are not including even though they are customary
      //
      req.socket.remoteAddress &&
        proxyReq.setHeader("X-Forwarded-For", req.socket.remoteAddress);
      req.socket.remoteAddress &&
        proxyReq.setHeader("X-Real-IP", req.socket.remoteAddress);
      req.headers.host &&
        proxyReq.setHeader("X-Forwarded-Host", req.headers.host);
    },
  },

  "/hub": {
    target: OPENUBL_ENV.OPENUBL_HUB_URL || "http://localhost:9002",
    logLevel: process.env.DEBUG ? "debug" : "info",

    changeOrigin: true,
    pathRewrite: {
      "^/hub": "",
    },

    onProxyReq: (proxyReq, req, _res) => {
      // Add the Bearer token to the request if it is not already present, AND if
      // the token is part of the request as a cookie
      if (req.cookies?.keycloak_cookie && !req.headers["authorization"]) {
        proxyReq.setHeader(
          "Authorization",
          `Bearer ${req.cookies.keycloak_cookie}`
        );
      }
    },
    onProxyRes: (proxyRes, req, res) => {
      const includesJsonHeaders =
        req.headers.accept?.includes("application/json");
      if (
        (!includesJsonHeaders && proxyRes.statusCode === 401) ||
        (!includesJsonHeaders && proxyRes.statusMessage === "Unauthorized")
      ) {
        res.redirect("/");
      }
    },
  },
};

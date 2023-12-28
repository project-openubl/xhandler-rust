import React, { Suspense, lazy } from "react";
import { Navigate, useRoutes } from "react-router-dom";

import { Bullseye, Spinner } from "@patternfly/react-core";

const Home = lazy(() => import("./pages/home"));
const Organizations = lazy(() => import("./pages/organizations"));
const Repositories = lazy(() => import("./pages/repositories"));
const ViewRepository = lazy(() => import("./pages/view-repository"));
const ViewSbom = lazy(() => import("./pages/view-sbom"));
const Advisories = lazy(() => import("./pages/advisories"));
const Packages = lazy(() => import("./pages/packages"));
const ViewPackage = lazy(() => import("./pages/view-package"));
const Vulnerabilities = lazy(() => import("./pages/vulnerabilities"));

export const ViewRepositoryRouteParam = "repositoryId";
export const ViewPackageRouteParam = "packageId";

export const AppRoutes = () => {
  const allRoutes = useRoutes([
    { path: "/organizations", element: <Organizations /> },
    { path: "/repositories", element: <Repositories /> },
    {
      path: `/repositories/:${ViewRepositoryRouteParam}`,
      element: <ViewRepository />,
    },
    {
      path: `/products/:${ViewRepositoryRouteParam}/sboms/:sbombId`,
      element: <ViewSbom />,
    },
    { path: "/home", element: <Home /> },
    { path: "/advisory", element: <Advisories /> },
    { path: "/packages", element: <Packages /> },
    { path: `/packages/:${ViewPackageRouteParam}`, element: <ViewPackage /> },
    { path: "/vulnerabilities", element: <Vulnerabilities /> },
    { path: "*", element: <Navigate to="/organizations" /> },
  ]);

  return (
    <Suspense
      fallback={
        <Bullseye>
          <Spinner />
        </Bullseye>
      }
    >
      {allRoutes}
    </Suspense>
  );
};

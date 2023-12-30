import React, { Suspense, lazy } from "react";
import { useRoutes } from "react-router-dom";

import { Bullseye, Spinner } from "@patternfly/react-core";

const Home = lazy(() => import("./pages/home"));
const Projects = lazy(() => import("./pages/projects"));
const Documents = lazy(() => import("./pages/documents"));

export const ViewRepositoryRouteParam = "repositoryId";
export const ViewPackageRouteParam = "packageId";

export const AppRoutes = () => {
  const allRoutes = useRoutes([
    { path: "/", element: <Projects /> },
    { path: "/home", element: <Home /> },
    { path: "/projects/:projectId/documents", element: <Documents /> },
    // { path: "*", element: <Navigate to="/organizations" /> },
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

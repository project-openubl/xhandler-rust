import React from "react";
import { NavLink, useMatch } from "react-router-dom";

import {
  Nav,
  NavGroup,
  NavItem,
  NavList,
  PageSidebar,
} from "@patternfly/react-core";
import { css } from "@patternfly/react-styles";

import { LayoutTheme } from "./layout-constants";
import { useFetchProjectById } from "@app/queries/projects";

const LINK_CLASS = "pf-v5-c-nav__link";
const ACTIVE_LINK_CLASS = "pf-m-current";

export const SidebarApp: React.FC = () => {
  const routeParams = useMatch("/projects/:projectId/*");

  let projectId = routeParams?.params.projectId;
  let { project } = useFetchProjectById(projectId);

  const renderPageNav = () => {
    return (
      <Nav id="nav-sidebar" aria-label="Nav" theme={LayoutTheme}>
        <NavGroup title="General">
          <li className="pf-v5-c-nav__item">
            <NavLink
              to="/"
              className={({ isActive }) => {
                return css(LINK_CLASS, isActive ? ACTIVE_LINK_CLASS : "");
              }}
            >
              Proyectos
            </NavLink>
          </li>
        </NavGroup>

        {projectId && (
          <NavGroup title={project ? `Proyecto: ${project.name}` : "Proyecto"}>
            <li className="pf-v5-c-nav__item">
              <NavLink
                to={`/projects/${projectId}/documents`}
                className={({ isActive }) => {
                  return css(LINK_CLASS, isActive ? ACTIVE_LINK_CLASS : "");
                }}
              >
                Comprobantes electrónicos
              </NavLink>
            </li>
          </NavGroup>
        )}
      </Nav>
    );
  };

  return <PageSidebar theme={LayoutTheme}>{renderPageNav()}</PageSidebar>;
};

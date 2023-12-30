import React from "react";
import { NavLink, useParams } from "react-router-dom";

import {
  Nav,
  NavGroup,
  NavItem,
  NavList,
  PageSidebar,
} from "@patternfly/react-core";
import { css } from "@patternfly/react-styles";

import { LayoutTheme } from "./layout-constants";

const LINK_CLASS = "pf-v5-c-nav__link";
const ACTIVE_LINK_CLASS = "pf-m-current";

export const SidebarApp: React.FC = () => {
  const params = useParams();
console.log(params);
  const renderPageNav = () => {
    return (
      <Nav id="nav-sidebar" aria-label="Nav" theme={LayoutTheme}>
        <NavList>
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
        </NavList>

        {params && (
          <NavGroup title="Proyecto">
            <li className="pf-v5-c-nav__item">
              <NavLink
                to={`/projects/${"projectId"}/documents`}
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

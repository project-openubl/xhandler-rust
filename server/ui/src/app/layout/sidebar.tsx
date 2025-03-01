import type React from "react";

import { Link } from "@tanstack/react-router";

import {
  Nav,
  NavList,
  PageSidebar,
  PageSidebarBody,
} from "@patternfly/react-core";
import { css } from "@patternfly/react-styles";

const ITEM_CLASS = "pf-v6-c-nav__item";
const LINK_CLASS = "pf-v6-c-nav__link";
const ACTIVE_LINK_CLASS = "pf-m-current";

export const SidebarApp: React.FC = () => {
  const renderPageNav = () => {
    return (
      <Nav id="nav-sidebar" aria-label="Nav">
        <NavList>
          <li className={ITEM_CLASS}>
            <Link
              to="/"
              className={css(LINK_CLASS)}
              activeProps={{
                className: css(LINK_CLASS, ACTIVE_LINK_CLASS),
              }}
              activeOptions={{ exact: true }}
            >
              Home
            </Link>
          </li>
          <li className={ITEM_CLASS}>
            <Link
              to="/about"
              className={css(LINK_CLASS)}
              activeProps={{
                className: css(LINK_CLASS, ACTIVE_LINK_CLASS),
              }}
              activeOptions={{ exact: true }}
            >
              About
            </Link>
          </li>
          <li className={ITEM_CLASS}>
            <Link
              to="/credentials"
              className={css(LINK_CLASS)}
              activeProps={{
                className: css(LINK_CLASS, ACTIVE_LINK_CLASS),
              }}
              activeOptions={{ exact: true }}
            >
              Credenciales
            </Link>
          </li>
        </NavList>
      </Nav>
    );
  };

  return (
    <PageSidebar>
      <PageSidebarBody>{renderPageNav()}</PageSidebarBody>
    </PageSidebar>
  );
};

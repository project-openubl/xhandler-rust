import React, { useReducer, useState } from "react";

import {
  Brand,
  Button,
  ButtonVariant,
  Dropdown,
  DropdownItem,
  DropdownList,
  Masthead,
  MastheadBrand,
  MastheadContent,
  MastheadMain,
  MastheadToggle,
  MenuToggle,
  MenuToggleElement,
  PageToggleButton,
  Toolbar,
  ToolbarContent,
  ToolbarGroup,
  ToolbarItem,
} from "@patternfly/react-core";
import BarsIcon from "@patternfly/react-icons/dist/js/icons/bars-icon";
import QuestionCircleIcon from "@patternfly/react-icons/dist/esm/icons/question-circle-icon";
import EllipsisVIcon from "@patternfly/react-icons/dist/esm/icons/ellipsis-v-icon";
import GithubIcon from "@patternfly/react-icons/dist/esm/icons/github-icon";
import HelpIcon from "@patternfly/react-icons/dist/esm/icons/help-icon";

import { AboutApp } from "./about";
import logo from "../images/logo.svg";

export const HeaderApp: React.FC = () => {
  const [isAboutOpen, toggleIsAboutOpen] = useReducer((state) => !state, false);
  const [isKebabDropdownOpen, setIsKebabDropdownOpen] = useState(false);

  const kebabDropdownItems = (
    <>
      <DropdownItem
        key="github"
        isExternalLink
        to="https://github.com/carlosthe19916/sbomhub"
      >
        <GithubIcon /> Github
      </DropdownItem>
      <DropdownItem key="about" onClick={toggleIsAboutOpen}>
        <HelpIcon /> About
      </DropdownItem>
    </>
  );

  const onKebabDropdownToggle = () => {
    setIsKebabDropdownOpen(!isKebabDropdownOpen);
  };

  const onKebabDropdownSelect = () => {
    setIsKebabDropdownOpen(!isKebabDropdownOpen);
  };

  return (
    <>
      <AboutApp isOpen={isAboutOpen} onClose={toggleIsAboutOpen} />

      <Masthead>
        <MastheadToggle>
          <PageToggleButton variant="plain" aria-label="Global navigation">
            <BarsIcon />
          </PageToggleButton>
        </MastheadToggle>
        <MastheadMain>
          <MastheadBrand>
            <Brand
              widths={{ default: "100px", md: "120px", "2xl": "155px" }}
              src={logo}
              alt="Logo"
            >
              <source media="(min-width: 768px)" srcSet={logo} />
              <source srcSet={logo} />
            </Brand>
          </MastheadBrand>
        </MastheadMain>
        <MastheadContent>
          <Toolbar id="toolbar" isFullHeight isStatic>
            <ToolbarContent>
              <ToolbarGroup
                variant="icon-button-group"
                align={{ default: "alignRight" }}
                spacer={{ default: "spacerNone", md: "spacerMd" }}
              >
                <ToolbarGroup
                  variant="icon-button-group"
                  visibility={{ default: "hidden", lg: "visible" }}
                >
                  <ToolbarItem>
                    <Button
                      aria-label="Github"
                      variant={ButtonVariant.plain}
                      icon={<GithubIcon />}
                      component="a"
                      target="_blank"
                      href="https://github.com/trustification/trustification"
                    />
                  </ToolbarItem>
                  <ToolbarItem>
                    <Button
                      aria-label="About"
                      variant={ButtonVariant.plain}
                      icon={<QuestionCircleIcon />}
                      onClick={toggleIsAboutOpen}
                    />
                  </ToolbarItem>
                </ToolbarGroup>
                <ToolbarItem
                  visibility={{
                    default: "hidden",
                    md: "visible",
                    lg: "hidden",
                  }}
                >
                  <Dropdown
                    isOpen={isKebabDropdownOpen}
                    onSelect={onKebabDropdownSelect}
                    onOpenChange={(isOpen: boolean) =>
                      setIsKebabDropdownOpen(isOpen)
                    }
                    popperProps={{ position: "right" }}
                    toggle={(toggleRef: React.Ref<MenuToggleElement>) => (
                      <MenuToggle
                        ref={toggleRef}
                        onClick={onKebabDropdownToggle}
                        isExpanded={isKebabDropdownOpen}
                        variant="plain"
                        aria-label="About"
                      >
                        <EllipsisVIcon aria-hidden="true" />
                      </MenuToggle>
                    )}
                  >
                    <DropdownList>{kebabDropdownItems}</DropdownList>
                  </Dropdown>
                </ToolbarItem>
              </ToolbarGroup>
            </ToolbarContent>
          </Toolbar>
        </MastheadContent>
      </Masthead>
    </>
  );
};

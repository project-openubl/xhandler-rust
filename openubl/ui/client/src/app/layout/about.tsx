import React from "react";

import {
  AboutModal,
  TextContent,
} from "@patternfly/react-core";

import logo from "../images/logo.svg";
import backgroundImage from "../images/pfbg-icon.svg";

interface IButtonAboutAppProps {
  isOpen: boolean;
  onClose: () => void;
}

export const AboutApp: React.FC<IButtonAboutAppProps> = ({
  isOpen,
  onClose,
}) => {
  return (
    <AboutModal
      isOpen={isOpen}
      onClose={onClose}
      productName={"SbomHub"}
      brandImageAlt="Brand Image"
      brandImageSrc={logo}
      backgroundImageSrc={backgroundImage}
      trademark="Copyright © 2020, 2023 by SbomHub"
    >
      <TextContent>Description</TextContent>
    </AboutModal>
  );
};

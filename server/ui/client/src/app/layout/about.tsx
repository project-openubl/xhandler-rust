import type React from "react";

import { AboutModal, Content, ContentVariants } from "@patternfly/react-core";

import backgroundImage from "@app/assets/pfbg-icon.svg";

interface IButtonAboutAppProps {
  isOpen: boolean;
  onClose: () => void;
}

const TRANSPARENT_1x1_GIF =
  "data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw== ";

export const AboutApp: React.FC<IButtonAboutAppProps> = ({
  isOpen,
  onClose,
}) => {
  return (
    <AboutModal
      isOpen={isOpen}
      onClose={onClose}
      productName={"OpenUBL"}
      brandImageAlt="Logo"
      brandImageSrc={TRANSPARENT_1x1_GIF}
      backgroundImageSrc={backgroundImage}
      trademark={`COPYRIGHT © 2020, ${new Date().getFullYear()}`}
    >
      <Content>
        <Content component={ContentVariants.p}>
          OpenUBL is vendor-neutral, thought-leadering, mostly informational
          collection of resources devoted to making Software Supply Chains
          easier to create, manage, consume and ultimately… to trust!
        </Content>

        <Content component={ContentVariants.p}>
          For more information refer to{" "}
          <Content
            component={ContentVariants.a}
            href={"https://project-openubl.github.io/"}
            target="_blank"
          >
            OpenUBL documentation
          </Content>
        </Content>
      </Content>
      <Content className="pf-v6-u-py-xl">
        <Content>
          <Content component="dl">
            <Content component="dt">Version</Content>
            <Content component="dd">99.0.0</Content>
          </Content>
        </Content>
      </Content>
    </AboutModal>
  );
};

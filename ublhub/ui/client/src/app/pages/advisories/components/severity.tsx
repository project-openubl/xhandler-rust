import React from "react";

import { Flex, FlexItem } from "@patternfly/react-core";
import { Advisory, AdvisorySeverity } from "@app/api/models";
import ShieldIcon from "@patternfly/react-icons/dist/esm/icons/shield-alt-icon";

import {
  global_info_color_100 as lowColor,
  global_warning_color_100 as moderateColor,
  global_danger_color_100 as importantColor,
  global_palette_purple_400 as criticalColor,
} from "@patternfly/react-tokens";

type AdvisorySeverityListType = {
  [key in AdvisorySeverity]: {
    color: { name: string; value: string; var: string };
  };
};
const advisorySeverityList: AdvisorySeverityListType = {
  Low: {
    color: lowColor,
  },
  Moderate: {
    color: moderateColor,
  },
  Important: {
    color: importantColor,
  },
  Critical: {
    color: criticalColor,
  },
};

interface SeverityProps {
  advisory: Advisory;
}

export const Severity: React.FC<SeverityProps> = ({ advisory }) => {
  let severityProps = advisorySeverityList[advisory.severity];

  return (
    <Flex
      spaceItems={{ default: "spaceItemsSm" }}
      alignItems={{ default: "alignItemsCenter" }}
      flexWrap={{ default: "nowrap" }}
      style={{ whiteSpace: "nowrap" }}
    >
      <FlexItem>
        <ShieldIcon color={severityProps.color.value} />
      </FlexItem>
      <FlexItem>{advisory.severity}</FlexItem>
    </Flex>
  );
};

import type React from "react";

import {
  EmptyState,
  EmptyStateBody,
  EmptyStateVariant,
} from "@patternfly/react-core";

export const StateNoData: React.FC = () => {
  return (
    <EmptyState
      variant={EmptyStateVariant.sm}
      titleText="No data available"
      headingLevel="h4"
      status="info"
    >
      <EmptyStateBody>No data available to be shown here.</EmptyStateBody>
    </EmptyState>
  );
};

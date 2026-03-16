import type React from "react";

import {
  EmptyState,
  EmptyStateBody,
  EmptyStateVariant,
} from "@patternfly/react-core";

export const StateError: React.FC = () => {
  return (
    <EmptyState
      variant={EmptyStateVariant.sm}
      titleText="Unable to connect"
      headingLevel="h4"
      status="danger"
    >
      <EmptyStateBody>
        There was an error retrieving data. Check your connection and try again.
      </EmptyStateBody>
    </EmptyState>
  );
};

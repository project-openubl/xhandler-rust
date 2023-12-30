import React from "react";
import {
  Bullseye,
  EmptyState,
  EmptyStateBody,
  EmptyStateIcon,
  EmptyStateVariant,
  Spinner,
  Title,
} from "@patternfly/react-core";
import { Tbody, Tr, Td } from "@patternfly/react-table";
import ExclamationCircleIcon from "@patternfly/react-icons/dist/esm/icons/exclamation-circle-icon";
import CubesIcon from "@patternfly/react-icons/dist/esm/icons/cubes-icon";

import { global_danger_color_200 as globalDangerColor200 } from "@patternfly/react-tokens";

export interface IConditionalTableBodyProps {
  numRenderedColumns: number;
  isLoading?: boolean;
  isError?: boolean;
  isNoData?: boolean;
  errorEmptyState?: React.ReactNode;
  noDataEmptyState?: React.ReactNode;
  children: React.ReactNode;
}

export const ConditionalTableBody: React.FC<IConditionalTableBodyProps> = ({
  numRenderedColumns,
  isLoading = false,
  isError = false,
  isNoData = false,
  errorEmptyState = null,
  noDataEmptyState = null,
  children,
}) => {
  const error = (
    <EmptyState variant={EmptyStateVariant.sm}>
      <EmptyStateIcon
        icon={ExclamationCircleIcon}
        color={globalDangerColor200.value}
      />
      <Title headingLevel="h2" size="lg">
        Unable to connect
      </Title>
      <EmptyStateBody>
        There was an error retrieving data. Check your connection and try again.
      </EmptyStateBody>
    </EmptyState>
  );

  const noData = (
    <EmptyState variant={EmptyStateVariant.sm}>
      <EmptyStateIcon icon={CubesIcon} />
      <Title headingLevel="h2" size="lg">
        No data available
      </Title>
      <EmptyStateBody>No data available to be shown here.</EmptyStateBody>
    </EmptyState>
  );

  return (
    <>
      {isLoading ? (
        <Tbody>
          <Tr>
            <Td colSpan={numRenderedColumns}>
              <Bullseye>
                <Spinner size="xl" />
              </Bullseye>
            </Td>
          </Tr>
        </Tbody>
      ) : isError ? (
        <Tbody aria-label="Table error">
          <Tr>
            <Td colSpan={numRenderedColumns}>
              <Bullseye>{errorEmptyState || error}</Bullseye>
            </Td>
          </Tr>
        </Tbody>
      ) : isNoData ? (
        <Tbody aria-label="Table error">
          <Tr>
            <Td colSpan={numRenderedColumns}>
              <Bullseye>{noDataEmptyState || noData}</Bullseye>
            </Td>
          </Tr>
        </Tbody>
      ) : (
        children
      )}
    </>
  );
};

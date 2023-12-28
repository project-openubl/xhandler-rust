import React from "react";

import {
  Button,
  EmptyState,
  EmptyStateIcon,
  EmptyStateVariant,
  Flex,
  FlexItem,
  List,
  ListComponent,
  ListItem,
  OrderType,
  PageSection,
  PageSectionVariants,
  Text,
  TextContent,
  Title,
  Toolbar,
  ToolbarContent,
  ToolbarItem,
} from "@patternfly/react-core";
import {
  getApiRequestParams,
  useLocalTableControls,
  useTableControlProps,
  useTableControlUrlParams,
} from "@app/shared/hooks/table-controls";
import {
  FilterToolbar,
  FilterType,
} from "@app/shared/components/FilterToolbar";
import { useFetchPackages } from "@app/queries/packages";
import { useSelectionState } from "@app/shared/hooks/useSelectionState";
import { SimplePagination } from "@app/shared/components/SimplePagination";
import {
  ExpandableRowContent,
  Table,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
} from "@patternfly/react-table";
import {
  ConditionalTableBody,
  TableHeaderContentWithControls,
  TableRowContentWithControls,
} from "@app/shared/components/TableControls";
import DownloadIcon from "@patternfly/react-icons/dist/esm/icons/download-icon";
import spacing from "@patternfly/react-styles/css/utilities/Spacing/spacing";
import { Link, NavLink } from "react-router-dom";
import ShieldIcon from "@patternfly/react-icons/dist/esm/icons/shield-alt-icon";
import {
  global_info_color_100 as lowColor,
  global_warning_color_100 as moderateColor,
  global_danger_color_100 as importantColor,
  global_palette_purple_400 as criticalColor,
} from "@patternfly/react-tokens";
import {
  IPageDrawerContentProps,
  PageDrawerContent,
} from "@app/shared/components/PageDrawerContext";
import CubesIcon from "@patternfly/react-icons/dist/esm/icons/cubes-icon";

interface RowData {
  name: string;
  description: string;
}

export const Packages: React.FC = () => {
  const rows: RowData[] = [
    {
      name: "openssl",
      description: "description",
    },
    {
      name: "log4j",
      description: "description",
    },
    {
      name: "package-3",
      description: "description",
    },
  ];

  const tableControls = useLocalTableControls({
    idProperty: "name",
    items: rows,
    columnNames: {
      name: "Name",
      description: "Description",
    },
    hasActionsColumn: true,
    filterCategories: [
      {
        key: "q",
        title: "Name",
        type: FilterType.search,
        placeholderText: "Filter by cve name...",
      },
    ],
    sortableColumns: ["name"],
    getSortValues: (item) => ({
      name: item?.name || "",
    }),
    hasPagination: true,
  });

  const {
    currentPageItems,
    numRenderedColumns,
    propHelpers: {
      toolbarProps,
      filterToolbarProps,
      paginationToolbarItemProps,
      paginationProps,
      tableProps,
      getThProps,
      getTdProps,
      getExpandedContentTdProps,
      getCompoundExpandTdProps,
    },
    expansionDerivedState: { isCellExpanded },
  } = tableControls;

  return (
    <>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">Packages</Text>
          <Text component="p">Search packages</Text>
        </TextContent>
      </PageSection>
      <PageSection>
        <div
          style={{
            backgroundColor: "var(--pf-v5-global--BackgroundColor--100)",
          }}
        >
          <Toolbar {...toolbarProps}>
            <ToolbarContent>
              <FilterToolbar {...filterToolbarProps} />
              <ToolbarItem {...paginationToolbarItemProps}>
                <SimplePagination
                  idPrefix="packages-table"
                  isTop
                  paginationProps={paginationProps}
                />
              </ToolbarItem>
            </ToolbarContent>
          </Toolbar>

          <Table {...tableProps} aria-label="Packages table">
            <Thead>
              <Tr>
                <TableHeaderContentWithControls {...tableControls}>
                  <Th {...getThProps({ columnKey: "name" })} />
                  <Th {...getThProps({ columnKey: "description" })} />
                </TableHeaderContentWithControls>
              </Tr>
            </Thead>
            <ConditionalTableBody
              isLoading={false}
              // isError={!!fetchError}
              isNoData={rows.length === 0}
              numRenderedColumns={numRenderedColumns}
            >
              {currentPageItems?.map((item, rowIndex) => {
                return (
                  <Tbody key={item.name}>
                    <Tr>
                      <TableRowContentWithControls
                        {...tableControls}
                        item={item}
                        rowIndex={rowIndex}
                      >
                        <Td {...getTdProps({ columnKey: "name" })}>
                          <NavLink to={`/packages/${item.name}`}>
                            {item.name}
                          </NavLink>
                        </Td>
                        <Td {...getTdProps({ columnKey: "description" })}>
                          {item.description}
                        </Td>
                      </TableRowContentWithControls>
                    </Tr>
                  </Tbody>
                );
              })}
            </ConditionalTableBody>
          </Table>

          <SimplePagination
            idPrefix="dependencies-table"
            isTop={false}
            paginationProps={paginationProps}
          />
        </div>
      </PageSection>
    </>
  );
};

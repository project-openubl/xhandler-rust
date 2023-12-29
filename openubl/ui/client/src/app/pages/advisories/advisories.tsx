import React from "react";

import {
  Button,
  PageSection,
  PageSectionVariants,
  Stack,
  StackItem,
  Text,
  TextContent,
  Toolbar,
  ToolbarContent,
  ToolbarItem,
} from "@patternfly/react-core";
import { useFetchAdvisories } from "@app/queries/advisories";
import {
  getApiRequestParams,
  useTableControlProps,
} from "@app/shared/hooks/table-controls";
import { useTableControlUrlParams } from "@app/shared/hooks/table-controls";
import {
  FilterToolbar,
  FilterType,
} from "@app/shared/components/FilterToolbar";
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
import { AdvisoryDetails } from "./components/details";
import { Severity } from "./components/severity";

export const Advisories: React.FC = () => {
  const tableControlState = useTableControlUrlParams({
    columnNames: {
      id: "Id",
      title: "Title",
      severity: "Severity",
      revision: "Revision",
      download: "Download",
      vulnerabilities: "Vulnerabilities",
    },
    sortableColumns: [],
    initialSort: null,
    filterCategories: [
      {
        key: "q",
        title: "Name",
        type: FilterType.search,
        placeholderText: "Search",
      },
    ],
    initialItemsPerPage: 10,
    expandableVariant: "single",
  });

  const {
    result: { data: currentPageItems, total: totalItemCount },
    isFetching,
    fetchError,
  } = useFetchAdvisories(
    getApiRequestParams({
      ...tableControlState, // Includes filterState, sortState and paginationState
    })
  );

  const tableControls = useTableControlProps({
    ...tableControlState, // Includes filterState, sortState and paginationState
    idProperty: "id",
    currentPageItems,
    totalItemCount,
    isLoading: isFetching,
    selectionState: useSelectionState({
      items: currentPageItems,
      isEqual: (a, b) => a.id === b.id,
    }),
  });

  const {
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
    },
    expansionDerivedState: { isCellExpanded },
  } = tableControls;

  return (
    <>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">Advisories</Text>
          <Text component="p">Search security advisories</Text>
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
                  idPrefix="advisories-table"
                  isTop
                  paginationProps={paginationProps}
                />
              </ToolbarItem>
            </ToolbarContent>
          </Toolbar>

          <Table {...tableProps} aria-label="Advisories table">
            <Thead>
              <Tr>
                <TableHeaderContentWithControls {...tableControls}>
                  <Th {...getThProps({ columnKey: "id" })} />
                  <Th {...getThProps({ columnKey: "title" })} />
                  <Th {...getThProps({ columnKey: "severity" })} />
                  <Th {...getThProps({ columnKey: "revision" })} />
                  <Th {...getThProps({ columnKey: "download" })} />
                  <Th {...getThProps({ columnKey: "vulnerabilities" })} />
                </TableHeaderContentWithControls>
              </Tr>
            </Thead>
            <ConditionalTableBody
              isLoading={isFetching}
              isError={!!fetchError}
              isNoData={totalItemCount === 0}
              numRenderedColumns={numRenderedColumns}
            >
              {currentPageItems?.map((item, rowIndex) => {
                return (
                  <Tbody key={item.id} isExpanded={isCellExpanded(item)}>
                    <Tr>
                      <TableRowContentWithControls
                        {...tableControls}
                        item={item}
                        rowIndex={rowIndex}
                      >
                        <Td width={15} {...getTdProps({ columnKey: "id" })}>
                          {item.id}
                        </Td>
                        <Td
                          width={45}
                          modifier="truncate"
                          {...getTdProps({ columnKey: "title" })}
                        >
                          {item.title}
                        </Td>
                        <Td {...getTdProps({ columnKey: "severity" })}>
                          <Severity advisory={item} />
                        </Td>
                        <Td {...getTdProps({ columnKey: "revision" })}>
                          {item.date}
                        </Td>
                        <Td {...getTdProps({ columnKey: "download" })}>
                          <Button
                            variant="plain"
                            icon={<DownloadIcon />}
                            component="a"
                            href={item.href}
                            target="_blank"
                          />
                        </Td>
                        <Td {...getTdProps({ columnKey: "download" })}>
                          {item.cves.length ?? "N/A"}
                        </Td>
                      </TableRowContentWithControls>
                    </Tr>
                    {isCellExpanded(item) ? (
                      <Tr isExpanded>
                        <Td />
                        <Td
                          {...getExpandedContentTdProps({ item })}
                          className={spacing.pyLg}
                        >
                          <ExpandableRowContent>
                            <Stack hasGutter>
                              <StackItem>{item.desc}</StackItem>
                              <StackItem>
                                <AdvisoryDetails advisory={item} />
                              </StackItem>
                            </Stack>
                          </ExpandableRowContent>
                        </Td>
                      </Tr>
                    ) : null}
                  </Tbody>
                );
              })}
            </ConditionalTableBody>
          </Table>

          <SimplePagination
            idPrefix="advisories-table"
            isTop={false}
            paginationProps={paginationProps}
          />
        </div>
      </PageSection>
    </>
  );
};

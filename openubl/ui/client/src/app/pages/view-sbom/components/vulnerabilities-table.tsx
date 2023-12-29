import {
  FilterToolbar,
  FilterType,
} from "@app/shared/components/FilterToolbar";
import { SimplePagination } from "@app/shared/components/SimplePagination";
import {
  ConditionalTableBody,
  TableHeaderContentWithControls,
  TableRowContentWithControls,
} from "@app/shared/components/TableControls";
import { useLocalTableControls } from "@app/shared/hooks/table-controls";
import {
  Button,
  ButtonVariant,
  DescriptionList,
  DescriptionListDescription,
  DescriptionListGroup,
  DescriptionListTerm,
  EmptyState,
  EmptyStateIcon,
  EmptyStateVariant,
  Flex,
  FlexItem,
  List,
  ListComponent,
  ListItem,
  OrderType,
  Tab,
  TabTitleText,
  Tabs,
  Text,
  TextContent,
  Title,
  Toolbar,
  ToolbarContent,
  ToolbarGroup,
  ToolbarItem,
} from "@patternfly/react-core";
import {
  ActionsColumn,
  ExpandableRowContent,
  Table,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
} from "@patternfly/react-table";
import React, { useState } from "react";
import { NavLink } from "react-router-dom";
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
  packages: string[];
}

export const VulnerabilitiesTable: React.FC = () => {
  const rows: RowData[] = [
    {
      name: "CVE-1",
      description: "description",
      packages: ["antlr:antlr:jar"],
    },
    {
      name: "CVE-2",
      description: "description",
      packages: ["antlr:antlr:jar"],
    },
    {
      name: "CVE-3",
      description: "description",
      packages: ["biz.aQute.bnd:biz.aQute.bnd.transform:jar"],
    },
    {
      name: "CVE-4",
      description: "description",
      packages: ["biz.aQute.bnd:biz.aQute.bnd.transform:jar"],
    },
    {
      name: "CVE-6",
      description: "description",
      packages: ["aopalliance:aopalliance:jar"],
    },
  ];

  const tableControls = useLocalTableControls({
    idProperty: "name",
    items: rows,
    columnNames: {
      name: "Name",
      description: "Description",
      packages: "Packages",
    },
    hasActionsColumn: true,
    expandableVariant: "compound",
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

  const [activeRowItem, setActiveRowItem] = React.useState<string>();

  return (
    <>
      <div
        style={{
          backgroundColor: "var(--pf-v5-global--BackgroundColor--100)",
        }}
      >
        <Toolbar {...toolbarProps}>
          <ToolbarContent>
            <FilterToolbar {...filterToolbarProps} showFiltersSideBySide />
            {/* <ToolbarGroup variant="button-group">
              <ToolbarItem>
                <Button
                  type="button"
                  id="create-product"
                  aria-label="Create new product"
                  variant={ButtonVariant.primary}
                  onClick={() => setCreateUpdateModalState("create")}
                >
                  Create new
                </Button>
              </ToolbarItem>
            </ToolbarGroup> */}
            <ToolbarItem {...paginationToolbarItemProps}>
              <SimplePagination
                idPrefix="products-table"
                isTop
                paginationProps={paginationProps}
              />
            </ToolbarItem>
          </ToolbarContent>
        </Toolbar>

        <Table {...tableProps} aria-label="Products table">
          <Thead>
            <Tr>
              <TableHeaderContentWithControls {...tableControls}>
                <Th {...getThProps({ columnKey: "name" })} />
                <Th {...getThProps({ columnKey: "description" })} />
                <Th {...getThProps({ columnKey: "packages" })} />
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
                <Tbody key={item.name} isExpanded={isCellExpanded(item)}>
                  <Tr>
                    <TableRowContentWithControls
                      {...tableControls}
                      item={item}
                      rowIndex={rowIndex}
                    >
                      <Td
                        {...getCompoundExpandTdProps({
                          item: item,
                          rowIndex,
                          columnKey: "name",
                        })}
                      >
                        {item.name}
                      </Td>
                      <Td
                        modifier="truncate"
                        {...getTdProps({ columnKey: "description" })}
                      >
                        {item.description}
                      </Td>
                      <Td
                        {...getCompoundExpandTdProps({
                          item: item,
                          rowIndex,
                          columnKey: "packages",
                        })}
                      >
                        {item.packages.length}
                      </Td>
                    </TableRowContentWithControls>
                  </Tr>
                  {isCellExpanded(item) ? (
                    <Tr isExpanded>
                      <Td
                        {...getExpandedContentTdProps({
                          item: item,
                        })}
                      >
                        <ExpandableRowContent>
                          {isCellExpanded(item, "name") && (
                            <div className="pf-v5-u-m-md">
                              <DescriptionList columnModifier={{ lg: "3Col" }}>
                                <DescriptionListGroup>
                                  <DescriptionListTerm>
                                    CVSS score applicability (General)
                                  </DescriptionListTerm>
                                  <DescriptionListDescription>
                                    The CVSS score(s) listed for this
                                    vulnerability do not reflect the associated
                                    product's status, and are included for
                                    informational purposes to better understand
                                    the severity of this vulnerability.
                                  </DescriptionListDescription>
                                </DescriptionListGroup>
                                <DescriptionListGroup>
                                  <DescriptionListTerm>IDs</DescriptionListTerm>
                                  <DescriptionListDescription>
                                    https://bugzilla.redhat.com/show_bug.cgi?id=2222204
                                    (Red Hat Bugzilla)
                                  </DescriptionListDescription>
                                </DescriptionListGroup>
                                <DescriptionListGroup>
                                  <DescriptionListTerm>
                                    Vulnerability summary (Summary)
                                  </DescriptionListTerm>
                                  <DescriptionListDescription>
                                    memory allocation hazard and crash
                                  </DescriptionListDescription>
                                </DescriptionListGroup>                                
                              </DescriptionList>
                            </div>
                          )}
                          {isCellExpanded(item, "packages") && (
                            <div className="pf-v5-u-m-md">
                              <List
                                component={ListComponent.ol}
                                type={OrderType.number}
                              >
                                {item.packages.map((e, vuln_index) => (
                                  <ListItem key={vuln_index}>
                                    <Button
                                      variant="link"
                                      onClick={() => setActiveRowItem(e)}
                                    >
                                      {e}
                                    </Button>
                                  </ListItem>
                                ))}
                              </List>
                            </div>
                          )}
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
          idPrefix="dependencies-table"
          isTop={false}
          paginationProps={paginationProps}
        />

        <DependencyAppsDetailDrawer
          cve={activeRowItem || null}
          onCloseClick={() => setActiveRowItem(undefined)}
        ></DependencyAppsDetailDrawer>
      </div>
    </>
  );
};

export interface ICVEDetailDrawerProps
  extends Pick<IPageDrawerContentProps, "onCloseClick"> {
  cve: string | null;
}

export const DependencyAppsDetailDrawer: React.FC<ICVEDetailDrawerProps> = ({
  cve,
  onCloseClick,
}) => {
  return (
    <PageDrawerContent
      isExpanded={!!cve}
      onCloseClick={onCloseClick}
      focusKey={cve || undefined}
      pageKey="analysis-app-dependencies"
      drawerPanelContentProps={{ defaultSize: "600px" }}
    >
      {!cve ? (
        <EmptyState variant={EmptyStateVariant.sm}>
          <EmptyStateIcon icon={CubesIcon} />
          <Title headingLevel="h2" size="lg">
            No data
          </Title>
        </EmptyState>
      ) : (
        <>
          <TextContent>
            <Text component="small">Package details</Text>
            <Title headingLevel="h2" size="lg">
              Details of the package
            </Title>
          </TextContent>
        </>
      )}
    </PageDrawerContent>
  );
};

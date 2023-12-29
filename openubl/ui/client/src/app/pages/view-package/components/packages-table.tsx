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
  version: string;
  vulnerabilities: string[];
  products: string[];
}

export const PackagesTable: React.FC = () => {
  const rows: RowData[] = [
    {
      version: "1.2.17",
      vulnerabilities: ["CVE-1", "CVE-2"],
      products: ["JBoss EAP", "Quarkus"],
    },
    {
      version: "1.2.16",
      vulnerabilities: ["CVE-1", "CVE-2"],
      products: ["Product1"],
    },
    {
      version: "1.2.15",
      vulnerabilities: ["CVE-1", "CVE-2"],
      products: ["Product1"],
    },
    {
      version: "1.2.14",
      vulnerabilities: ["CVE-1", "CVE-2"],
      products: ["Product1"],
    },
    {
      version: "1.2.13",
      vulnerabilities: ["CVE-1", "CVE-2"],
      products: ["Product1"],
    },
  ];

  const tableControls = useLocalTableControls({
    idProperty: "version",
    items: rows,
    columnNames: {
      version: "Version",
      vulnerabilities: "Vulnerabilities",
      products: "Products",
    },
    hasActionsColumn: true,
    expandableVariant: "compound",
    filterCategories: [
      {
        key: "q",
        title: "Version",
        type: FilterType.search,
        placeholderText: "Filter by version...",
      },
    ],
    sortableColumns: ["version"],
    getSortValues: (item) => ({
      version: item?.version || "",
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

  const [activeRowItem, setActiveRowItem] = React.useState<
    "vulnerability" | "product"
  >();

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
                <Th {...getThProps({ columnKey: "version" })} />
                <Th {...getThProps({ columnKey: "vulnerabilities" })} />
                <Th {...getThProps({ columnKey: "products" })} />
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
                <Tbody key={item.version} isExpanded={isCellExpanded(item)}>
                  <Tr>
                    <TableRowContentWithControls
                      {...tableControls}
                      item={item}
                      rowIndex={rowIndex}
                    >
                      <Td>{item.version}</Td>
                      <Td
                        {...getCompoundExpandTdProps({
                          item: item,
                          rowIndex,
                          columnKey: "vulnerabilities",
                        })}
                      >
                        <Flex
                          spaceItems={{ default: "spaceItemsSm" }}
                          alignItems={{ default: "alignItemsCenter" }}
                          flexWrap={{ default: "nowrap" }}
                          style={{ whiteSpace: "nowrap" }}
                        >
                          <FlexItem>
                            <Flex
                              spaceItems={{ default: "spaceItemsSm" }}
                              alignItems={{ default: "alignItemsCenter" }}
                              flexWrap={{ default: "nowrap" }}
                              style={{ whiteSpace: "nowrap" }}
                            >
                              <FlexItem>
                                <ShieldIcon color={lowColor.value} />
                              </FlexItem>
                              <FlexItem>0</FlexItem>
                            </Flex>
                          </FlexItem>
                          <FlexItem>
                            <Flex
                              spaceItems={{ default: "spaceItemsSm" }}
                              alignItems={{ default: "alignItemsCenter" }}
                              flexWrap={{ default: "nowrap" }}
                              style={{ whiteSpace: "nowrap" }}
                            >
                              <FlexItem>
                                <ShieldIcon color={moderateColor.value} />
                              </FlexItem>
                              <FlexItem>1</FlexItem>
                            </Flex>
                          </FlexItem>
                          <FlexItem>
                            <Flex
                              spaceItems={{ default: "spaceItemsSm" }}
                              alignItems={{ default: "alignItemsCenter" }}
                              flexWrap={{ default: "nowrap" }}
                              style={{ whiteSpace: "nowrap" }}
                            >
                              <FlexItem>
                                <ShieldIcon color={importantColor.value} />
                              </FlexItem>
                              <FlexItem>9</FlexItem>
                            </Flex>
                          </FlexItem>
                          <FlexItem>
                            <Flex
                              spaceItems={{ default: "spaceItemsSm" }}
                              alignItems={{ default: "alignItemsCenter" }}
                              flexWrap={{ default: "nowrap" }}
                              style={{ whiteSpace: "nowrap" }}
                            >
                              <FlexItem>
                                <ShieldIcon color={criticalColor.value} />
                              </FlexItem>
                              <FlexItem>6</FlexItem>
                            </Flex>
                          </FlexItem>
                        </Flex>
                      </Td>
                      <Td
                        {...getCompoundExpandTdProps({
                          item: item,
                          rowIndex,
                          columnKey: "products",
                        })}
                      >
                        {item.products.length}
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
                          {isCellExpanded(item, "vulnerabilities") && (
                            <div className="pf-v5-u-m-md">
                              <List
                                component={ListComponent.ol}
                                type={OrderType.number}
                              >
                                {item.vulnerabilities.map((e, vuln_index) => (
                                  <ListItem key={vuln_index}>
                                    <Button
                                      variant="link"
                                      onClick={() =>
                                        setActiveRowItem("vulnerability")
                                      }
                                    >
                                      {e}
                                    </Button>
                                  </ListItem>
                                ))}
                              </List>
                            </div>
                          )}
                          {isCellExpanded(item, "products") && (
                            <div className="pf-v5-u-m-md">
                              <List
                                component={ListComponent.ol}
                                type={OrderType.number}
                              >
                                {item.products.map((e, vuln_index) => (
                                  <ListItem key={vuln_index}>
                                    <Button
                                      variant="link"
                                      onClick={() =>
                                        setActiveRowItem("product")
                                      }
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
  cve: "vulnerability" | "product" | null;
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
            <Text component="small">{cve} details</Text>
            <Title headingLevel="h2" size="lg">
              Details of the {cve}
            </Title>
          </TextContent>
        </>
      )}
    </PageDrawerContent>
  );
};

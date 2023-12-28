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
  products: string[];
}

export const Vulnerabilities: React.FC = () => {
  const rows: RowData[] = [
    {
      name: "CVE-1",
      description: "description",
      products: ["Product1", "Product3"],
    },
    {
      name: "CVE-2",
      description: "description",
      products: ["Product7"],
    },
    {
      name: "CVE-3",
      description: "description",
      products: ["Product1"],
    },
  ];

  const tableControls = useLocalTableControls({
    idProperty: "name",
    items: rows,
    columnNames: {
      name: "Name",
      description: "Description",
      products: "Products",
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

  const [activeRowItem, setActiveRowItem] = React.useState<
    "vuln" | "product"
  >();

  return (
    <>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">Vulnerabilities</Text>
          <Text component="p">Search vulnerabilities</Text>
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
                        <Td {...getTdProps({ columnKey: "description" })}>
                          {item.description}
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
                            {isCellExpanded(item, "name") && <>CVE details</>}
                            {isCellExpanded(item, "products") && (
                              <>
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
                              </>
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
        </div>
      </PageSection>

      <DependencyAppsDetailDrawer
        entity={activeRowItem || null}
        onCloseClick={() => setActiveRowItem(undefined)}
      ></DependencyAppsDetailDrawer>
    </>
  );
};

export interface ICVEDetailDrawerProps
  extends Pick<IPageDrawerContentProps, "onCloseClick"> {
  entity: "vuln" | "product" | null;
}

export const DependencyAppsDetailDrawer: React.FC<ICVEDetailDrawerProps> = ({
  entity: entity,
  onCloseClick,
}) => {
  return (
    <PageDrawerContent
      isExpanded={!!entity}
      onCloseClick={onCloseClick}
      focusKey={entity || undefined}
      pageKey="analysis-app-dependencies"
      drawerPanelContentProps={{ defaultSize: "600px" }}
    >
      {!entity ? (
        <EmptyState variant={EmptyStateVariant.sm}>
          <EmptyStateIcon icon={CubesIcon} />
          <Title headingLevel="h2" size="lg">
            No data
          </Title>
        </EmptyState>
      ) : (
        <>
          <TextContent>
            <Text component="small">{entity} details</Text>
            <Title headingLevel="h2" size="lg">
              Details of the {entity}
            </Title>
          </TextContent>
        </>
      )}
    </PageDrawerContent>
  );
};

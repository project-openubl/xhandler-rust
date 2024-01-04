import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { NavLink, useParams } from "react-router-dom";

import {
  Button,
  ButtonVariant,
  PageSection,
  PageSectionVariants,
  Text,
  TextContent,
  Toolbar,
  ToolbarContent,
  ToolbarGroup,
  ToolbarItem,
} from "@patternfly/react-core";
import {
  ActionsColumn,
  Table,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
} from "@patternfly/react-table";

import { FilterToolbar, FilterType } from "@app/components/FilterToolbar";
import { SimplePagination } from "@app/components/SimplePagination";
import {
  ConditionalTableBody,
  TableHeaderContentWithControls,
  TableRowContentWithControls,
} from "@app/components/TableControls";

import { useLocalTableControls } from "@app/hooks/table-controls";
import { useFetchProjects } from "@app/queries/projects";

import { UploadFilesDrawer } from "./upload-files-drawer";

export const Projects: React.FC = () => {
  const { t } = useTranslation();
  const { projectId } = useParams();

  const [uploadFilesToProjectId, setUploadFilesToProjectId] = useState<
    string | number | null
  >(null);

  const { projects, isFetching, fetchError, refetch } = useFetchProjects();

  const tableControls = useLocalTableControls({
    idProperty: "id",
    items: projects,
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
        placeholderText: "Search by name...",
        getItemValue: (item) => item.name || "",
      },
    ],
    sortableColumns: ["name"],
    getSortValues: (item) => ({
      name: item?.name || "",
    }),
    isPaginationEnabled: true,
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
    },
  } = tableControls;

  return (
    <>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">{t("terms.documents")}</Text>
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
              <ToolbarGroup variant="button-group">
                <ToolbarItem>
                  <Button
                    type="button"
                    id="create-project"
                    aria-label="Create new project"
                    variant={ButtonVariant.primary}
                    // onClick={() => setCreateUpdateModalState("create")}
                  >
                    Crear proyecto
                  </Button>
                </ToolbarItem>
                <ToolbarItem>
                  <Button
                    type="button"
                    id="upload-files"
                    aria-label="Upload files"
                    variant={ButtonVariant.secondary}
                    onClick={() => setUploadFilesToProjectId(projectId || null)}
                  >
                    Upload files
                  </Button>
                </ToolbarItem>
              </ToolbarGroup>
              <ToolbarItem {...paginationToolbarItemProps}>
                <SimplePagination
                  idPrefix="documents-table"
                  isTop
                  paginationProps={paginationProps}
                />
              </ToolbarItem>
            </ToolbarContent>
          </Toolbar>

          <Table {...tableProps} aria-label="Projects table">
            <Thead>
              <Tr>
                <TableHeaderContentWithControls {...tableControls}>
                  <Th {...getThProps({ columnKey: "name" })} />
                  <Th {...getThProps({ columnKey: "description" })} />
                </TableHeaderContentWithControls>
              </Tr>
            </Thead>
            <ConditionalTableBody
              isLoading={isFetching}
              isError={!!fetchError}
              isNoData={projects.length === 0}
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
                        <Td width={15} {...getTdProps({ columnKey: "name" })}>
                          <NavLink to={`/projects/${item.id}`}>
                            {item.name}
                          </NavLink>
                        </Td>
                        <Td
                          width={20}
                          modifier="truncate"
                          {...getTdProps({ columnKey: "description" })}
                        >
                          {item.description}
                        </Td>
                        <Td isActionCell>
                          <ActionsColumn
                            items={
                              [
                                // {
                                //   title: "Editar",
                                //   onClick: () => setCreateUpdateModalState(item),
                                // },
                                // {
                                //   title: "Eliminar",
                                //   onClick: () => deleteRow(item),
                                // },
                              ]
                            }
                          />
                        </Td>
                      </TableRowContentWithControls>
                    </Tr>
                  </Tbody>
                );
              })}
            </ConditionalTableBody>
          </Table>

          <SimplePagination
            idPrefix="documents-table"
            isTop={false}
            paginationProps={paginationProps}
          />
        </div>

        <UploadFilesDrawer
          projectId={uploadFilesToProjectId}
          onCloseClick={() => setUploadFilesToProjectId(null)}
        />
      </PageSection>
    </>
  );
};

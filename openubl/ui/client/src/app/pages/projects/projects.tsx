import React, { useContext, useState } from "react";
import { NavLink } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { AxiosError } from "axios";

import {
  Button,
  ButtonVariant,
  Modal,
  ModalVariant,
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

import { SimplePagination } from "@app/components/SimplePagination";
import { FilterToolbar, FilterType } from "@app/components/FilterToolbar";
import {
  ConditionalTableBody,
  TableHeaderContentWithControls,
  TableRowContentWithControls,
} from "@app/components/TableControls";

import { useLocalTableControls } from "@app/hooks/table-controls";
import {
  useFetchProjects,
  useDeleteProjectMutation,
} from "@app/queries/projects";

import { Project } from "@app/api/models";
import { ProjectForm } from "./components/project-form";
import { ConfirmDialog } from "@app/components/ConfirmDialog";
import { NotificationsContext } from "@app/components/NotificationsContext";
import { getAxiosErrorMessage } from "@app/utils/utils";

export const Projects: React.FC = () => {
  const { t } = useTranslation();
  const { pushNotification } = useContext(NotificationsContext);

  const [isConfirmDialogOpen, setIsConfirmDialogOpen] =
    useState<boolean>(false);
  const [projectToDelete, setProjectToDelete] = React.useState<Project>();

  const [createUpdateModalState, setCreateUpdateModalState] = useState<
    "create" | Project | null
  >(null);
  const isCreateUpdateModalOpen = createUpdateModalState !== null;
  const projectToUpdate =
    createUpdateModalState !== "create" ? createUpdateModalState : null;

  const onDeleteOrgSuccess = () => {
    pushNotification({
      title: t("terms.projectDeleted"),
      variant: "success",
    });
  };

  const onDeleteOrgError = (error: AxiosError) => {
    pushNotification({
      title: getAxiosErrorMessage(error),
      variant: "danger",
    });
  };

  const { projects, isFetching, fetchError, refetch } = useFetchProjects();

  const { mutate: deleteproject } = useDeleteProjectMutation(
    onDeleteOrgSuccess,
    onDeleteOrgError
  );

  const tableControls = useLocalTableControls({
    idProperty: "id",
    items: projects,
    columnNames: {
      name: t("terms.name"),
      description: t("terms.description"),
    },
    hasActionsColumn: true,
    filterCategories: [
      {
        key: "q",
        title: t("terms.name"),
        type: FilterType.search,
        placeholderText:
          t("actions.filterBy", {
            what: t("terms.name").toLowerCase(),
          }) + "...",
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

  const closeCreateUpdateModal = () => {
    setCreateUpdateModalState(null);
    refetch;
  };

  const deleteRow = (row: Project) => {
    setProjectToDelete(row);
    setIsConfirmDialogOpen(true);
  };

  return (
    <>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">{t("terms.projects")}</Text>
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
                    onClick={() => setCreateUpdateModalState("create")}
                  >
                    {t("actions.createNew")}
                  </Button>
                </ToolbarItem>
              </ToolbarGroup>
              <ToolbarItem {...paginationToolbarItemProps}>
                <SimplePagination
                  idPrefix="projects-table"
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
                          <NavLink to={`/projects/${item.id}/documents`}>
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
                            items={[
                              {
                                title: t("actions.edit"),
                                onClick: () => setCreateUpdateModalState(item),
                              },
                              {
                                title: t("actions.delete"),
                                onClick: () => deleteRow(item),
                              },
                            ]}
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
            idPrefix="projects-table"
            isTop={false}
            paginationProps={paginationProps}
          />
        </div>
      </PageSection>

      <Modal
        id="create-edit-project-modal"
        title={t(projectToUpdate ? "dialog.title.update" : "dialog.title.new", {
          what: t("terms.project").toLowerCase(),
        })}
        variant={ModalVariant.medium}
        isOpen={isCreateUpdateModalOpen}
        onClose={closeCreateUpdateModal}
      >
        <ProjectForm
          project={projectToUpdate ? projectToUpdate : undefined}
          onClose={closeCreateUpdateModal}
        />
      </Modal>

      {isConfirmDialogOpen && (
        <ConfirmDialog
          title={t("dialog.title.deleteWithName", {
            what: t("terms.project").toLowerCase(),
            name: projectToDelete?.name,
          })}
          isOpen={true}
          titleIconVariant={"warning"}
          message={t("dialog.message.delete")}
          confirmBtnVariant={ButtonVariant.danger}
          confirmBtnLabel={t("actions.delete")}
          cancelBtnLabel={t("actions.cancel")}
          onCancel={() => setIsConfirmDialogOpen(false)}
          onClose={() => setIsConfirmDialogOpen(false)}
          onConfirm={() => {
            if (projectToDelete) {
              deleteproject(projectToDelete.id);
              setProjectToDelete(undefined);
            }
            setIsConfirmDialogOpen(false);
          }}
        />
      )}
    </>
  );
};

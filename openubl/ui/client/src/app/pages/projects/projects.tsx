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
  ToolbarContent,
  ToolbarGroup,
  ToolbarItem,
} from "@patternfly/react-core";
import { ActionsColumn, Td as TdAction } from "@patternfly/react-table";
import {
  useClientTableBatteries,
  ConditionalTableBody,
  FilterType,
} from "@mturley-latest/react-table-batteries";

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

  const tableControls = useClientTableBatteries({
    persistTo: "urlParams",
    idProperty: "id",
    items: projects,
    isLoading: isFetching,
    columnNames: {
      name: t("terms.name"),
      description: t("terms.description"),
    },
    hasActionsColumn: true,
    filter: {
      isEnabled: true,
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
    },
    sort: {
      isEnabled: true,
      sortableColumns: ["name"],
      getSortValues: (project) => ({
        name: project?.name || "",
      }),
    },
    pagination: { isEnabled: true },
  });

  const {
    currentPageItems,
    numRenderedColumns,
    components: {
      Table,
      Thead,
      Tr,
      Th,
      Tbody,
      Td,
      Toolbar,
      FilterToolbar,
      PaginationToolbarItem,
      Pagination,
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
          <Toolbar>
            <ToolbarContent>
              <FilterToolbar id="project-toolbar" />
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
              <PaginationToolbarItem>
                <Pagination
                  variant="top"
                  isCompact
                  widgetId="projects-pagination-top"
                />
              </PaginationToolbarItem>
            </ToolbarContent>
          </Toolbar>

          <Table aria-label="Projects table">
            <Thead>
              <Tr isHeaderRow>
                <Th columnKey="name" />
                <Th columnKey="description" />
              </Tr>
            </Thead>
            <ConditionalTableBody
              isLoading={isFetching}
              isError={!!fetchError}
              isNoData={projects.length === 0}
              numRenderedColumns={numRenderedColumns}
            >
              <Tbody>
                {currentPageItems?.map((item, rowIndex) => {
                  return (
                    <Tr key={item.name} item={item} rowIndex={rowIndex}>
                      <Td width={15} columnKey="name">
                        <NavLink to={`/projects/${item.id}/documents`}>
                          {item.name}
                        </NavLink>
                      </Td>
                      <Td
                        width={20}
                        modifier="truncate"
                        columnKey="description"
                      >
                        {item.description}
                      </Td>
                      <TdAction isActionCell>
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
                      </TdAction>
                    </Tr>
                  );
                })}
              </Tbody>
            </ConditionalTableBody>
          </Table>
          <Pagination
            variant="bottom"
            isCompact
            widgetId="projects-pagination-bottom"
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

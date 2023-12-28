import React, { useContext, useState } from "react";
import { NavLink } from "react-router-dom";
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

import { SimplePagination } from "@app/shared/components/SimplePagination";
import {
  FilterToolbar,
  FilterType,
} from "@app/shared/components/FilterToolbar";
import {
  ConditionalTableBody,
  TableHeaderContentWithControls,
  TableRowContentWithControls,
} from "@app/shared/components/TableControls";

import {
  getApiRequestParams,
  useTableControlProps,
  useTableControlUrlParams,
} from "@app/shared/hooks/table-controls";
import {
  useDeleteRepositoryMutation,
  useFetchRepositories,
} from "@app/queries/repositories";
import { useFetchOrganizations } from "@app/queries/organizations";

import { Repository } from "@app/api/models";
import { ConfirmDialog } from "@app/shared/components/ConfirmDialog";
import { NotificationsContext } from "@app/shared/components/NotificationsContext";
import { getAxiosErrorMessage } from "@app/utils/utils";

import { RepositoryForm } from "./components/repository-form";
import { TableURLParamKeyPrefix } from "@app/Constants";
import { useSelectionState } from "@app/shared/hooks/useSelectionState";

export const Repositories: React.FC = () => {
  const { pushNotification } = useContext(NotificationsContext);

  const [isConfirmDialogOpen, setIsConfirmDialogOpen] =
    useState<Boolean>(false);
  const [repositoryIdToDelete, setRepositoryIdToDelete] =
    React.useState<number>();

  const [createUpdateModalState, setCreateUpdateModalState] = useState<
    "create" | Repository | null
  >(null);
  const isCreateUpdateModalOpen = createUpdateModalState !== null;
  const repositoryToUpdate =
    createUpdateModalState !== "create" ? createUpdateModalState : null;

  //
  const onDeleteRepositorySuccess = (response: any) => {
    pushNotification({
      title: "Repository deleted",
      variant: "success",
    });
  };

  const onDeleteRepositoryError = (error: AxiosError) => {
    pushNotification({
      title: getAxiosErrorMessage(error),
      variant: "danger",
    });
  };

  //
  const { result: organizations, isFetching: isFetchingOrganizations } =
    useFetchOrganizations();

  const { mutate: deleteRepository } = useDeleteRepositoryMutation(
    onDeleteRepositorySuccess,
    onDeleteRepositoryError
  );

  const tableControlState = useTableControlUrlParams({
    urlParamKeyPrefix: TableURLParamKeyPrefix.repositories,
    columnNames: {
      name: "Name",
      organization: "Organization",
      description: "Description",
    },
    sortableColumns: ["name"],
    initialSort: { columnKey: "name", direction: "asc" },
    filterCategories: [
      {
        key: "q",
        title: "Name",
        type: FilterType.search,
        placeholderText: "Filter by name...",
        getServerFilterValue: (value) => (value ? [`*${value[0]}*`] : []),
      },
      {
        key: "organization",
        title: "Organization",
        type: FilterType.multiselect,
        placeholderText: "Organization",
        selectOptions: organizations.map((org) => ({
          key: org.id.toString(),
          value: org.name,
        })),
      },
    ],
    initialItemsPerPage: 10,
  });

  const {
    result: { data: currentPageRepositories, total: totalItemCount },
    isFetching,
    fetchError,
    refetch: refetchRepositories,
  } = useFetchRepositories(
    getApiRequestParams({
      ...tableControlState,
      apiSortFieldKeys: {
        name: "name",
      },
    })
  );

  const tableControls = useTableControlProps({
    ...tableControlState,
    idProperty: "id",
    currentPageItems: currentPageRepositories,
    totalItemCount,
    isLoading: isFetching,
    // TODO FIXME - we don't need selectionState but it's required by this hook?
    selectionState: useSelectionState({
      items: currentPageRepositories,
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
    },
  } = tableControls;

  const closeCreateUpdateModal = () => {
    setCreateUpdateModalState(null);
    refetchRepositories;
  };

  const deleteRow = (row: Repository) => {
    setRepositoryIdToDelete(row.id);
    setIsConfirmDialogOpen(true);
  };

  return (
    <>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">Repositories</Text>
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
              <FilterToolbar {...filterToolbarProps} showFiltersSideBySide />
              <ToolbarGroup variant="button-group">
                <ToolbarItem>
                  <Button
                    type="button"
                    id="create-repository"
                    aria-label="Create new repository"
                    variant={ButtonVariant.primary}
                    onClick={() => setCreateUpdateModalState("create")}
                  >
                    Create Repository
                  </Button>
                </ToolbarItem>
              </ToolbarGroup>
              <ToolbarItem {...paginationToolbarItemProps}>
                <SimplePagination
                  idPrefix="repositories-table"
                  isTop
                  paginationProps={paginationProps}
                />
              </ToolbarItem>
            </ToolbarContent>
          </Toolbar>

          <Table {...tableProps} aria-label="Repositories table">
            <Thead>
              <Tr>
                <TableHeaderContentWithControls {...tableControls}>
                  <Th {...getThProps({ columnKey: "name" })} />
                  <Th {...getThProps({ columnKey: "organization" })} />
                  <Th {...getThProps({ columnKey: "description" })} />
                </TableHeaderContentWithControls>
              </Tr>
            </Thead>
            <ConditionalTableBody
              isLoading={isFetching}
              isError={!!fetchError}
              isNoData={totalItemCount === 0}
              numRenderedColumns={numRenderedColumns}
            >
              <Tbody>
                {currentPageRepositories?.map((repository, rowIndex) => {
                  return (
                    <Tr key={repository.id}>
                      <TableRowContentWithControls
                        {...tableControls}
                        item={repository}
                        rowIndex={rowIndex}
                      >
                        <Td {...getTdProps({ columnKey: "name" })}>
                          <NavLink to={`/repositories/${repository.id}`}>
                            {repository.name}
                          </NavLink>
                        </Td>
                        <Td
                          modifier="truncate"
                          {...getTdProps({ columnKey: "organization" })}
                        >
                          {repository.organization.name}
                        </Td>
                        <Td
                          modifier="truncate"
                          {...getTdProps({ columnKey: "description" })}
                        >
                          {repository.description}
                        </Td>
                        <Td isActionCell>
                          <ActionsColumn
                            items={[
                              {
                                title: "Edit",
                                onClick: () =>
                                  setCreateUpdateModalState(repository),
                              },
                              {
                                title: "Delete",
                                onClick: () => deleteRow(repository),
                              },
                            ]}
                          />
                        </Td>
                      </TableRowContentWithControls>
                    </Tr>
                  );
                })}
              </Tbody>
            </ConditionalTableBody>
          </Table>

          <SimplePagination
            idPrefix="repositories-table"
            isTop={false}
            paginationProps={paginationProps}
          />
        </div>
      </PageSection>

      <Modal
        id="create-edit-repository-modal"
        title={repositoryToUpdate ? "Update repository" : "New repository"}
        variant={ModalVariant.medium}
        isOpen={isCreateUpdateModalOpen}
        onClose={closeCreateUpdateModal}
      >
        <RepositoryForm
          repository={repositoryToUpdate ? repositoryToUpdate : undefined}
          onClose={closeCreateUpdateModal}
        />
      </Modal>

      {isConfirmDialogOpen && (
        <ConfirmDialog
          title={"Delete repository"}
          isOpen={true}
          titleIconVariant={"warning"}
          message={`Are you sure you want to delete this repository?`}
          confirmBtnVariant={ButtonVariant.danger}
          confirmBtnLabel="Delete"
          cancelBtnLabel="Cancel"
          onCancel={() => setIsConfirmDialogOpen(false)}
          onClose={() => setIsConfirmDialogOpen(false)}
          onConfirm={() => {
            if (repositoryIdToDelete) {
              deleteRepository(repositoryIdToDelete);
              setRepositoryIdToDelete(undefined);
            }
            setIsConfirmDialogOpen(false);
          }}
        />
      )}
    </>
  );
};

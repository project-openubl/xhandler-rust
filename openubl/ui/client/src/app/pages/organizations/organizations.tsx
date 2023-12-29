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

import { useLocalTableControls } from "@app/shared/hooks/table-controls";
import {
  useDeleteOrganizationMutation,
  useFetchOrganizations,
} from "@app/queries/organizations";

import { Organization } from "@app/api/models";
import { OrganizationForm } from "./components/organization-form";
import { ConfirmDialog } from "@app/shared/components/ConfirmDialog";
import { NotificationsContext } from "@app/shared/components/NotificationsContext";
import { getAxiosErrorMessage } from "@app/utils/utils";

export const Organizations: React.FC = () => {
  const { pushNotification } = useContext(NotificationsContext);

  const [isConfirmDialogOpen, setIsConfirmDialogOpen] =
    useState<boolean>(false);
  const [orgIdToDelete, setOrgIdToDelete] = React.useState<number>();

  const [createUpdateModalState, setCreateUpdateModalState] = useState<
    "create" | Organization | null
  >(null);
  const isCreateUpdateModalOpen = createUpdateModalState !== null;
  const orgToUpdate =
    createUpdateModalState !== "create" ? createUpdateModalState : null;

  //
  const onDeleteOrgSuccess = (response: any) => {
    pushNotification({
      title: "Organization deleted",
      variant: "success",
    });
  };

  const onDeleteOrgError = (error: AxiosError) => {
    pushNotification({
      title: getAxiosErrorMessage(error),
      variant: "danger",
    });
  };

  //

  const {
    result: orgs,
    isFetching,
    fetchError,
    refetch,
  } = useFetchOrganizations();

  const { mutate: deleteOrg } = useDeleteOrganizationMutation(
    onDeleteOrgSuccess,
    onDeleteOrgError
  );

  const tableControls = useLocalTableControls({
    idProperty: "name",
    items: orgs,
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
        placeholderText: "Search",
        getItemValue: (item) => {
          return item.name || "";
        },
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
    },
  } = tableControls;

  const closeCreateUpdateModal = () => {
    setCreateUpdateModalState(null);
    refetch;
  };

  const deleteRow = (row: Organization) => {
    setOrgIdToDelete(row.id);
    setIsConfirmDialogOpen(true);
  };

  return (
    <>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">Organizations</Text>
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
                    id="create-organization"
                    aria-label="Create new organization"
                    variant={ButtonVariant.primary}
                    onClick={() => setCreateUpdateModalState("create")}
                  >
                    Create Organization
                  </Button>
                </ToolbarItem>
              </ToolbarGroup>
              <ToolbarItem {...paginationToolbarItemProps}>
                <SimplePagination
                  idPrefix="organizations-table"
                  isTop
                  paginationProps={paginationProps}
                />
              </ToolbarItem>
            </ToolbarContent>
          </Toolbar>

          <Table {...tableProps} aria-label="Organizations table">
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
              isNoData={orgs.length === 0}
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
                          <NavLink to={`/organizations/${item.name}`}>
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
                                title: "Edit",
                                onClick: () => setCreateUpdateModalState(item),
                              },
                              {
                                title: "Delete",
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
            idPrefix="organizations-table"
            isTop={false}
            paginationProps={paginationProps}
          />
        </div>
      </PageSection>

      <Modal
        id="create-edit-organization-modal"
        title={orgToUpdate ? "Update organization" : "New organization"}
        variant={ModalVariant.medium}
        isOpen={isCreateUpdateModalOpen}
        onClose={closeCreateUpdateModal}
      >
        <OrganizationForm
          organization={orgToUpdate ? orgToUpdate : undefined}
          onClose={closeCreateUpdateModal}
        />
      </Modal>

      {isConfirmDialogOpen && (
        <ConfirmDialog
          title={"Delete organization"}
          isOpen={true}
          titleIconVariant={"warning"}
          message={`Are you sure you want to delete this organization?`}
          confirmBtnVariant={ButtonVariant.danger}
          confirmBtnLabel="Delete"
          cancelBtnLabel="Cancel"
          onCancel={() => setIsConfirmDialogOpen(false)}
          onClose={() => setIsConfirmDialogOpen(false)}
          onConfirm={() => {
            if (orgIdToDelete) {
              deleteOrg(orgIdToDelete);
              setOrgIdToDelete(undefined);
            }
            setIsConfirmDialogOpen(false);
          }}
        />
      )}
    </>
  );
};

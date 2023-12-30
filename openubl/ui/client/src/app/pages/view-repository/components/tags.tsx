import React from "react";
import {
  ActionsColumn,
  Table,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
} from "@patternfly/react-table";
import {
  Button,
  ButtonVariant,
  Modal,
  ModalVariant,
  Toolbar,
  ToolbarContent,
  ToolbarGroup,
  ToolbarItem,
} from "@patternfly/react-core";

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

import { useFetchTags } from "@app/queries/tags";
import { Repository, TAG_STATUS } from "@app/api/models";
import { TagForm } from "./tag-form";

interface ITagsProps {
  repository: Repository;
}

export const Tags = ({ repository }: ITagsProps) => {
  const [isCreateModalOpen, setIsCreateModalOpen] = React.useState(false);

  const {
    result: tags,
    isFetching,
    fetchError,
    refetch: refetchTags,
  } = useFetchTags(repository.id);

  const tableControls = useLocalTableControls({
    idProperty: "id",
    items: tags,
    columnNames: {
      tag: "Tag",
      status: "Status",
    },
    hasActionsColumn: true,
    filterCategories: [
      {
        key: "q",
        title: "Name",
        type: FilterType.search,
        placeholderText: "Filter by name...",
        getServerFilterValue: (value) => (value ? [`*${value[0]}*`] : []),
      },
      {
        key: "status",
        title: "Status",
        type: FilterType.multiselect,
        placeholderText: "Status",
        selectOptions: TAG_STATUS.map((e) => ({
          key: e,
          value: e,
        })),
      },
    ],
    sortableColumns: ["tag"],
    getSortValues: (item) => ({
      tag: item?.tag || "",
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

  const closeCreateModal = () => {
    setIsCreateModalOpen(false);
    refetchTags;
  };

  return (
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
                onClick={() => setIsCreateModalOpen(true)}
              >
                Create Tag
              </Button>
            </ToolbarItem>
          </ToolbarGroup>
          <ToolbarItem {...paginationToolbarItemProps}>
            <SimplePagination
              idPrefix="tags-table"
              isTop
              paginationProps={paginationProps}
            />
          </ToolbarItem>
        </ToolbarContent>
      </Toolbar>

      <Table {...tableProps} aria-label="Tags table">
        <Thead>
          <Tr>
            <TableHeaderContentWithControls {...tableControls}>
              <Th {...getThProps({ columnKey: "tag" })} />
              <Th {...getThProps({ columnKey: "status" })} />
            </TableHeaderContentWithControls>
          </Tr>
        </Thead>
        <ConditionalTableBody
          isLoading={isFetching}
          isError={!!fetchError}
          isNoData={tags.length === 0}
          numRenderedColumns={numRenderedColumns}
        >
          <Tbody>
            {currentPageItems?.map((tag, rowIndex) => {
              return (
                <Tr key={tag.id}>
                  <TableRowContentWithControls
                    {...tableControls}
                    item={tag}
                    rowIndex={rowIndex}
                  >
                    <Td {...getTdProps({ columnKey: "tag" })}>{tag.tag}</Td>
                    <Td
                      modifier="truncate"
                      {...getTdProps({ columnKey: "status" })}
                    >
                      {tag.status}
                    </Td>

                    <Td isActionCell>
                      <ActionsColumn
                        items={[
                          {
                            title: "Edit",
                            onClick: () => {},
                          },
                          {
                            title: "Delete",
                            onClick: () => {},
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
        idPrefix="tags-table"
        isTop={false}
        paginationProps={paginationProps}
      />

      <Modal
        id="create-tag-modal"
        title={"New tag"}
        variant={ModalVariant.medium}
        isOpen={isCreateModalOpen}
        onClose={closeCreateModal}
      >
        <TagForm repository={repository} onClose={closeCreateModal} />
      </Modal>
    </div>
  );
};

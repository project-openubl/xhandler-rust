import { ToolbarItemProps, ToolbarProps } from "@patternfly/react-core";
import {
  TableProps,
  TdProps,
  ThProps,
  TrProps,
} from "@patternfly/react-table";
import spacing from "@patternfly/react-styles/css/utilities/Spacing/spacing";

import { IToolbarBulkSelectorProps } from "@app/shared/components/ToolbarBulkSelector/toolbar-bulk-selector";
import { objectKeys } from "@app/utils/utils";
import { IUseTableControlPropsArgs } from "./types";
import { getFilterProps } from "./filtering";
import { getSortProps } from "./sorting";
import { getPaginationProps, usePaginationEffects } from "./pagination";
import { getActiveRowDerivedState, useActiveRowEffects } from "./active-row";
import { handlePropagatedRowClick } from "./utils";
import { getExpansionDerivedState } from "./expansion";

export const useTableControlProps = <
  TItem,
  TColumnKey extends string,
  TSortableColumnKey extends TColumnKey,
  TFilterCategoryKey extends string = string
>(
  args: IUseTableControlPropsArgs<
    TItem,
    TColumnKey,
    TSortableColumnKey,
    TFilterCategoryKey
  >
) => {
  // Note: To avoid repetition, not all args are destructured here since the entire
  //       args object is passed to other other helpers which require other parts of it.
  //       For future additions, inspect `args` to see if it has anything more you need.
  const {
    currentPageItems,
    forceNumRenderedColumns,
    filterState: { setFilterValues },
    expansionState: { expandedCells },
    selectionState: {
      selectAll,
      areAllSelected,
      selectedItems,
      selectMultiple,
      toggleItemSelected,
      isItemSelected,
    },
    columnNames,
    sortableColumns = [],
    isSelectable = false,
    expandableVariant = null,
    hasActionsColumn = false,
    hasClickableRows = false,
    variant,
    idProperty,
  } = args;

  const columnKeys = objectKeys(columnNames);

  // Some table controls rely on extra columns inserted before or after the ones included in columnNames.
  // We need to account for those when dealing with props based on column index and colSpan.
  let numColumnsBeforeData = 0;
  let numColumnsAfterData = 0;
  if (isSelectable) numColumnsBeforeData++;
  if (expandableVariant === "single") numColumnsBeforeData++;
  if (hasActionsColumn) numColumnsAfterData++;
  const numRenderedColumns =
    forceNumRenderedColumns ||
    columnKeys.length + numColumnsBeforeData + numColumnsAfterData;

  const expansionDerivedState = getExpansionDerivedState(args);
  const { isCellExpanded, setCellExpanded } = expansionDerivedState;

  const activeRowDerivedState = getActiveRowDerivedState(args);
  useActiveRowEffects({ ...args, activeRowDerivedState });
  const { activeRowItem, setActiveRowItem, clearActiveRow } =
    activeRowDerivedState;

  const toolbarProps: Omit<ToolbarProps, "ref"> = {
    className: variant === "compact" ? spacing.pt_0 : "",
    collapseListedFiltersBreakpoint: "xl",
    clearAllFilters: () => setFilterValues({}),
    clearFiltersButtonText: "Clear all filters",
  };

  const filterToolbarProps = getFilterProps(args);

  const paginationProps = getPaginationProps(args);
  usePaginationEffects(args);

  const paginationToolbarItemProps: ToolbarItemProps = {
    variant: "pagination",
    align: { default: "alignRight" },
  };

  const toolbarBulkSelectorProps: IToolbarBulkSelectorProps<TItem> = {
    onSelectAll: selectAll,
    areAllSelected,
    selectedRows: selectedItems,
    paginationProps,
    currentPageItems,
    onSelectMultiple: selectMultiple,
  };

  const tableProps: Omit<TableProps, "ref"> = {
    variant,
    // TODO PF V5 obsolete  
    // hasSelectableRowCaption: hasClickableRows,
  };

  const getThProps = ({
    columnKey,
  }: {
    columnKey: TColumnKey;
  }): Omit<ThProps, "ref"> => ({
    ...(sortableColumns.includes(columnKey as TSortableColumnKey)
      ? getSortProps({
          ...args,
          columnKeys,
          columnKey: columnKey as TSortableColumnKey,
        })
      : {}),
    children: columnNames[columnKey],
  });

  const getClickableTrProps = ({
    onRowClick,
    item,
  }: {
    onRowClick?: TrProps["onRowClick"]; // Extra callback if necessary - setting the active row is built in
    item?: TItem; // Can be omitted if using this just for the click handler and not for active rows
  }): Omit<TrProps, "ref"> => ({
    isSelectable: true,
    isClickable: true,
    isRowSelected: item && item[idProperty] === activeRowItem?.[idProperty],
    onRowClick: (event) =>
      handlePropagatedRowClick(event, () => {
        if (item && activeRowItem?.[idProperty] !== item[idProperty]) {
          setActiveRowItem(item);
        } else {
          clearActiveRow();
        }
        onRowClick?.(event);
      }),
  });

  const getTdProps = ({
    columnKey,
  }: {
    columnKey: TColumnKey;
  }): Omit<TdProps, "ref"> => ({
    dataLabel: columnNames[columnKey],
  });

  const getSelectCheckboxTdProps = ({
    item,
    rowIndex,
  }: {
    item: TItem;
    rowIndex: number;
  }): Omit<TdProps, "ref"> => ({
    select: {
      rowIndex,
      onSelect: (_event, isSelecting) => {
        toggleItemSelected(item, isSelecting);
      },
      isSelected: isItemSelected(item),
    },
  });

  const getSingleExpandTdProps = ({
    item,
    rowIndex,
  }: {
    item: TItem;
    rowIndex: number;
  }): Omit<TdProps, "ref"> => ({
    expand: {
      rowIndex,
      isExpanded: isCellExpanded(item),
      onToggle: () =>
        setCellExpanded({
          item,
          isExpanding: !isCellExpanded(item),
        }),
      expandId: `expandable-row-${item[idProperty]}`,
    },
  });

  const getCompoundExpandTdProps = ({
    item,
    rowIndex,
    columnKey,
  }: {
    item: TItem;
    rowIndex: number;
    columnKey: TColumnKey;
  }): Omit<TdProps, "ref"> => ({
    ...getTdProps({ columnKey }),
    compoundExpand: {
      isExpanded: isCellExpanded(item, columnKey),
      onToggle: () =>
        setCellExpanded({
          item,
          isExpanding: !isCellExpanded(item, columnKey),
          columnKey,
        }),
      expandId: `compound-expand-${item[idProperty]}-${columnKey}`,
      rowIndex,
      columnIndex: columnKeys.indexOf(columnKey),
    },
  });

  const getExpandedContentTdProps = ({
    item,
  }: {
    item: TItem;
  }): Omit<TdProps, "ref"> => {
    const expandedColumnKey = expandedCells[String(item[idProperty])];
    return {
      dataLabel:
        typeof expandedColumnKey === "string"
          ? columnNames[expandedColumnKey]
          : undefined,
      noPadding: true,
      colSpan: numRenderedColumns,
      width: 100,
    };
  };

  return {
    ...args,
    numColumnsBeforeData,
    numColumnsAfterData,
    numRenderedColumns,
    propHelpers: {
      toolbarProps,
      toolbarBulkSelectorProps,
      filterToolbarProps,
      paginationProps,
      paginationToolbarItemProps,
      tableProps,
      getThProps,
      getClickableTrProps,
      getTdProps,
      getSelectCheckboxTdProps,
      getCompoundExpandTdProps,
      getSingleExpandTdProps,
      getExpandedContentTdProps,
    },
    expansionDerivedState,
    activeRowDerivedState,
  };
};

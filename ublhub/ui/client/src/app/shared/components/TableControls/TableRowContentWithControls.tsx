import React from "react";
import { Td } from "@patternfly/react-table";
import { useTableControlProps } from "@app/shared/hooks/table-controls";

export interface ITableRowContentWithControlsProps<
  TItem,
  TColumnKey extends string,
  TSortableColumnKey extends TColumnKey
> {
  expandableVariant?: "single" | "compound" | null;
  isSelectable?: boolean;
  propHelpers: ReturnType<
    typeof useTableControlProps<TItem, TColumnKey, TSortableColumnKey>
  >["propHelpers"];
  item: TItem;
  rowIndex: number;
  children: React.ReactNode;
}

export const TableRowContentWithControls = <
  TItem,
  TColumnKey extends string,
  TSortableColumnKey extends TColumnKey
>({
  expandableVariant = null,
  isSelectable = false,
  propHelpers: { getSingleExpandTdProps, getSelectCheckboxTdProps },
  item,
  rowIndex,
  children,
}: React.PropsWithChildren<
  ITableRowContentWithControlsProps<TItem, TColumnKey, TSortableColumnKey>
>) => (
  <>
    {expandableVariant === "single" ? (
      <Td {...getSingleExpandTdProps({ item, rowIndex })} />
    ) : null}
    {isSelectable ? (
      <Td {...getSelectCheckboxTdProps({ item, rowIndex })} />
    ) : null}
    {children}
  </>
);

import { KeyWithValueType } from "@app/utils/type-utils";
import { IExpansionState } from "./useExpansionState";

export interface IExpansionDerivedStateArgs<TItem, TColumnKey extends string> {
  idProperty: KeyWithValueType<TItem, string | number>;
  expansionState: IExpansionState<TColumnKey>;
}

// Note: This is not named `getLocalExpansionDerivedState` because it is always local,
//       and it is still used when working with server-managed tables.
export const getExpansionDerivedState = <TItem, TColumnKey extends string>({
  idProperty,
  expansionState: { expandedCells, setExpandedCells },
}: IExpansionDerivedStateArgs<TItem, TColumnKey>) => {
  // isCellExpanded:
  //  - If called with a columnKey, returns whether that specific cell is expanded
  //  - If called without a columnKey, returns whether the row is expanded at all
  const isCellExpanded = (item: TItem, columnKey?: TColumnKey) => {
    return columnKey
      ? expandedCells[String(item[idProperty])] === columnKey
      : !!expandedCells[String(item[idProperty])];
  };

  // setCellExpanded:
  //  - If called with a columnKey, sets that column expanded or collapsed (use for compound-expand)
  //  - If called without a columnKey, sets the entire row as expanded or collapsed (use for single-expand)
  const setCellExpanded = ({
    item,
    isExpanding = true,
    columnKey,
  }: {
    item: TItem;
    isExpanding?: boolean;
    columnKey?: TColumnKey;
  }) => {
    const newExpandedCells = { ...expandedCells };
    if (isExpanding) {
      newExpandedCells[String(item[idProperty])] = columnKey || true;
    } else {
      delete newExpandedCells[String(item[idProperty])];
    }
    setExpandedCells(newExpandedCells);
  };

  return { isCellExpanded, setCellExpanded };
};

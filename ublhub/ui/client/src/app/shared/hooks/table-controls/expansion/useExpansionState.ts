import React from "react";
import { useUrlParams } from "../../useUrlParams";
import { objectKeys } from "@app/utils/utils";
import { IExtraArgsForURLParamHooks } from "../types";

// TExpandedCells maps item[idProperty] values to either:
//  - The key of an expanded column in that row, if the table is compound-expandable
//  - The `true` literal value (the entire row is expanded), if non-compound-expandable
export type TExpandedCells<TColumnKey extends string> = Record<
  string,
  TColumnKey | boolean
>;

export interface IExpansionState<TColumnKey extends string> {
  expandedCells: Record<string, boolean | TColumnKey>;
  setExpandedCells: (
    newExpandedCells: Record<string, boolean | TColumnKey>
  ) => void;
}

export const useExpansionState = <
  TColumnKey extends string
>(): IExpansionState<TColumnKey> => {
  const [expandedCells, setExpandedCells] = React.useState<
    TExpandedCells<TColumnKey>
  >({});
  return { expandedCells, setExpandedCells };
};

export const useExpansionUrlParams = <
  TColumnKey extends string,
  TURLParamKeyPrefix extends string = string
>({
  urlParamKeyPrefix,
}: IExtraArgsForURLParamHooks<TURLParamKeyPrefix> = {}): IExpansionState<TColumnKey> => {
  const [expandedCells, setExpandedCells] = useUrlParams({
    keyPrefix: urlParamKeyPrefix,
    keys: ["expandedCells"],
    defaultValue: {} as TExpandedCells<TColumnKey>,
    serialize: (expandedCellsObj) => {
      if (objectKeys(expandedCellsObj).length === 0)
        return { expandedCells: null };
      return { expandedCells: JSON.stringify(expandedCellsObj) };
    },
    deserialize: ({ expandedCells: expandedCellsStr }) => {
      try {
        return JSON.parse(expandedCellsStr || "{}");
      } catch (e) {
        return {};
      }
    },
  });
  return { expandedCells, setExpandedCells };
};

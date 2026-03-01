import { useEffect } from "react";

import { useLocalStorage } from "@app/hooks/useStorage";

import type { ColumnSetting } from "../types";

export interface ColumnState<TColumnKey extends string> {
  id: TColumnKey;
  label: string;
  isVisible: boolean;
  isIdentity?: boolean;
}

export interface IColumnState<TColumnKey extends string> {
  columns: ColumnState<TColumnKey>[];
  defaultColumns: ColumnState<TColumnKey>[];
  setColumns: (newColumns: ColumnState<TColumnKey>[]) => void;
}

interface IColumnStateArgs<TColumnKey extends string> {
  initialColumns?: Partial<Record<TColumnKey, ColumnSetting>>;
  columnsKey: string;
  supportedColumns: Record<TColumnKey, string>;
}

export const useColumnState = <TColumnKey extends string>({
  initialColumns,
  supportedColumns,
  columnsKey,
}: IColumnStateArgs<TColumnKey>): IColumnState<TColumnKey> => {
  const defaultColumns = (
    Object.entries(supportedColumns) as [TColumnKey, string][]
  ).map(([id, label]) => ({
    id,
    label,
    isVisible: initialColumns?.[id]?.isVisible ?? true,
    isIdentity: initialColumns?.[id]?.isIdentity,
  }));
  const [columns, setColumns] = useLocalStorage<ColumnState<TColumnKey>[]>({
    key: columnsKey,
    defaultValue: defaultColumns,
  });

  useEffect(() => {
    const valid = columns.filter(({ id }) =>
      defaultColumns.find((it) => id === it.id),
    );
    if (valid.length !== defaultColumns.length) {
      setColumns(
        defaultColumns.map((it) => ({
          ...it,
          isVisible:
            valid.find(({ id }) => id === it.id)?.isVisible ?? it.isVisible,
        })),
      );
    }
  }, [defaultColumns, columns, setColumns]);

  return { columns, setColumns, defaultColumns };
};

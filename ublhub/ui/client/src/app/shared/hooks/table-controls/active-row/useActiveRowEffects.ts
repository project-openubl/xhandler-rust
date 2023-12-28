import * as React from "react";
import { getActiveRowDerivedState } from "./getActiveRowDerivedState";
import { IActiveRowState } from "./useActiveRowState";

interface IUseActiveRowEffectsArgs<TItem> {
  isLoading?: boolean;
  activeRowState: IActiveRowState;
  activeRowDerivedState: ReturnType<typeof getActiveRowDerivedState<TItem>>;
}

export const useActiveRowEffects = <TItem>({
  isLoading,
  activeRowState: { activeRowId },
  activeRowDerivedState: { activeRowItem, clearActiveRow },
}: IUseActiveRowEffectsArgs<TItem>) => {
  React.useEffect(() => {
    // If some state change (e.g. refetch, pagination) causes the active row to disappear,
    // remove its id from state so the drawer won't automatically reopen if the row comes back.
    if (!isLoading && activeRowId && !activeRowItem) {
      clearActiveRow();
    }
  }, [isLoading, activeRowId, activeRowItem]);
};

import { IPaginationState } from "./usePaginationState";

export interface ILocalPaginationDerivedStateArgs<TItem> {
  items: TItem[];
}

export const getLocalPaginationDerivedState = <TItem>({
  items,
  paginationState: { pageNumber, itemsPerPage },
}: ILocalPaginationDerivedStateArgs<TItem> & {
  paginationState: IPaginationState;
}) => {
  const pageStartIndex = (pageNumber - 1) * itemsPerPage;
  const currentPageItems = items.slice(
    pageStartIndex,
    pageStartIndex + itemsPerPage
  );
  return { currentPageItems };
};

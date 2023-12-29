import * as React from "react";
import { useUrlParams } from "../../useUrlParams";
import { IExtraArgsForURLParamHooks } from "../types";

export interface IPaginationState {
  pageNumber: number;
  setPageNumber: (pageNumber: number) => void;
  itemsPerPage: number;
  setItemsPerPage: (numItems: number) => void;
}

export interface IPaginationStateArgs {
  initialItemsPerPage?: number;
}

export const usePaginationState = ({
  initialItemsPerPage = 10,
}: IPaginationStateArgs): IPaginationState => {
  const [pageNumber, baseSetPageNumber] = React.useState(1);
  const setPageNumber = (num: number) => baseSetPageNumber(num >= 1 ? num : 1);
  const [itemsPerPage, setItemsPerPage] = React.useState(initialItemsPerPage);
  return { pageNumber, setPageNumber, itemsPerPage, setItemsPerPage };
};

export const usePaginationUrlParams = <
  TURLParamKeyPrefix extends string = string
>({
  initialItemsPerPage = 10,
  urlParamKeyPrefix,
}: IPaginationStateArgs &
  IExtraArgsForURLParamHooks<TURLParamKeyPrefix>): IPaginationState => {
  const defaultValue = { pageNumber: 1, itemsPerPage: initialItemsPerPage };
  const [paginationState, setPaginationState] = useUrlParams({
    keyPrefix: urlParamKeyPrefix,
    keys: ["pageNumber", "itemsPerPage"],
    defaultValue,
    serialize: ({ pageNumber, itemsPerPage }) => ({
      pageNumber: pageNumber ? String(pageNumber) : undefined,
      itemsPerPage: itemsPerPage ? String(itemsPerPage) : undefined,
    }),
    deserialize: ({ pageNumber, itemsPerPage }) =>
      pageNumber && itemsPerPage
        ? {
            pageNumber: parseInt(pageNumber, 10),
            itemsPerPage: parseInt(itemsPerPage, 10),
          }
        : defaultValue,
  });

  const setPageNumber = (pageNumber: number) =>
    setPaginationState({ pageNumber: pageNumber >= 1 ? pageNumber : 1 });
  const setItemsPerPage = (itemsPerPage: number) =>
    setPaginationState({ itemsPerPage });

  const { pageNumber, itemsPerPage } = paginationState;
  return {
    pageNumber: pageNumber || 1,
    itemsPerPage: itemsPerPage || initialItemsPerPage,
    setPageNumber,
    setItemsPerPage,
  };
};

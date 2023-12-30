export type New<T extends { id: number }> = Omit<T, "id">;
export type Only<T extends { id: number }> = Pick<T, "id">;

export interface ApiFilter {
  field: string;
  operator?: "=" | "!=" | "~" | ">" | ">=" | "<" | "<=";
  value:
    | string
    | number
    | {
        list: (string | number)[];
        operator?: "AND" | "OR";
      };
}

export interface ApiRequestParams {
  filters?: ApiFilter[];
  sort?: {
    field: string;
    direction: "asc" | "desc";
  };
  page?: {
    pageNumber: number; // 1-indexed
    itemsPerPage: number;
  };
}

export interface ApiPaginatedResult<T> {
  data: T[];
  total: number;
  params: ApiRequestParams;
}

//

export interface Project {
  id: number;
  name: string;
  description?: string;
}

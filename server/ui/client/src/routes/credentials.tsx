import { createFileRoute } from "@tanstack/react-router";
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";

import {
  Button,
  ButtonVariant,
  Content,
  PageSection,
  Toolbar,
  ToolbarContent,
  ToolbarGroup,
  ToolbarItem,
} from "@patternfly/react-core";

import { ConditionalTableBody } from "@app/components/TableControls/ConditionalTableBody";
import { Table, Tbody, Td, Th, Thead, Tr } from "@patternfly/react-table";
import { useFetchCredentials } from "@queries/credentials.ts";

export const Route = createFileRoute("/credentials")({
  component: CredentialComponent,
});

type CredentialColumn = {
  name: string;
  description: string;
  ruc: string;
};
// type CredentialColumnKeys = keyof CredentialColumn;

const columnHelper = createColumnHelper<CredentialColumn>();

function CredentialComponent() {
  const { credentials, isFetching, fetchError } = useFetchCredentials();

  // const [columnVisibility, setColumnVisibility] = React.useState({
  //   columnId1: true,
  //   columnId2: false, //hide this column by default
  //   columnId3: true,
  // });

  columnHelper.accessor("name", {});
  const table = useReactTable({
    columns: [
      columnHelper.accessor("name", { header: "Nombre" }),
      columnHelper.accessor("description", { header: "Descripción" }),
      columnHelper.accessor("ruc", { header: "RUC" }),
    ],
    data: credentials,
    getCoreRowModel: getCoreRowModel(),
    // state: {
    //   columnVisibility,
    // },
  });

  return (
    <>
      <PageSection variant="default">
        <Content>
          <h1>Credenciales</h1>
          <p>Credenciales usadas para enviar documentos</p>
        </Content>
      </PageSection>
      <PageSection variant="secondary">
        <Toolbar
          collapseListedFiltersBreakpoint="xl"
          clearFiltersButtonText="Clear all filters"
        >
          <ToolbarContent>
            <ToolbarGroup variant="action-group">
              <ToolbarItem>
                <Button
                  type="button"
                  id="create-credential"
                  aria-label="Create Credential"
                  variant={ButtonVariant.primary}
                  onClick={() => {
                    // setSaveApplicationModalState("create");
                  }}
                >
                  Create Credential
                </Button>
              </ToolbarItem>
            </ToolbarGroup>

            {/* <ToolbarItem {...paginationToolbarItemProps}>
              <SimplePagination
                idPrefix="app-assessments-table"
                isTop
                paginationProps={paginationProps}
              />
            </ToolbarItem> */}
          </ToolbarContent>
        </Toolbar>

        <Table aria-label="Credentials table">
          <Thead>
            {table.getHeaderGroups().map((headerGroup) => (
              <Tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <Th key={header.id}>
                    {flexRender(
                      header.column.columnDef.header,
                      header.getContext(),
                    )}
                  </Th>
                ))}
              </Tr>
            ))}
          </Thead>
          <ConditionalTableBody
            isLoading={isFetching}
            isError={!!fetchError}
            isNoData={credentials.length === 0}
            numRenderedColumns={1}
          >
            <Tbody>
              {table.getRowModel().rows.map((row) => (
                <Tr key={row.id}>
                  {row.getVisibleCells().map((cell) => (
                    <Td key={cell.id} dataLabel={cell.id}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext(),
                      )}
                    </Td>
                  ))}
                </Tr>
              ))}
            </Tbody>
          </ConditionalTableBody>
        </Table>
      </PageSection>
    </>
  );
}

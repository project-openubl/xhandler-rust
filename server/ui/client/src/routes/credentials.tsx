import { createFileRoute } from "@tanstack/react-router";
import {
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";

import { Content, PageSection } from "@patternfly/react-core";

import { Table, Tbody, Td, Th, Thead, Tr } from "@patternfly/react-table";
import { useFetchCredentials } from "@queries/credentials.ts";

export const Route = createFileRoute("/credentials")({
  component: CredentialComponent,
});

function CredentialComponent() {
  const { credentials } = useFetchCredentials();

  const table = useReactTable({
    columns: [
      {
        id: "name",
        header: "Nombre",
        accessorFn: (row) => row.name,
      },
      {
        id: "description",
        header: "Descripción",
        accessorFn: (row) => row.description,
      },
      {
        id: "ruc",
        header: "RUC",
        accessorFn: (row) => row.supplier_ids_applied_to,
      },
    ],
    data: credentials,
    getCoreRowModel: getCoreRowModel(),
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
          <Tbody>
            {table.getRowModel().rows.map((row) => (
              <Tr key={row.id}>
                {row.getVisibleCells().map((cell) => (
                  <Td key={cell.id} dataLabel={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </Td>
                ))}
              </Tr>
            ))}
          </Tbody>
        </Table>
      </PageSection>
    </>
  );
}

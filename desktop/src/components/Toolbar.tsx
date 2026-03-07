import {
  Toolbar as PfToolbar,
  ToolbarContent,
  ToolbarItem,
  ToolbarGroup,
  Button,
  Select,
  SelectOption,
  MenuToggle,
  MenuToggleElement,
  Switch,
  Label,
} from "@patternfly/react-core";
import { useState } from "react";

export type DocumentKind =
  | "Invoice"
  | "CreditNote"
  | "DebitNote"
  | "DespatchAdvice"
  | "Perception"
  | "Retention"
  | "SummaryDocuments"
  | "VoidedDocuments";

const DOCUMENT_KINDS: { value: DocumentKind; label: string }[] = [
  { value: "Invoice", label: "Invoice (Factura/Boleta)" },
  { value: "CreditNote", label: "CreditNote (Nota de Credito)" },
  { value: "DebitNote", label: "DebitNote (Nota de Debito)" },
  { value: "DespatchAdvice", label: "DespatchAdvice (Guia de Remision)" },
  { value: "Perception", label: "Perception (Percepcion)" },
  { value: "Retention", label: "Retention (Retencion)" },
  { value: "SummaryDocuments", label: "SummaryDocuments (Resumen Diario)" },
  { value: "VoidedDocuments", label: "VoidedDocuments (Comunicacion de Baja)" },
];

interface ToolbarProps {
  selectedKind: DocumentKind;
  onKindChange: (kind: DocumentKind) => void;
  onCreateXml: () => void;
  onSignXml: () => void;
  onSendXml: () => void;
  onApply: () => void;
  onSettingsClick: () => void;
  canCreate: boolean;
  canSign: boolean;
  canSend: boolean;
  loading: boolean;
  beta: boolean;
  onBetaChange: (checked: boolean) => void;
}

export default function Toolbar({
  selectedKind,
  onKindChange,
  onCreateXml,
  onSignXml,
  onSendXml,
  onApply,
  onSettingsClick,
  canCreate,
  canSign,
  canSend,
  loading,
  beta,
  onBetaChange,
}: ToolbarProps) {
  const [kindOpen, setKindOpen] = useState(false);

  return (
    <PfToolbar>
      <ToolbarContent>
        <ToolbarItem>
          <Select
            isOpen={kindOpen}
            onOpenChange={setKindOpen}
            onSelect={(_event, value) => {
              onKindChange(value as DocumentKind);
              setKindOpen(false);
            }}
            selected={selectedKind}
            toggle={(toggleRef: React.Ref<MenuToggleElement>) => (
              <MenuToggle
                ref={toggleRef}
                onClick={() => setKindOpen(!kindOpen)}
                isExpanded={kindOpen}
              >
                {DOCUMENT_KINDS.find((k) => k.value === selectedKind)?.label ??
                  selectedKind}
              </MenuToggle>
            )}
          >
            {DOCUMENT_KINDS.map((kind) => (
              <SelectOption key={kind.value} value={kind.value}>
                {kind.label}
              </SelectOption>
            ))}
          </Select>
        </ToolbarItem>

        <ToolbarGroup>
          <ToolbarItem>
            <Button
              variant="primary"
              onClick={onCreateXml}
              isDisabled={!canCreate || loading}
              isLoading={loading}
            >
              Crear XML
            </Button>
          </ToolbarItem>
          <ToolbarItem>
            <Button
              variant="secondary"
              onClick={onSignXml}
              isDisabled={!canSign || loading}
            >
              Firmar XML
            </Button>
          </ToolbarItem>
          <ToolbarItem>
            <Button
              variant="secondary"
              onClick={onSendXml}
              isDisabled={!canSend || loading}
            >
              Enviar
            </Button>
          </ToolbarItem>
          <ToolbarItem>
            <Button
              variant="tertiary"
              onClick={onApply}
              isDisabled={!canCreate || loading}
            >
              Aplicar todo
            </Button>
          </ToolbarItem>
        </ToolbarGroup>

        <ToolbarGroup align={{ default: "alignEnd" }}>
          <ToolbarItem>
            <Switch
              id="beta-toggle"
              label="Beta"
              isChecked={beta}
              onChange={(_event, checked) => onBetaChange(checked)}
            />
          </ToolbarItem>
          {beta && (
            <ToolbarItem>
              <Label color="orange">BETA</Label>
            </ToolbarItem>
          )}
          <ToolbarItem>
            <Button variant="plain" onClick={onSettingsClick}>
              Configuracion
            </Button>
          </ToolbarItem>
        </ToolbarGroup>
      </ToolbarContent>
    </PfToolbar>
  );
}

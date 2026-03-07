import { useState, useCallback } from "react";
import {
  Page,
  PageSection,
  Alert,
  AlertGroup,
  AlertVariant,
  Split,
  SplitItem,
} from "@patternfly/react-core";

import Toolbar from "./components/Toolbar";
import type { DocumentKind } from "./components/Toolbar";
import DocumentEditor from "./components/DocumentEditor";
import OutputPanel from "./components/OutputPanel";
import SettingsModal, {
  DEFAULT_SETTINGS,
  type Settings,
} from "./components/SettingsModal";
import { usePipeline } from "./hooks/usePipeline";
import type { PipelineStep } from "./hooks/usePipeline";
import { createXml, signXml, sendXml } from "./tauri";
import type { SendConfig } from "./tauri";

const TEMPLATES: Record<DocumentKind, string> = {
  Invoice: JSON.stringify(
    {
      kind: "Invoice",
      spec: {
        serie_numero: "F001-1",
        proveedor: { ruc: "", razon_social: "" },
        cliente: {
          tipo_documento_identidad: "6",
          numero_documento_identidad: "",
          nombre: "",
        },
        detalles: [{ descripcion: "", cantidad: 1, precio: 0 }],
      },
    },
    null,
    2
  ),
  CreditNote: JSON.stringify(
    {
      kind: "CreditNote",
      spec: {
        serie_numero: "FC01-1",
        tipo_nota: "01",
        comprobante_afectado_serie_numero: "F001-1",
        sustento_descripcion: "",
        proveedor: { ruc: "", razon_social: "" },
        cliente: {
          tipo_documento_identidad: "6",
          numero_documento_identidad: "",
          nombre: "",
        },
        detalles: [{ descripcion: "", cantidad: 1, precio: 0 }],
      },
    },
    null,
    2
  ),
  DebitNote: JSON.stringify(
    {
      kind: "DebitNote",
      spec: {
        serie_numero: "FD01-1",
        tipo_nota: "01",
        comprobante_afectado_serie_numero: "F001-1",
        sustento_descripcion: "",
        proveedor: { ruc: "", razon_social: "" },
        cliente: {
          tipo_documento_identidad: "6",
          numero_documento_identidad: "",
          nombre: "",
        },
        detalles: [{ descripcion: "", cantidad: 1, precio: 0 }],
      },
    },
    null,
    2
  ),
  DespatchAdvice: JSON.stringify(
    {
      kind: "DespatchAdvice",
      spec: {
        serie_numero: "T001-1",
        remitente: { ruc: "", razon_social: "" },
        destinatario: {
          tipo_documento_identidad: "1",
          numero_documento_identidad: "",
          nombre: "",
        },
        envio: {
          tipo_traslado: "01",
          peso_total: 0,
          peso_total_unidad_medida: "KGM",
          tipo_modalidad_traslado: "02",
          fecha_traslado: "",
          partida: { ubigeo: "", direccion: "" },
          destino: { ubigeo: "", direccion: "" },
        },
        detalles: [
          {
            unidad_medida: "NIU",
            cantidad: 1,
            codigo: "",
            descripcion: "",
          },
        ],
      },
    },
    null,
    2
  ),
  Perception: JSON.stringify(
    {
      kind: "Perception",
      spec: {
        serie: "P001",
        numero: 1,
        proveedor: { ruc: "", razon_social: "" },
        cliente: {
          tipo_documento_identidad: "6",
          numero_documento_identidad: "",
          nombre: "",
        },
        tipo_regimen: "01",
        tipo_regimen_porcentaje: 2,
        importe_total_percibido: 0,
        importe_total_cobrado: 0,
      },
    },
    null,
    2
  ),
  Retention: JSON.stringify(
    {
      kind: "Retention",
      spec: {
        serie: "R001",
        numero: 1,
        proveedor: { ruc: "", razon_social: "" },
        cliente: {
          tipo_documento_identidad: "6",
          numero_documento_identidad: "",
          nombre: "",
        },
        tipo_regimen: "01",
        tipo_regimen_porcentaje: 3,
        importe_total_retenido: 0,
        importe_total_pagado: 0,
      },
    },
    null,
    2
  ),
  SummaryDocuments: JSON.stringify(
    {
      kind: "SummaryDocuments",
      spec: {
        numero: 1,
        proveedor: { ruc: "", razon_social: "" },
        comprobantes: [
          {
            tipo_operacion: "1",
            comprobante: {
              tipo_comprobante: "03",
              serie_numero: "B001-1",
              cliente: {
                tipo_documento_identidad: "1",
                numero_documento_identidad: "",
              },
              valor_venta: { importe_total: 0, gravado: 0 },
              impuestos: { igv: 0, igv_tasa: 18 },
            },
          },
        ],
      },
    },
    null,
    2
  ),
  VoidedDocuments: JSON.stringify(
    {
      kind: "VoidedDocuments",
      spec: {
        numero: 1,
        proveedor: { ruc: "", razon_social: "" },
        comprobantes: [
          { serie: "F001", numero: 1, descripcion_sustento: "" },
        ],
      },
    },
    null,
    2
  ),
};

interface AlertItem {
  key: number;
  variant: AlertVariant;
  title: string;
}

let alertCounter = 0;

export default function App() {
  const [editorContent, setEditorContent] = useState(TEMPLATES.Invoice);
  const [selectedKind, setSelectedKind] = useState<DocumentKind>("Invoice");
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [settings, setSettings] = useState<Settings>(DEFAULT_SETTINGS);
  const [beta, setBeta] = useState(true);
  const [alerts, setAlerts] = useState<AlertItem[]>([]);

  const pipeline = usePipeline();

  const addAlert = useCallback(
    (variant: AlertVariant, title: string) => {
      const key = alertCounter++;
      setAlerts((prev) => [...prev, { key, variant, title }]);
      setTimeout(() => {
        setAlerts((prev) => prev.filter((a) => a.key !== key));
      }, 5000);
    },
    []
  );

  const handleKindChange = useCallback(
    (kind: DocumentKind) => {
      setSelectedKind(kind);
      setEditorContent(TEMPLATES[kind]);
      pipeline.reset();
    },
    [pipeline]
  );

  const handleEditorChange = useCallback(
    (value: string) => {
      setEditorContent(value);
      pipeline.reset();
    },
    [pipeline]
  );

  const buildSendConfig = useCallback((): SendConfig => {
    return {
      username: settings.username || undefined,
      password: settings.password || undefined,
      url_invoice: settings.urlInvoice || undefined,
      url_perception_retention: settings.urlPerceptionRetention || undefined,
      url_despatch: settings.urlDespatch || undefined,
      beta,
    };
  }, [settings, beta]);

  const handleCreateXml = useCallback(async () => {
    pipeline.setLoading(true);
    try {
      const xml = await createXml(editorContent);
      pipeline.addOutput({
        step: "create",
        label: "XML",
        content: xml,
        language: "xml",
        isError: false,
      });
      addAlert(AlertVariant.success, "XML creado exitosamente");
    } catch (e) {
      pipeline.addOutput({
        step: "create",
        label: "XML",
        content: String(e),
        language: "text",
        isError: true,
      });
      addAlert(AlertVariant.danger, `Error: ${e}`);
    } finally {
      pipeline.setLoading(false);
    }
  }, [editorContent, pipeline, addAlert]);

  const handleSignXml = useCallback(async () => {
    const createOutput = pipeline.getOutput("create");
    if (!createOutput || createOutput.isError) return;

    pipeline.setLoading(true);
    try {
      const signed = await signXml(
        createOutput.content,
        settings.privateKeyPem || null,
        settings.certificatePem || null,
        beta
      );
      pipeline.addOutput({
        step: "sign",
        label: "XML Firmado",
        content: signed,
        language: "xml",
        isError: false,
      });
      addAlert(AlertVariant.success, "XML firmado exitosamente");
    } catch (e) {
      pipeline.addOutput({
        step: "sign",
        label: "XML Firmado",
        content: String(e),
        language: "text",
        isError: true,
      });
      addAlert(AlertVariant.danger, `Error al firmar: ${e}`);
    } finally {
      pipeline.setLoading(false);
    }
  }, [pipeline, settings, beta, addAlert]);

  const handleSendXml = useCallback(async () => {
    const signOutput = pipeline.getOutput("sign");
    if (!signOutput || signOutput.isError) return;

    pipeline.setLoading(true);
    try {
      const response = await sendXml(signOutput.content, buildSendConfig());
      const content = JSON.stringify(response, null, 2);
      const isError = response.type === "Error";
      pipeline.addOutput({
        step: "send",
        label: "Respuesta SUNAT",
        content,
        language: "json",
        isError,
      });
      if (isError) {
        addAlert(AlertVariant.warning, `SUNAT error: ${response.message}`);
      } else {
        addAlert(AlertVariant.success, "Documento enviado exitosamente");
      }
    } catch (e) {
      pipeline.addOutput({
        step: "send",
        label: "Respuesta SUNAT",
        content: String(e),
        language: "text",
        isError: true,
      });
      addAlert(AlertVariant.danger, `Error al enviar: ${e}`);
    } finally {
      pipeline.setLoading(false);
    }
  }, [pipeline, buildSendConfig, addAlert]);

  const handleApply = useCallback(async () => {
    pipeline.setLoading(true);
    try {
      // Step 1: Create
      const xml = await createXml(editorContent);
      pipeline.addOutput({
        step: "create",
        label: "XML",
        content: xml,
        language: "xml",
        isError: false,
      });

      // Step 2: Sign
      const signed = await signXml(
        xml,
        settings.privateKeyPem || null,
        settings.certificatePem || null,
        beta
      );
      pipeline.addOutput({
        step: "sign",
        label: "XML Firmado",
        content: signed,
        language: "xml",
        isError: false,
      });

      // Step 3: Send
      const response = await sendXml(signed, buildSendConfig());
      const content = JSON.stringify(response, null, 2);
      const isError = response.type === "Error";
      pipeline.addOutput({
        step: "send",
        label: "Respuesta SUNAT",
        content,
        language: "json",
        isError,
      });

      if (isError) {
        addAlert(AlertVariant.warning, `SUNAT error: ${response.message}`);
      } else {
        addAlert(AlertVariant.success, "Pipeline completado exitosamente");
      }
    } catch (e) {
      addAlert(AlertVariant.danger, `Error en pipeline: ${e}`);
    } finally {
      pipeline.setLoading(false);
    }
  }, [editorContent, pipeline, settings, beta, buildSendConfig, addAlert]);

  return (
    <Page>
      <PageSection padding={{ default: "noPadding" }}>
        <Toolbar
          selectedKind={selectedKind}
          onKindChange={handleKindChange}
          onCreateXml={handleCreateXml}
          onSignXml={handleSignXml}
          onSendXml={handleSendXml}
          onApply={handleApply}
          onSettingsClick={() => setSettingsOpen(true)}
          canCreate={editorContent.trim().length > 0}
          canSign={pipeline.canSign}
          canSend={pipeline.canSend}
          loading={pipeline.loading}
          beta={beta}
          onBetaChange={setBeta}
        />
      </PageSection>

      <PageSection
        isFilled
        padding={{ default: "noPadding" }}
        style={{ height: "calc(100vh - 80px)" }}
      >
        <Split hasGutter style={{ height: "100%" }}>
          <SplitItem isFilled style={{ minWidth: "40%" }}>
            <DocumentEditor
              value={editorContent}
              onChange={handleEditorChange}
              language="json"
            />
          </SplitItem>
          <SplitItem isFilled style={{ minWidth: "40%" }}>
            <OutputPanel
              outputs={pipeline.outputs}
              currentStep={pipeline.currentStep}
              onStepClick={(step: PipelineStep) =>
                pipeline.setCurrentStep(step)
              }
            />
          </SplitItem>
        </Split>
      </PageSection>

      <AlertGroup isToast isLiveRegion>
        {alerts.map((alert) => (
          <Alert key={alert.key} variant={alert.variant} title={alert.title} />
        ))}
      </AlertGroup>

      <SettingsModal
        isOpen={settingsOpen}
        onClose={() => setSettingsOpen(false)}
        settings={settings}
        onSave={setSettings}
      />
    </Page>
  );
}

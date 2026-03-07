import {
  CodeEditor,
  Language,
} from "@patternfly/react-code-editor";
import {
  Breadcrumb,
  BreadcrumbItem,
  EmptyState,
  EmptyStateBody,
} from "@patternfly/react-core";

import type { PipelineStep, StepOutput } from "../hooks/usePipeline";

interface OutputPanelProps {
  outputs: StepOutput[];
  currentStep: PipelineStep | null;
  onStepClick: (step: PipelineStep) => void;
}

const STEP_LABELS: Record<PipelineStep, string> = {
  create: "XML",
  sign: "XML Firmado",
  send: "Respuesta SUNAT",
};

const LANGUAGE_MAP: Record<string, Language> = {
  xml: Language.xml,
  json: Language.json,
  text: Language.plaintext,
};

export default function OutputPanel({
  outputs,
  currentStep,
  onStepClick,
}: OutputPanelProps) {
  const activeOutput = outputs.find((o) => o.step === currentStep);

  if (outputs.length === 0) {
    return (
      <EmptyState titleText="Sin resultados" headingLevel="h4">
        <EmptyStateBody>
          Escribe un documento JSON y haz clic en &quot;Crear XML&quot; para
          generar el UBL XML.
        </EmptyStateBody>
      </EmptyState>
    );
  }

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      <Breadcrumb style={{ padding: "8px 16px" }}>
        {outputs.map((output) => (
          <BreadcrumbItem
            key={output.step}
            isActive={output.step === currentStep}
            onClick={() => onStepClick(output.step)}
            component="button"
          >
            {STEP_LABELS[output.step]}
          </BreadcrumbItem>
        ))}
      </Breadcrumb>

      <div style={{ flex: 1 }}>
        {activeOutput && (
          <CodeEditor
            isFullHeight
            isDarkTheme
            isReadOnly
            isCopyEnabled
            isDownloadEnabled
            downloadFileName={`${activeOutput.step}-output.${activeOutput.language === "xml" ? "xml" : activeOutput.language === "json" ? "json" : "txt"}`}
            isLineNumbersVisible
            language={LANGUAGE_MAP[activeOutput.language] ?? Language.plaintext}
            code={activeOutput.content}
            options={{
              minimap: { enabled: false },
              fontSize: 14,
              wordWrap: "on",
              automaticLayout: true,
              scrollBeyondLastLine: false,
            }}
          />
        )}
      </div>
    </div>
  );
}

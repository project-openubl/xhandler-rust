import {
  CodeEditor,
  Language,
} from "@patternfly/react-code-editor";
import type { editor } from "monaco-editor";
import type { Monaco } from "@monaco-editor/react";

interface DocumentEditorProps {
  value: string;
  onChange: (value: string) => void;
  language: "json" | "yaml";
}

const LANGUAGE_MAP: Record<string, Language> = {
  json: Language.json,
  yaml: Language.yaml,
};

const DOCUMENT_KINDS = [
  "Invoice",
  "CreditNote",
  "DebitNote",
  "DespatchAdvice",
  "Perception",
  "Retention",
  "SummaryDocuments",
  "VoidedDocuments",
];

export default function DocumentEditor({
  value,
  onChange,
  language,
}: DocumentEditorProps) {
  const handleEditorDidMount = (
    _editor: editor.IStandaloneCodeEditor,
    monaco: Monaco
  ) => {
    if (language === "json") {
      monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
        validate: true,
        allowComments: false,
        schemaValidation: "error",
        schemas: [
          {
            uri: "https://openubl.io/schemas/document-input.json",
            fileMatch: ["*"],
            schema: {
              type: "object",
              required: ["kind", "spec"],
              properties: {
                kind: {
                  type: "string",
                  enum: DOCUMENT_KINDS,
                  description: "Tipo de documento",
                },
                spec: {
                  type: "object",
                  description:
                    "Especificacion del documento segun su tipo (kind)",
                },
              },
            },
          },
        ],
      });
    }
  };

  return (
    <CodeEditor
      isFullHeight
      isDarkTheme
      isLineNumbersVisible
      isUploadEnabled
      isCopyEnabled
      isDownloadEnabled
      downloadFileName="document.json"
      language={LANGUAGE_MAP[language]}
      code={value}
      onCodeChange={onChange}
      onEditorDidMount={handleEditorDidMount}
      options={{
        minimap: { enabled: false },
        fontSize: 14,
        wordWrap: "on",
        formatOnPaste: true,
        automaticLayout: true,
        scrollBeyondLastLine: false,
        tabSize: 2,
      }}
    />
  );
}

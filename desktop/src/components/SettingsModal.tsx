import {
  Modal,
  ModalBody,
  ModalHeader,
  ModalFooter,
  Button,
  Form,
  FormGroup,
  TextInput,
  ExpandableSection,
} from "@patternfly/react-core";
import { useState, useEffect } from "react";

export interface Settings {
  privateKeyPem: string;
  certificatePem: string;
  username: string;
  password: string;
  urlInvoice: string;
  urlPerceptionRetention: string;
  urlDespatch: string;
}

export const DEFAULT_SETTINGS: Settings = {
  privateKeyPem: "",
  certificatePem: "",
  username: "",
  password: "",
  urlInvoice: "",
  urlPerceptionRetention: "",
  urlDespatch: "",
};

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  settings: Settings;
  onSave: (settings: Settings) => void;
}

export default function SettingsModal({
  isOpen,
  onClose,
  settings,
  onSave,
}: SettingsModalProps) {
  const [form, setForm] = useState<Settings>(settings);
  const [urlsExpanded, setUrlsExpanded] = useState(false);

  useEffect(() => {
    setForm(settings);
  }, [settings]);

  const handleSave = () => {
    onSave(form);
    onClose();
  };

  const update = (field: keyof Settings, value: string) => {
    setForm((prev) => ({ ...prev, [field]: value }));
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} variant="medium">
      <ModalHeader title="Configuracion" />
      <ModalBody>
        <Form>
          <FormGroup label="Llave privada (PEM)" fieldId="private-key">
            <TextInput
              id="private-key"
              value={form.privateKeyPem}
              onChange={(_event, val) => update("privateKeyPem", val)}
              placeholder="Contenido PEM de la llave privada PKCS#1"
              type="text"
            />
          </FormGroup>
          <FormGroup label="Certificado (PEM)" fieldId="certificate">
            <TextInput
              id="certificate"
              value={form.certificatePem}
              onChange={(_event, val) => update("certificatePem", val)}
              placeholder="Contenido PEM del certificado X.509"
              type="text"
            />
          </FormGroup>
          <FormGroup label="Usuario SOL" fieldId="username">
            <TextInput
              id="username"
              value={form.username}
              onChange={(_event, val) => update("username", val)}
              placeholder="20123456789MODDATOS"
            />
          </FormGroup>
          <FormGroup label="Clave SOL" fieldId="password">
            <TextInput
              id="password"
              type="password"
              value={form.password}
              onChange={(_event, val) => update("password", val)}
            />
          </FormGroup>

          <ExpandableSection
            toggleText="URLs personalizadas"
            isExpanded={urlsExpanded}
            onToggle={(_event, expanded) => setUrlsExpanded(expanded)}
          >
            <FormGroup
              label="URL Facturas"
              fieldId="url-invoice"
            >
              <TextInput
                id="url-invoice"
                value={form.urlInvoice}
                onChange={(_event, val) => update("urlInvoice", val)}
                placeholder="Dejar vacio para usar URL por defecto"
              />
            </FormGroup>
            <FormGroup
              label="URL Percepciones/Retenciones"
              fieldId="url-pr"
            >
              <TextInput
                id="url-pr"
                value={form.urlPerceptionRetention}
                onChange={(_event, val) =>
                  update("urlPerceptionRetention", val)
                }
                placeholder="Dejar vacio para usar URL por defecto"
              />
            </FormGroup>
            <FormGroup
              label="URL Guias de Remision"
              fieldId="url-despatch"
            >
              <TextInput
                id="url-despatch"
                value={form.urlDespatch}
                onChange={(_event, val) => update("urlDespatch", val)}
                placeholder="Dejar vacio para usar URL por defecto"
              />
            </FormGroup>
          </ExpandableSection>
        </Form>
      </ModalBody>
      <ModalFooter>
        <Button variant="primary" onClick={handleSave}>
          Guardar
        </Button>
        <Button variant="link" onClick={onClose}>
          Cancelar
        </Button>
      </ModalFooter>
    </Modal>
  );
}

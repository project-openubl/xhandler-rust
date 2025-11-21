import {
  Button,
  ButtonVariant,
  Modal,
  ModalBody,
  ModalFooter,
  ModalHeader,
} from "@patternfly/react-core";
import type React from "react";

import { CredentialForm } from "./CredentialsForm";
import { useCredentialsFormData as useCredentialsFormHook } from "../hooks/useCredentialsFormData";

export interface ApplicationFormModalProps {
  credential: Credential | null;
  onClose: () => void;
}

export const ApplicationFormModal: React.FC<ApplicationFormModalProps> = ({
  credential,
  onClose,
}) => {
  const formProps = useCredentialsFormHook({ c, onClose });
  return (
    <Modal isOpen={true} variant="medium" onClose={onClose}>
      <ModalHeader
        title={credential ? "Actualizar Credencial" : "Nueva Credencial"}
      />
      <ModalBody id="modal-box-body-basic">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat. Duis aute irure dolor in reprehenderit in voluptate
        velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint
        occaecat cupidatat non proident, sunt in culpa qui officia deserunt
        mollit anim id est laborum.
      </ModalBody>
      <ModalFooter>
        <Button
          key="submit"
          id="submit"
          aria-label="submit"
          variant={ButtonVariant.primary}
          isDisabled={formProps.isSubmitDisabled}
          onClick={formProps.onSubmit}
        >
          {!credential ? "Crear" : "Actualizar"}
        </Button>
        <Button
          key="cancel"
          id="cancel"
          aria-label="cancel"
          variant={ButtonVariant.link}
          isDisabled={formProps.isCancelDisabled}
          onClick={onClose}
        >
          Cancelar
        </Button>
      </ModalFooter>
    </Modal>
  );
};

import type React from "react";

import { yupResolver } from "@hookform/resolvers/yup";
import { useForm } from "react-hook-form";
import { array, object, string } from "yup";

import {
  ActionGroup,
  Button,
  ButtonVariant,
  Form,
} from "@patternfly/react-core";

import { EmailRecepientInput } from "@app/components/EmailRecepientInput";
import {
  HookFormPFGroupController,
  HookFormPFTextArea,
  HookFormPFTextInput,
} from "@app/components/HookFormPFFields";
import { duplicateNameCheck } from "@app/utils/utils";
import type { New } from "@client/helpers";
import type { Credentials } from "@client/models";

import { useCredentialsFormData } from "../hooks/useCredentialsFormData";

export interface FormValues {
  name: string;
  description?: string;
  ruc: string[];
}

export interface CredentialFormProps {
  credential: Credentials | null;
  onClose: () => void;
}

export const CredentialForm: React.FC<CredentialFormProps> = ({
  credential,
  onClose,
}) => {
  const { credentials, createCredential, updateCredential } =
    useCredentialsFormData({
      onActionSuccess: onClose,
    });

  const validationSchema = object().shape({
    name: string()
      .trim()
      .required()
      .min(3)
      .max(120)
      .test(
        "Duplicate name",
        "A credential with this name already exists. Use a different name.",
        (value) => {
          return duplicateNameCheck(
            credentials || [],
            credential || null,
            value || ""
          );
        }
      ),
    description: string().trim().max(250),
    ruc: array(string().trim().required().length(11)).required().min(1),
  });

  const {
    handleSubmit,
    formState: { isSubmitting, isValidating, isValid, isDirty },
    control,
  } = useForm<FormValues>({
    defaultValues: {
      name: credential?.name || "",
      description: credential?.description || "",
      ruc: credential?.supplier_ids_applied_to || [],
    },
    resolver: yupResolver(validationSchema),
    mode: "all",
  });

  const onSubmit = (formValues: FormValues) => {
    const payload: New<Credentials> = {
      name: formValues.name.trim(),
      description: formValues.description?.trim(),
      supplier_ids_applied_to: formValues.ruc,
    };

    if (credential) {
      updateCredential({ id: credential.id, ...payload });
    } else {
      createCredential(payload);
    }
  };

  return (
    <Form onSubmit={handleSubmit(onSubmit)}>
      <HookFormPFTextInput
        control={control}
        name="name"
        label="Nombre"
        fieldId="name"
        isRequired
      />
      <HookFormPFTextArea
        control={control}
        name="description"
        label="Descripción"
        fieldId="description"
      />
      <HookFormPFGroupController
        control={control}
        name="ruc"
        label="RUC"
        fieldId="ruc"
        renderInput={({ field: { value, name, onChange } }) => (
          <EmailRecepientInput
            aria-label={name}
            value={value}
            onChange={(selection) => onChange(selection)}
            onClear={() => onChange("")}
          />
        )}
      />
      <ActionGroup>
        <Button
          type="submit"
          id="submit"
          aria-label="submit"
          variant={ButtonVariant.primary}
          isDisabled={!isValid || isSubmitting || isValidating || !isDirty}
        >
          {!credential ? "Crear" : "Guardar"}
        </Button>
        <Button
          type="button"
          id="cancel"
          aria-label="cancel"
          variant={ButtonVariant.link}
          isDisabled={isSubmitting || isValidating}
          onClick={onClose}
        >
          "Cancelar"
        </Button>
      </ActionGroup>
    </Form>
  );
};

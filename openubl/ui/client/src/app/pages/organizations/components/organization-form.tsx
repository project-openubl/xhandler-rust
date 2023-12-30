import React, { useContext } from "react";
import { AxiosError, AxiosResponse } from "axios";
import { object, string } from "yup";
import { useForm } from "react-hook-form";
import { yupResolver } from "@hookform/resolvers/yup";

import {
  ActionGroup,
  Button,
  ButtonVariant,
  Form,
} from "@patternfly/react-core";

import { New, Organization } from "@app/api/models";
import { duplicateFieldCheck } from "@app/utils/utils";
import {
  useCreateOrganizationMutation,
  useFetchOrganizations,
  useUpdateOrganizationMutation,
} from "@app/queries/organizations";

import {
  HookFormPFTextArea,
  HookFormPFTextInput,
} from "@app/shared/components/HookFormPFFields";
import { NotificationsContext } from "@app/shared/components/NotificationsContext";

export interface FormValues {
  name: string;
  description?: string;
}

export interface IOrganizationFormProps {
  organization?: Organization;
  onClose: () => void;
}

export const OrganizationForm: React.FC<IOrganizationFormProps> = ({
  organization,
  onClose,
}) => {
  const { pushNotification } = useContext(NotificationsContext);

  const { result: organizations } = useFetchOrganizations();

  const validationSchema = object().shape({
    name: string()
      .trim()
      .required()
      .min(3)
      .max(120)
      .matches(/[a-z0-9]([-a-z0-9]*[a-z0-9])?/)
      .test(
        "Duplicate name",
        "A organization with this name address already exists. Use a different name.",
        (value) =>
          duplicateFieldCheck(
            "name",
            organizations,
            organization || null,
            value || ""
          )
      ),
    description: string().trim().max(250),
  });

  const {
    handleSubmit,
    formState: { isSubmitting, isValidating, isValid, isDirty },
    getValues,
    control,
  } = useForm<FormValues>({
    defaultValues: {
      name: organization?.name || "",
      description: organization?.description || "",
    },
    resolver: yupResolver(validationSchema),
    mode: "onChange",
  });

  const onCreateOrganizationSuccess = (_: AxiosResponse<Organization>) =>
    pushNotification({
      title: "Organization created",
      variant: "success",
    });

  const onCreateOrganizationError = (error: AxiosError) => {
    pushNotification({
      title: "Error while creating organization",
      variant: "danger",
    });
  };

  const { mutate: createOrganization } = useCreateOrganizationMutation(
    onCreateOrganizationSuccess,
    onCreateOrganizationError
  );

  const onUpdateOrganizationSuccess = (_: AxiosResponse<Organization>) =>
    pushNotification({
      title: "Organization saved",
      variant: "success",
    });

  const onUpdateOrganizationError = (error: AxiosError) => {
    pushNotification({
      title: "Error while saving data",
      variant: "danger",
    });
  };
  const { mutate: updateOrganization } = useUpdateOrganizationMutation(
    onUpdateOrganizationSuccess,
    onUpdateOrganizationError
  );

  const onSubmit = (formValues: FormValues) => {
    const payload: New<Organization> = {
      name: formValues.name.trim(),
      description: formValues.description?.trim(),
    };

    if (organization) {
      updateOrganization({ id: organization.id, ...payload });
    } else {
      createOrganization(payload);
    }
    onClose();
  };

  return (
    <Form onSubmit={handleSubmit(onSubmit)}>
      <HookFormPFTextInput
        control={control}
        name="name"
        label="Name"
        fieldId="name"
        isRequired
      />
      <HookFormPFTextArea
        control={control}
        name="description"
        label="Description"
        fieldId="description"
        resizeOrientation="vertical"
      />

      <ActionGroup>
        <Button
          type="submit"
          aria-label="submit"
          id="organization-form-submit"
          variant={ButtonVariant.primary}
          isDisabled={!isValid || isSubmitting || isValidating || !isDirty}
        >
          {!organization ? "Create" : "Save"}
        </Button>
        <Button
          type="button"
          id="cancel"
          aria-label="cancel"
          variant={ButtonVariant.link}
          isDisabled={isSubmitting || isValidating}
          onClick={onClose}
        >
          Cancel
        </Button>
      </ActionGroup>
    </Form>
  );
};

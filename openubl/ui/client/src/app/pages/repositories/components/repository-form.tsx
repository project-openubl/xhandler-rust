import React, { useContext } from "react";
import { AxiosError, AxiosResponse } from "axios";
import { useForm } from "react-hook-form";
import { number, object, string } from "yup";
import { yupResolver } from "@hookform/resolvers/yup";

import {
  ActionGroup,
  Button,
  ButtonVariant,
  Form,
  FormSelectOption,
} from "@patternfly/react-core";

import { New, Organization, Repository } from "@app/api/models";
import { useFetchOrganizations } from "@app/queries/organizations";

import {
  HookFormPFSelect,
  HookFormPFTextArea,
  HookFormPFTextInput,
} from "@app/shared/components/HookFormPFFields";
import { NotificationsContext } from "@app/shared/components/NotificationsContext";
import {
  useCreateRepositoryMutation,
  useFetchRepositories,
  useUpdateRepositoryMutation,
} from "@app/queries/repositories";

export interface FormValues {
  organization: number;
  name: string;
  description?: string;
}

export interface IRepositoryFormProps {
  repository?: Repository;
  onClose: () => void;
}

export const RepositoryForm: React.FC<IRepositoryFormProps> = ({
  repository,
  onClose,
}) => {
  const { pushNotification } = useContext(NotificationsContext);

  const { result: organizations } = useFetchOrganizations();
  const { result: repositories } = useFetchRepositories();

  const validationSchema = object().shape({
    organization: number().required(),
    name: string()
      .trim()
      .required()
      .min(3)
      .max(120)
      .matches(/[a-z0-9]([-a-z0-9]*[a-z0-9])?/),
    description: string().trim().max(250),
  });

  const {
    
    handleSubmit,
    formState: { isSubmitting, isValidating, isValid, isDirty },
    getValues,
    control,
  } = useForm<FormValues>({
    defaultValues: {
      name: repository?.name || "",
      description: repository?.description || "",
      organization: repository?.organization.id || undefined,
    },
    resolver: yupResolver(validationSchema),
    mode: "onChange",
  });

  const onCreateRepositorySuccess = (_: AxiosResponse<Repository>) =>
    pushNotification({
      title: "Repository created",
      variant: "success",
    });

  const onCreateRepositoryError = (error: AxiosError) => {
    pushNotification({
      title: "Error while creating repository",
      variant: "danger",
    });
  };

  const { mutate: createRepository } = useCreateRepositoryMutation(
    onCreateRepositorySuccess,
    onCreateRepositoryError
  );

  const onUpdateRepositorySuccess = (_: AxiosResponse<Repository>) =>
    pushNotification({
      title: "Product saved",
      variant: "success",
    });

  const onUpdateRepositoryError = (error: AxiosError) => {
    pushNotification({
      title: "Error while saving data",
      variant: "danger",
    });
  };
  const { mutate: updateRepository } = useUpdateRepositoryMutation(
    onUpdateRepositorySuccess,
    onUpdateRepositoryError
  );

  const onSubmit = (formValues: FormValues) => {
    const payload: New<Repository> = {
      organization: { id: formValues.organization } as Organization,
      name: formValues.name.trim(),
      description: formValues.description?.trim(),
    };

    if (repository) {
      updateRepository({ id: repository.id, ...payload });
    } else {
      createRepository(payload);
    }
    onClose();
  };

  return (
    <Form onSubmit={handleSubmit(onSubmit)}>
      <HookFormPFSelect
        control={control}
        name="organization"
        label="Organization"
        fieldId="organization"
        isRequired
        isDisabled={!!repository}
      >
        <FormSelectOption value={""} label={"Select one"} isDisabled />
        {organizations.map((organization) => (
          <FormSelectOption
            key={organization.id}
            value={organization.id}
            label={organization.name}
          />
        ))}
      </HookFormPFSelect>
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
          id="repository-form-submit"
          variant={ButtonVariant.primary}
          isDisabled={!isValid || isSubmitting || isValidating || !isDirty}
        >
          {!repository ? "Create" : "Save"}
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

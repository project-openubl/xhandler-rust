import React, { useContext } from "react";
import { AxiosError } from "axios";
import { object, string } from "yup";
import { useForm } from "react-hook-form";
import { yupResolver } from "@hookform/resolvers/yup";
import { useTranslation } from "react-i18next";

import {
  ActionGroup,
  Button,
  ButtonVariant,
  Form,
} from "@patternfly/react-core";

import { New, Project } from "@app/api/models";
import { duplicateFieldCheck } from "@app/utils/utils";
import {
  useCreateProjectMutation,
  useFetchProjectById,
  useFetchProjects,
  useUpdateProjectMutation,
} from "@app/queries/projects";

import {
  HookFormPFTextArea,
  HookFormPFTextInput,
} from "@app/components/HookFormPFFields";
import { NotificationsContext } from "@app/components/NotificationsContext";

export interface FormValues {
  name: string;
  description?: string;
}

export interface IProjectFormProps {
  project?: Project;
  onClose: () => void;
}

export const ProjectForm: React.FC<IProjectFormProps> = ({
  project,
  onClose,
}) => {
  const { t } = useTranslation();
  const { pushNotification } = useContext(NotificationsContext);

  const { projects } = useFetchProjects();

  const validationSchema = object().shape({
    name: string()
      .trim()
      .required()
      .min(3)
      .max(120)
      .matches(/[a-z0-9]([-a-z0-9]*[a-z0-9])?/)
      .test(
        "Duplicate name",
        "A project with this name already exists. Use a different name.",
        (value) =>
          duplicateFieldCheck("name", projects, project || null, value || "")
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
      name: project?.name || "",
      description: project?.description || "",
    },
    resolver: yupResolver(validationSchema),
    mode: "onChange",
  });

  const onCreateProjectSuccess = (_: Project) =>
    pushNotification({
      title: t("toastr.success.created", {
        type: t("terms.project"),
      }),
      variant: "success",
    });

  const onCreateProjectError = (error: AxiosError) => {
    pushNotification({
      title: t("toastr.fail.create", {
        type: t("terms.project").toLowerCase(),
      }),
      variant: "danger",
    });
  };

  const { mutate: createProject } = useCreateProjectMutation(
    onCreateProjectSuccess,
    onCreateProjectError
  );

  const onUpdateProjectSuccess = (_: Project) =>
    pushNotification({
      title: t("toastr.success.saved", {
        type: t("terms.project"),
      }),
      variant: "success",
    });

  const onUpdateProjectError = (error: AxiosError) => {
    pushNotification({
      title: t("toastr.fail.save", {
        type: t("terms.project").toLowerCase(),
      }),
      variant: "danger",
    });
  };
  const { mutate: updateProject } = useUpdateProjectMutation(
    onUpdateProjectSuccess,
    onUpdateProjectError
  );

  const onSubmit = (formValues: FormValues) => {
    const payload: New<Project> = {
      name: formValues.name.trim(),
      description: formValues.description?.trim(),
    };

    if (project) {
      updateProject({ id: project.id, ...payload });
    } else {
      createProject(payload);
    }
    onClose();
  };

  return (
    <Form onSubmit={handleSubmit(onSubmit)}>
      <HookFormPFTextInput
        control={control}
        name="name"
        label={t("terms.name")}
        fieldId="name"
        isRequired
      />
      <HookFormPFTextArea
        control={control}
        name="description"
        label={t("terms.description")}
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
          {!project ? t("terms.create") : t("terms.save")}
        </Button>
        <Button
          type="button"
          id="cancel"
          aria-label="cancel"
          variant={ButtonVariant.link}
          isDisabled={isSubmitting || isValidating}
          onClick={onClose}
        >
          {t("terms.cancel")}
        </Button>
      </ActionGroup>
    </Form>
  );
};

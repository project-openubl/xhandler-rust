import * as React from "react";
import { AxiosError, AxiosResponse } from "axios";

import {
  ActionGroup,
  Button,
  ButtonVariant,
  FileUpload,
  Form,
  FormGroup,
} from "@patternfly/react-core";

import { useForm, Controller } from "react-hook-form";
import { mixed, object, string } from "yup";
import { yupResolver } from "@hookform/resolvers/yup";

import { NotificationsContext } from "@app/shared/components/NotificationsContext";
import { Repository, Tag } from "@app/api/models";
import { HookFormPFTextInput } from "@app/shared/components/HookFormPFFields";
import { useCreateTagMutation } from "@app/queries/tags";

export interface FormValues {
  tag: string;
  file?: File;
}

export interface ITagFormProps {
  repository: Repository;
  onClose: () => void;
}

export const TagForm: React.FC<ITagFormProps> = ({ repository, onClose }) => {
  const { pushNotification } = React.useContext(NotificationsContext);

  const validationSchema = object().shape({
    tag: string().required(),
    file: mixed<File>().when("mode", {
      is: () => true,
      then: (schema) => schema.required(),
    }),
  });

  const {
    handleSubmit,
    formState: { isSubmitting, isValidating, isValid, isDirty },
    getValues,
    control,
  } = useForm<FormValues>({
    defaultValues: {
      tag: "",
    },
    resolver: yupResolver(validationSchema),
    mode: "onChange",
  });

  const onCreateTagSuccess = (_: AxiosResponse<Tag>) =>
    pushNotification({
      title: "Tag created",
      variant: "success",
    });

  const onCreateTagError = (error: AxiosError) => {
    pushNotification({
      title: "Error while creating tag",
      variant: "danger",
    });
  };

  const { mutate: createTag } = useCreateTagMutation(
    onCreateTagSuccess,
    onCreateTagError
  );

  const onSubmit = (formValues: FormValues) => {
    const formData = new FormData();
    formData.append("tag", formValues.tag);
    formValues.file && formData.append("file", formValues.file);

    createTag({ repositoryId: repository.id, formData });
    onClose();
  };

  return (
    <Form onSubmit={handleSubmit(onSubmit)}>
      <HookFormPFTextInput
        control={control}
        name="tag"
        label="Tag"
        fieldId="tag"
        isRequired
      />
      <Controller
        control={control}
        name="file"
        render={({
          field: { onChange, value, name, ref },
          fieldState: { invalid, isTouched, isDirty, error },
          formState,
        }) => (
          <FormGroup label="File" fieldId="file" isRequired>
            <FileUpload
              id="tag-file"
              name={name}
              value={value}
              filename={value?.name}
              filenamePlaceholder="Drag and drop a file or upload one"
              onFileInputChange={(_, file) => onChange(file)}
              onClearClick={() => onChange(undefined)}
              browseButtonText="Select file"
            />
          </FormGroup>
        )}
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

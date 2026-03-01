import React from "react";

import type { AxiosError } from "axios";

import { NotificationsContext } from "@app/components/NotificationsContext";
import type { New } from "@client/helpers";
import type { Credentials } from "@client/models";
import {
  useCreateCredentialsMutation,
  useFetchCredentials,
  useUpdateCredentialsMutation,
} from "@queries/credentials";

export const useCredentialsFormData = ({
  onActionSuccess = () => {},
  onActionFail = () => {},
}: {
  onActionSuccess?: () => void;
  onActionFail?: () => void;
}) => {
  const { pushNotification } = React.useContext(NotificationsContext);

  // Fetch data
  const { credentials } = useFetchCredentials();

  // Mutation notification handlers
  const onCreateCredentialSuccess = (data: Credentials) => {
    pushNotification({
      title: `Credential ${data.name} created`,
      variant: "success",
    });
    onActionSuccess();
  };

  const onCreateCredentialError = (
    _error: AxiosError,
    _payload: New<Credentials>,
  ) => {
    pushNotification({
      title: "Failed to create Credential",
      variant: "danger",
    });
    onActionFail();
  };

  const onUpdateCredentialSuccess = (payload: Credentials) => {
    pushNotification({
      title: `Credential ${payload.name} updated`,
      variant: "success",
    });
    onActionSuccess();
  };

  const onUpdateCredentialError = (
    _error: AxiosError,
    _payload: New<Credentials>,
  ) => {
    pushNotification({
      title: "Failed to update Credential",
      variant: "danger",
    });
    onActionFail();
  };

  // Mutations
  const { mutate: createCredential } = useCreateCredentialsMutation(
    onCreateCredentialSuccess,
    onCreateCredentialError,
  );

  const { mutate: updateCredential } = useUpdateCredentialsMutation(
    onUpdateCredentialSuccess,
    onUpdateCredentialError,
  );

  // Send back source data and action that are needed by the Form
  return {
    credentials,
    createCredential,
    updateCredential,
  };
};

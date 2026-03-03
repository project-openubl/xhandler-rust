import * as React from "react";

import type { AlertProps } from "@patternfly/react-core";

export interface INotification {
  title: string;
  variant: AlertProps["variant"];
  message?: React.ReactNode;
  hideCloseButton?: boolean;
  timeout?: number | boolean;
}

export interface INotificationsProvider {
  children: React.ReactNode;
}

interface INotificationsContext {
  pushNotification: (notification: INotification) => void;
  dismissNotification: (key: string) => void;
  notifications: INotification[];
}

const appContextDefaultValue = {} as INotificationsContext;

export const NotificationsContext = React.createContext<INotificationsContext>(
  appContextDefaultValue,
);

import * as React from "react";

import {
  type INotification,
  type INotificationsProvider,
  NotificationsContext,
} from "./NotificationsContext.tsx";

const notificationDefault: Pick<INotification, "hideCloseButton"> = {
  hideCloseButton: false,
};

export const NotificationsProvider: React.FunctionComponent<
  INotificationsProvider
> = ({ children }: INotificationsProvider) => {
  const [notifications, setNotifications] = React.useState<INotification[]>([]);

  const pushNotification = (
    notification: INotification,
    clearNotificationDelay?: number,
  ) => {
    setNotifications([
      ...notifications,
      { ...notificationDefault, ...notification },
    ]);
    setTimeout(() => setNotifications([]), clearNotificationDelay ?? 10000);
  };

  const dismissNotification = (title: string) => {
    const remainingNotifications = notifications.filter(
      (n) => n.title !== title,
    );
    setNotifications(remainingNotifications);
  };

  return (
    <NotificationsContext.Provider
      value={{
        pushNotification,
        dismissNotification,
        notifications,
      }}
    >
      {children}
    </NotificationsContext.Provider>
  );
};

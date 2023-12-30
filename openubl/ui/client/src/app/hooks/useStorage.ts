import * as React from "react";

type StorageType = "localStorage" | "sessionStorage";

const getValueFromStorage = <T>(
  storageType: StorageType,
  key: string,
  defaultValue: T
): T => {
  if (typeof window === "undefined") return defaultValue;
  try {
    const itemJSON = window[storageType].getItem(key);
    return itemJSON ? (JSON.parse(itemJSON) as T) : defaultValue;
  } catch (error) {
    console.error(error);
    return defaultValue;
  }
};

const setValueInStorage = <T>(
  storageType: StorageType,
  key: string,
  newValue: T | undefined
) => {
  if (typeof window === "undefined") return;
  try {
    if (newValue !== undefined) {
      const newValueJSON = JSON.stringify(newValue);
      window[storageType].setItem(key, newValueJSON);
      if (storageType === "localStorage") {
        // setItem only causes the StorageEvent to be dispatched in other windows. We dispatch it here
        // manually so that all instances of useLocalStorage on this window also react to this change.
        window.dispatchEvent(
          new StorageEvent("storage", { key, newValue: newValueJSON })
        );
      }
    } else {
      window[storageType].removeItem(key);
      if (storageType === "localStorage") {
        window.dispatchEvent(
          new StorageEvent("storage", { key, newValue: null })
        );
      }
    }
  } catch (error) {
    console.error(error);
  }
};

const useStorage = <T>(
  storageType: StorageType,
  key: string,
  defaultValue: T
): [T, React.Dispatch<React.SetStateAction<T>>] => {
  const [cachedValue, setCachedValue] = React.useState<T>(
    getValueFromStorage(storageType, key, defaultValue)
  );

  const usingStorageEvents =
    storageType === "localStorage" && typeof window !== "undefined";

  const setValue: React.Dispatch<React.SetStateAction<T>> = React.useCallback(
    (newValueOrFn: T | ((prevState: T) => T)) => {
      const newValue =
        newValueOrFn instanceof Function
          ? newValueOrFn(getValueFromStorage(storageType, key, defaultValue))
          : newValueOrFn;
      setValueInStorage(storageType, key, newValue);
      if (!usingStorageEvents) {
        // The cache won't update automatically if there is no StorageEvent dispatched.
        setCachedValue(newValue);
      }
    },
    [storageType, key, defaultValue, usingStorageEvents]
  );

  React.useEffect(() => {
    if (!usingStorageEvents) return;
    const onStorageUpdated = (event: StorageEvent) => {
      if (event.key === key) {
        setCachedValue(
          event.newValue ? JSON.parse(event.newValue) : defaultValue
        );
      }
    };
    window.addEventListener("storage", onStorageUpdated);
    return () => {
      window.removeEventListener("storage", onStorageUpdated);
    };
  }, [key, defaultValue, usingStorageEvents]);

  return [cachedValue, setValue];
};

export const useLocalStorage = <T>(
  key: string,
  defaultValue: T
): [T, React.Dispatch<React.SetStateAction<T>>] =>
  useStorage("localStorage", key, defaultValue);

export const useSessionStorage = <T>(
  key: string,
  defaultValue: T
): [T, React.Dispatch<React.SetStateAction<T>>] =>
  useStorage("sessionStorage", key, defaultValue);

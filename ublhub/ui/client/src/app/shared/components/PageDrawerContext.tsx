import * as React from "react";
import {
  Drawer,
  DrawerActions,
  DrawerCloseButton,
  DrawerContent,
  DrawerContentBody,
  DrawerHead,
  DrawerPanelContent,
  DrawerPanelContentProps,
} from "@patternfly/react-core";
import pageStyles from "@patternfly/react-styles/css/components/Page/page";

const usePageDrawerState = () => {
  const [isDrawerExpanded, setIsDrawerExpanded] = React.useState(false);
  const [drawerChildren, setDrawerChildren] =
    React.useState<React.ReactNode>(null);
  const [drawerPanelContentProps, setDrawerPanelContentProps] = React.useState<
    Partial<DrawerPanelContentProps>
  >({});
  const [drawerPageKey, setDrawerPageKey] = React.useState<string>("");
  const drawerFocusRef = React.useRef(document.createElement("span"));
  return {
    isDrawerExpanded,
    setIsDrawerExpanded,
    drawerChildren,
    setDrawerChildren,
    drawerPanelContentProps,
    setDrawerPanelContentProps,
    drawerPageKey,
    setDrawerPageKey,
    drawerFocusRef: drawerFocusRef as typeof drawerFocusRef | null,
  };
};

type PageDrawerState = ReturnType<typeof usePageDrawerState>;

const PageDrawerContext = React.createContext<PageDrawerState>({
  isDrawerExpanded: false,
  setIsDrawerExpanded: () => {},
  drawerChildren: null,
  setDrawerChildren: () => {},
  drawerPanelContentProps: {},
  setDrawerPanelContentProps: () => {},
  drawerPageKey: "",
  setDrawerPageKey: () => {},
  drawerFocusRef: null,
});

// PageContentWithDrawerProvider should only be rendered as the direct child of a PatternFly Page component.
interface IPageContentWithDrawerProviderProps {
  children: React.ReactNode; // The entire content of the page. See usage in client/src/app/layout/DefaultLayout.
}
export const PageContentWithDrawerProvider: React.FC<
  IPageContentWithDrawerProviderProps
> = ({ children }) => {
  const pageDrawerState = usePageDrawerState();
  const {
    isDrawerExpanded,
    drawerFocusRef,
    drawerChildren,
    drawerPanelContentProps,
    drawerPageKey,
  } = pageDrawerState;
  return (
    <PageDrawerContext.Provider value={pageDrawerState}>
      <div className={pageStyles.pageDrawer}>
        <Drawer
          isExpanded={isDrawerExpanded}
          onExpand={() => drawerFocusRef?.current?.focus()}
          position="right"
        >
          <DrawerContent
            panelContent={
              <DrawerPanelContent
                isResizable
                id="page-drawer-content"
                defaultSize="500px"
                minSize="150px"
                key={drawerPageKey}
                {...drawerPanelContentProps}
              >
                {drawerChildren}
              </DrawerPanelContent>
            }
          >
            <DrawerContentBody>{children}</DrawerContentBody>
          </DrawerContent>
        </Drawer>
      </div>
    </PageDrawerContext.Provider>
  );
};

let numPageDrawerContentInstances = 0;

// PageDrawerContent can be rendered anywhere, but must have only one instance rendered at a time.
export interface IPageDrawerContentProps {
  isExpanded: boolean;
  onCloseClick: () => void; // Should be used to update local state such that `isExpanded` becomes false.
  children: React.ReactNode; // The content to show in the drawer when `isExpanded` is true.
  drawerPanelContentProps?: Partial<DrawerPanelContentProps>; // Additional props for the DrawerPanelContent component.
  focusKey?: string | number; // A unique key representing the object being described in the drawer. When this changes, the drawer will regain focus.
  pageKey: string; // A unique key representing the page where the drawer is used. Causes the drawer to remount when changing pages.
}

export const PageDrawerContent: React.FC<IPageDrawerContentProps> = ({
  isExpanded: localIsExpandedProp,
  onCloseClick,
  children,
  drawerPanelContentProps: localDrawerPanelContentProps,
  focusKey,
  pageKey: localPageKeyProp,
}) => {
  const {
    setIsDrawerExpanded,
    drawerFocusRef,
    setDrawerChildren,
    setDrawerPanelContentProps,
    setDrawerPageKey,
  } = React.useContext(PageDrawerContext);

  // Warn if we are trying to render more than one PageDrawerContent (they'll fight over the same state).
  React.useEffect(() => {
    numPageDrawerContentInstances++;
    return () => {
      numPageDrawerContentInstances--;
    };
  }, []);
  if (numPageDrawerContentInstances > 1) {
    console.warn(
      `${numPageDrawerContentInstances} instances of PageDrawerContent are currently rendered! Only one instance of this component should be rendered at a time.`
    );
  }

  // Lift the value of isExpanded out to the context, but derive it from local state such as a selected table row.
  // This is the ONLY place where `setIsDrawerExpanded` should be called.
  // To expand/collapse the drawer, use the `isExpanded` prop when rendering PageDrawerContent.
  React.useEffect(() => {
    setIsDrawerExpanded(localIsExpandedProp);
    return () => {
      setIsDrawerExpanded(false);
      setDrawerChildren(null);
    };
  }, [localIsExpandedProp]);

  // Same with pageKey and drawerPanelContentProps, keep them in sync with the local prop on PageDrawerContent.
  React.useEffect(() => {
    setDrawerPageKey(localPageKeyProp);
    return () => {
      setDrawerPageKey("");
    };
  }, [localPageKeyProp]);

  React.useEffect(() => {
    setDrawerPanelContentProps(localDrawerPanelContentProps || {});
  }, [localDrawerPanelContentProps]);

  // If the drawer is already expanded describing app A, then the user clicks app B, we want to send focus back to the drawer.
  React.useEffect(() => {
    drawerFocusRef?.current?.focus();
  }, [focusKey]);

  React.useEffect(() => {
    setDrawerChildren(
      <DrawerHead>
        <span tabIndex={0} ref={drawerFocusRef}>
          {children}
        </span>
        <DrawerActions>
          <DrawerCloseButton
            // We call onCloseClick here instead of setIsDrawerExpanded
            // because we want the isExpanded prop of PageDrawerContent to be the source of truth.
            onClick={onCloseClick}
          />
        </DrawerActions>
      </DrawerHead>
    );
  }, [children]);

  return null;
};

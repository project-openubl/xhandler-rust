import React from "react";

import {
  Breadcrumb,
  BreadcrumbItem,
  Label,
  PageSection,
  PageSectionVariants,
  Spinner,
  Tab,
  TabContent,
  TabTitleText,
  Tabs,
  Text,
  TextContent,
} from "@patternfly/react-core";
import { usePackageById } from "@app/queries/packages";
import { useParams } from "react-router-dom";
import { PackagesTable } from "./components/packages-table";

export const ViewPackage: React.FC = () => {
  const { packageId } = useParams();

  const { result, isFetching } = usePackageById(packageId || "");

  const contentRef1 = React.createRef<HTMLElement>();
  const contentRef2 = React.createRef<HTMLElement>();
  const contentRef3 = React.createRef<HTMLElement>();

  return (
    <>
      <PageSection type="breadcrumb">
        <Breadcrumb>
          <BreadcrumbItem to="#">Packages</BreadcrumbItem>
          <BreadcrumbItem to="#" isActive>
            log4j
          </BreadcrumbItem>
        </Breadcrumb>
      </PageSection>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">log4j <Label color="blue">Maven</Label></Text>
          {/* <Text component="p">Package description</Text> */}
        </TextContent>
      </PageSection>
      <PageSection>
        <PackagesTable />
      </PageSection>
    </>
  );
};

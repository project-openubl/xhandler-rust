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
import { CodeEditor, Language } from "@patternfly/react-code-editor";
import { VulnerabilitiesTable } from "./components/vulnerabilities-table";

export const ViewSbom: React.FC = () => {
  const contentRef0 = React.createRef<HTMLElement>();
  const contentRef1 = React.createRef<HTMLElement>();
  const contentRef2 = React.createRef<HTMLElement>();
  const contentRef3 = React.createRef<HTMLElement>();

  return (
    <>
      <PageSection type="breadcrumb">
        <Breadcrumb>
          <BreadcrumbItem to="#">Products</BreadcrumbItem>
          <BreadcrumbItem to="#">JBoss EAP</BreadcrumbItem>
          <BreadcrumbItem isActive>Jboss EAP 7.4</BreadcrumbItem>
        </Breadcrumb>
      </PageSection>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">
            Jboss EAP 7.4 <Label color="blue">version: 7.4</Label>
          </Text>
          <Text component="p">SBOM created on 01/01/2099</Text>
        </TextContent>
      </PageSection>
      <PageSection type="tabs">
        <Tabs defaultActiveKey={0} inset={{ default: "insetLg" }}>
          <Tab
            eventKey={0}
            title={<TabTitleText>Overview</TabTitleText>}
            tabContentRef={contentRef0}
          />
          <Tab
            eventKey={1}
            title={<TabTitleText>Packages</TabTitleText>}
            tabContentRef={contentRef1}
          />
          <Tab
            eventKey={2}
            title={<TabTitleText>Vulnerabilities</TabTitleText>}
            tabContentRef={contentRef2}
          />
          <Tab
            eventKey={3}
            title={<TabTitleText>Source</TabTitleText>}
            tabContentRef={contentRef3}
          />
        </Tabs>
      </PageSection>
      <PageSection>
        <TabContent
          eventKey={0}
          id="refTab1Section"
          ref={contentRef0}
          aria-label="This is content for the first separate content tab"
        >
          Overview
        </TabContent>
        <TabContent
          eventKey={1}
          id="refTab2Section"
          ref={contentRef1}
          aria-label="This is content for the second separate content tab"
          hidden
        >
          <PackagesTable />
        </TabContent>
        <TabContent
          eventKey={2}
          id="refTab2Section"
          ref={contentRef2}
          aria-label="This is content for the second separate content tab"
          hidden
        >
          <VulnerabilitiesTable />
        </TabContent>
        <TabContent
          eventKey={3}
          id="refTab3Section"
          ref={contentRef3}
          aria-label="This is content for the third separate content tab"
          hidden
        >
          <CodeEditor
            isDarkTheme
            isLineNumbersVisible
            isReadOnly
            isMinimapVisible
            isLanguageLabelVisible
            code="SBOM content here"
            // onChange={onChange}
            language={Language.json}
            // onEditorDidMount={onEditorDidMount}
            height="400px"
          />
        </TabContent>
      </PageSection>
    </>
  );
};

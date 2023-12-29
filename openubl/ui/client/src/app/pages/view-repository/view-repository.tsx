import React from "react";
import { NavLink, useParams } from "react-router-dom";

import {
  Breadcrumb,
  BreadcrumbItem,
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
import { Tags } from "./components/tags";
import {
  useFetchRepositories,
  useRepositoryById,
} from "@app/queries/repositories";

export const ViewRepository: React.FC = () => {
  const refTags = React.createRef<HTMLElement>();

  let { repositoryId } = useParams();
  const { result: repository } = useRepositoryById(repositoryId!);

  return (
    <>
      <PageSection type="breadcrumb">
        <Breadcrumb>
          <BreadcrumbItem>
            <NavLink to="/repositories">Repositories</NavLink>
          </BreadcrumbItem>
          <BreadcrumbItem isActive>View repository</BreadcrumbItem>
        </Breadcrumb>
      </PageSection>
      <PageSection variant={PageSectionVariants.light}>
        <TextContent>
          <Text component="h1">{repository?.name}</Text>
          <Text component="p">{repository?.description}</Text>
        </TextContent>
      </PageSection>
      <PageSection type="tabs">
        <Tabs defaultActiveKey={0} inset={{ default: "insetLg" }}>
          <Tab
            eventKey={0}
            title={<TabTitleText>Tags</TabTitleText>}
            tabContentRef={refTags}
          />
        </Tabs>
      </PageSection>
      <PageSection>
        <TabContent eventKey={0} id="tags" ref={refTags} aria-label="Tags tab">
          {repository && <Tags repository={repository} />}
        </TabContent>
      </PageSection>
    </>
  );
};

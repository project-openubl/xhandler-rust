import type React from "react";

import { getString } from "@app/utils/utils";
import { Tooltip } from "@patternfly/react-core";

import type { AutocompleteOptionProps } from "./Autocomplete";

export const LabelToolip: React.FC<{
  content?: AutocompleteOptionProps["tooltip"];
  children: React.ReactElement;
}> = ({ content, children }) =>
  content ? (
    <Tooltip content={<div>{getString(content)}</div>}>{children}</Tooltip>
  ) : (
    children
  );

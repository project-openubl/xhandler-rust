import React from "react";

export const handlePropagatedRowClick = <
  E extends React.KeyboardEvent | React.MouseEvent
>(
  event: E | undefined,
  onRowClick: (event: E) => void
) => {
  // Check if there is a clickable element between the event target and the row such as a
  // checkbox, button or link. Don't trigger the row click if those are clicked.
  // This recursive parent check is necessary because the event target could be,
  // for example, the SVG icon inside a button rather than the button itself.
  const isClickableElementInTheWay = (element: Element): boolean => {
    if (["input", "button", "a"].includes(element.tagName.toLowerCase())) {
      return true;
    }
    if (
      !element.parentElement ||
      element.parentElement?.tagName.toLowerCase() === "tr"
    ) {
      return false;
    }
    return isClickableElementInTheWay(element.parentElement);
  };
  if (
    event?.target instanceof Element &&
    !isClickableElementInTheWay(event.target)
  ) {
    onRowClick(event);
  }
};

export interface ColumnSetting {
  // visibility status, can change in time
  isVisible?: boolean;
  // column is always visible because it's needed to uniquely identify the row
  isIdentity?: boolean;
}

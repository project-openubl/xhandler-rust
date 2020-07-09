---
id: design_principles
title: Design principles
---

- **You don't have to know UBL standard** - XBuilder do not require you to know the UBL details, it exposes a set of POJOs that are, internally, transpiled into XML files that follows the UBL standards.
- **Math operations must be provided out of the box** - XBuilder executes all math operations required to fill certain values in the XML files. Math operations like taxes, total amounts, discounts, etc. must be executed internally.
- **Apply default values when possible** - XBuilder should fill all missing data with default values. XBuilder requires only minimal data.

## How XBuilder works

We believe that you should provide as minimal data as possible and then allow XBuilder to do the rest for you. The input data provided by whoever uses XBuilder is not based in UBL but based in common and mainstream bussiness language.

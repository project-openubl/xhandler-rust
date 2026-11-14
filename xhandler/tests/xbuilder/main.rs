//! Snapshot tests of the XML created by xbuilder: each module checks the
//! rendered XML against tests/resources/xbuilder/e2e, validates it against the
//! UBL XSD and (unless XBUILDER_SKIP_SUNAT=true) sends it to SUNAT beta.

mod common;

mod catalog_names;
mod credit_note;
mod credit_note_atributos;
mod credit_note_isc;
mod credit_note_orden_compra;
mod credit_note_product_codes;
mod debit_note;
mod debit_note_atributos;
mod debit_note_product_codes;
mod despatch_advice;
mod enricher;
mod homologacion_group01;
mod homologacion_group02;
mod homologacion_group03;
mod homologacion_group04;
mod homologacion_group05;
mod homologacion_group06;
mod homologacion_group07;
mod homologacion_group08;
mod homologacion_group09;
mod homologacion_group10;
mod homologacion_group11;
mod homologacion_group12;
mod homologacion_group13;
mod homologacion_group14;
mod invoice;
mod invoice_anticipos;
mod invoice_atributos;
mod invoice_cargos;
mod invoice_descuentos;
mod invoice_detraccion;
mod invoice_direccion_entrega;
mod invoice_documento_relacionado;
mod invoice_fecha_vencimiento;
mod invoice_forma_pago;
mod invoice_guia_embebida;
mod invoice_guias;
mod invoice_igv_tipo;
mod invoice_isc;
mod invoice_issue30;
mod invoice_moneda;
mod invoice_orden_compra;
mod invoice_percepcion;
mod invoice_product_codes;
mod invoice_special_characters;
mod perception;
mod retention;
mod reversion;
mod summary_documents;
mod voided_documents;

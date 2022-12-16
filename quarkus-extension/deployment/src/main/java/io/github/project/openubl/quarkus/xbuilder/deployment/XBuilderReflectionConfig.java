/*
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 *
 * Licensed under the Apache License - 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.quarkus.xbuilder.deployment;

import io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument;
import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection(
        targets = {
                io.github.project.openubl.xbuilder.content.models.common.Cliente.class,
                io.github.project.openubl.xbuilder.content.models.common.Proveedor.class,
                io.github.project.openubl.xbuilder.content.models.common.Contacto.class,
                io.github.project.openubl.xbuilder.content.models.common.Firmante.class,
                io.github.project.openubl.xbuilder.content.models.common.Direccion.class,
                io.github.project.openubl.xbuilder.content.models.common.Cliente.ClienteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.Contacto.ContactoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.Firmante.FirmanteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.Proveedor.ProveedorBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.Direccion.DireccionBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Note.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Invoice.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Note.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporte.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteNote.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteInvoice.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote.DebitNoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote.CreditNoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CargoDescuento.CargoDescuentoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Invoice.InvoiceBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Note.NoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote.CreditNoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote.DebitNoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.FormaDePago.FormaDePagoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteNote.TotalImporteNoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporte.TotalImporteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteNote.TotalImporteNoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteInvoice.TotalImporteInvoiceBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument.SalesDocumentBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Invoice.InvoiceBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Note.NoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteInvoice.TotalImporteInvoiceBuilder.class,

                io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments.class,
                io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments.VoidedDocumentsBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocumentsItem.class,
                io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocumentsItem.VoidedDocumentsItemBuilder.class
        }
)
public class XBuilderReflectionConfig {
}

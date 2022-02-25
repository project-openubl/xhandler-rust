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
package io.github.project.openubl.xbuilder.enricher.pojos;

import io.github.project.openubl.xbuilder.content.models.common.*;
import io.github.project.openubl.xbuilder.content.models.standard.general.*;

public class Utils {

    private static void assignGeneric(BaseDocumento document, EnrichDocument data) {
        document.moneda = data.moneda;
        document.fechaEmision = data.fechaEmision;

        document.formaDePago = data.formaDePagoTipo;

        if (document.firmante == null) {
            document.firmante = new Firmante();
        }
        document.firmante.ruc = data.firmanteRuc;
        document.firmante.razonSocial = data.firmanteRazonSocial;

        if (document.proveedor == null) {
            document.proveedor = new Proveedor();
        }
        if (document.proveedor.direccion == null) {
            document.proveedor.direccion = new Direccion();
        }
        document.proveedor.direccion.codigoLocal = data.proveedorDireccionCodigoPostal;

        if (document.cliente == null) {
            document.cliente = new Cliente();
        }
        document.cliente.tipoDocumentoIdentidad = data.clienteTipoDocumentoIdentidad;
    }

    public static void assign(BoletaFactura invoice, EnrichDocument data) {
        assignGeneric(invoice, data);
        invoice.tipoComprobante = data.invoiceTipoComprobante;
    }

    public static void assign(NotaDeCredito creditNote, EnrichDocument data) {
        assignGeneric(creditNote, data);
        creditNote.tipoNota = data.noteTipoNota;
        creditNote.comprobanteAfectadoTipo = data.noteComprobanteAfectadoTipo;
    }

    public static void assign(NotaDeDebito debitNote, EnrichDocument data) {
        assignGeneric(debitNote, data);
        debitNote.tipoNota = data.noteTipoNota;
        debitNote.comprobanteAfectadoTipo = data.noteComprobanteAfectadoTipo;
    }

    public static void assign(DocumentoDetalle line, EnrichDocumentLine data) {
        line.unidadMedida = data.unidadMedida;
        line.igvTipo = data.igvTipo;
        line.icbTasa = data.icbTasa;
        line.igvTasa = data.igvTasa;
        line.precioReferenciaTipo = data.precioReferenciaTipo;
    }

}

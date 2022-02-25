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

import io.github.project.openubl.xbuilder.content.models.standard.general.*;

public class EnrichDocumentFactory {

    private static void assignGeneric(EnrichDocument result, BaseDocumento dto) {
        result.type = DocumentType.INVOICE;
        result.moneda = dto.moneda;
        result.fechaEmision = dto.fechaEmision;
        result.formaDePagoTipo = dto.formaDePago;
        result.formaDePagoNumeroCuotas = dto.formaDePagoCuotas != null ? dto.formaDePagoCuotas.size() : 0;

        if (dto.firmante != null) {
            result.firmanteRuc = dto.firmante.ruc;
            result.firmanteRazonSocial = dto.firmante.razonSocial;
        }

        if (dto.proveedor != null && dto.proveedor.direccion != null) {
            result.proveedorDireccionCodigoPostal = dto.proveedor.direccion.codigoLocal;
        }

        if (dto.cliente != null) {
            result.clienteTipoDocumentoIdentidad = dto.cliente.tipoDocumentoIdentidad;
        }
    }

   public static EnrichDocument build(BoletaFactura dto) {
       EnrichDocument result = new EnrichDocument();

       result.type = DocumentType.INVOICE;
       assignGeneric(result, dto);

       return result;
   }

    public static EnrichDocument build(NotaDeCredito dto) {
        EnrichDocument result = new EnrichDocument();

        result.type = DocumentType.CREDIT_NOTE;
        assignGeneric(result, dto);

        result.noteTipoNota = dto.tipoNota;
        result.noteComprobanteAfectadoTipo = dto.comprobanteAfectadoTipo;

        return result;
    }

    public static EnrichDocument build(NotaDeDebito dto) {
        EnrichDocument result = new EnrichDocument();

        result.type = DocumentType.DEBIT_NOTE;
        assignGeneric(result, dto);

        result.noteTipoNota = dto.tipoNota;
        result.noteComprobanteAfectadoTipo = dto.comprobanteAfectadoTipo;

        return result;
    }

    public static EnrichDocumentLine build(DocumentoDetalle dto) {
        EnrichDocumentLine result = new EnrichDocumentLine();

        result.index = dto.index;
        result.unidadMedida = dto.unidadMedida;
        result.igvTipo = dto.igvTipo;
        result.icbTasa = dto.icbTasa;
        result.igvTasa = dto.igvTasa;

        return result;
    }

}

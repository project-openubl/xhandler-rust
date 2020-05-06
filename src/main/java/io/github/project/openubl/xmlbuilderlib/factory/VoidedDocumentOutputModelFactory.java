/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 * <p>
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * <p>
 * https://www.eclipse.org/legal/epl-2.0/
 * <p>
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xmlbuilderlib.factory;

import io.github.project.openubl.xmlbuilderlib.config.XMLBuilderConfig;
import io.github.project.openubl.xmlbuilderlib.factory.common.FirmanteOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.factory.common.ProveedorOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.VoidedDocumentInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.VoidedDocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.VoidedDocumentLineOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.VoidedDocumentOutputModel;
import io.github.project.openubl.xmlbuilderlib.utils.SystemClock;

import java.text.MessageFormat;

import static io.github.project.openubl.xmlbuilderlib.utils.DateUtils.toGregorianCalendarDate;

public class VoidedDocumentOutputModelFactory {

    private VoidedDocumentOutputModelFactory() {
        // Only static methods
    }

    public static VoidedDocumentOutputModel getVoidedDocument(VoidedDocumentInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        VoidedDocumentOutputModel.Builder builder = VoidedDocumentOutputModel.Builder.aVoidedDocumentOutputModel();

        // datos generales
        String fechaEmision = input.getFechaEmision() != null
                ? toGregorianCalendarDate(input.getFechaEmision(), systemClock.getTimeZone())
                : toGregorianCalendarDate(systemClock.getCalendarInstance().getTimeInMillis(), systemClock.getTimeZone());

        builder.withFechaEmision(fechaEmision)
                .withDescripcionSustento(input.getDescripcionSustento())
                .withProveedor(ProveedorOutputModelFactory.getProveedor(input.getProveedor()))
                .withFirmante(
                        input.getFirmante() != null
                                ? FirmanteOutputModelFactory.getFirmante(input.getFirmante())
                                : FirmanteOutputModelFactory.getFirmante(input.getProveedor())
                );

        // Comprobante
        VoidedDocumentLineInputModel comprobanteAfectado = input.getComprobante();

        String[] serieNumeroComprobanteAfectado = comprobanteAfectado.getSerieNumero().split("-");
        Catalog1 tipoComprobanteAfectado = Catalog.valueOfCode(Catalog1.class, comprobanteAfectado.getTipoComprobante()).orElseThrow(Catalog.invalidCatalogValue);

        builder.withComprobante(VoidedDocumentLineOutputModel.Builder.aVoidedDocumentLineOutputModel()
                .withFechaEmision(toGregorianCalendarDate(comprobanteAfectado.getFechaEmision(), systemClock.getTimeZone()))
                .withTipoComprobante(tipoComprobanteAfectado)
                .withSerie(serieNumeroComprobanteAfectado[0])
                .withNumero(serieNumeroComprobanteAfectado[1])
                .build()
        );

        // Serie
        String codigo;
        switch (tipoComprobanteAfectado) {
            case PERCEPCION:
            case RETENCION:
            case GUIA_REMISION_REMITENTE:
                codigo = "RR";
                break;
            default:
                codigo = "RA";
        }

        builder.withSerieNumero(
                MessageFormat.format("{0}-{1}-{2}", codigo, fechaEmision.replaceAll("-", ""), input.getNumero())
        );

        return builder.build();
    }

}

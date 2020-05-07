/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 *
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.eclipse.org/legal/epl-2.0/
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xmlbuilderlib.factory;

import io.github.project.openubl.xmlbuilderlib.config.XMLBuilderConfig;
import io.github.project.openubl.xmlbuilderlib.factory.common.ClienteOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.factory.common.FirmanteOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.factory.common.ProveedorOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.*;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.*;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.*;
import io.github.project.openubl.xmlbuilderlib.utils.SystemClock;

import java.text.MessageFormat;
import java.util.function.Function;
import java.util.stream.Collectors;

import static io.github.project.openubl.xmlbuilderlib.utils.DateUtils.toGregorianCalendarDate;

public class SummaryDocumentOutputModelFactory {

    private SummaryDocumentOutputModelFactory() {
        // Only static methods
    }

    public static SummaryDocumentOutputModel getSummaryDocument(SummaryDocumentInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        Function<SummaryDocumentImpuestosInputModel, SummaryDocumentImpuestosOutputModel> summaryLineComprobanteImpuestosFn = inputImpuestos -> {
            SummaryDocumentImpuestosOutputModel.Builder builder = SummaryDocumentImpuestosOutputModel.Builder.aSummaryDocumentImpuestosOutputModel()
                    .withIgv(ImpuestoTotalResumenDiarioOutputModel.Builder.anImpuestoTotalResumenDiarioOutputModel()
                            .withImporte(inputImpuestos.getIgv())
                            .withCategoria(Catalog5.IGV)
                            .build()
                    );
            if (inputImpuestos.getIcb() != null) {
                builder.withIcb(ImpuestoTotalResumenDiarioOutputModel.Builder.anImpuestoTotalResumenDiarioOutputModel()
                        .withImporte(inputImpuestos.getIcb())
                        .withCategoria(Catalog5.ICBPER)
                        .build()
                );
            }
            return builder.build();
        };

        Function<SummaryDocumentComprobanteValorVentaInputModel, SummaryDocumentComprobanteValorVentaOutputModel> summaryLineComprobanteValorVentaFn = inputValorVenta -> {
            SummaryDocumentComprobanteValorVentaOutputModel.Builder builder = SummaryDocumentComprobanteValorVentaOutputModel.Builder.aSummaryDocumentComprobanteValorVentaOutputModel()
                    .withImporteTotal(inputValorVenta.getImporteTotal())
                    .withOtrosCargos(inputValorVenta.getOtrosCargos());
            if (inputValorVenta.getGravado() != null) {
                builder.withGravado(TotalValorVentaOutputModel.Builder.aTotalValorVentaOutputModel()
                        .withTipo(Catalog7_1.GRAVADO)
                        .withImporte(inputValorVenta.getGravado())
                        .build()
                );
            }
            if (inputValorVenta.getExonerado() != null) {
                builder.withExonerado(TotalValorVentaOutputModel.Builder.aTotalValorVentaOutputModel()
                        .withTipo(Catalog7_1.EXONERADO)
                        .withImporte(inputValorVenta.getExonerado())
                        .build()
                );
            }
            if (inputValorVenta.getInafecto() != null) {
                builder.withInafecto(TotalValorVentaOutputModel.Builder.aTotalValorVentaOutputModel()
                        .withTipo(Catalog7_1.INAFECTO)
                        .withImporte(inputValorVenta.getInafecto())
                        .build()
                );
            }
            if (inputValorVenta.getGratuito() != null) {
                builder.withGratuito(TotalValorVentaOutputModel.Builder.aTotalValorVentaOutputModel()
                        .withTipo(Catalog7_1.GRATUITA)
                        .withImporte(inputValorVenta.getGratuito())
                        .build()
                );
            }
            return builder.build();
        };

        Function<SummaryDocumentComprobanteInputModel, SummaryDocumentComprobanteOutputModel> summaryLineComprobanteFn = inputComprobante -> SummaryDocumentComprobanteOutputModel.Builder.aSummaryDocumentComprobanteOutputModel()
                .withTipo(
                        Catalog.valueOfCode(Catalog1.class, inputComprobante.getTipo()).orElseThrow(Catalog.invalidCatalogValue)
                )
                .withSerieNumero(inputComprobante.getSerieNumero())
                .withCliente(ClienteOutputModelFactory.getCliente(inputComprobante.getCliente()))
                .withImpuestos(summaryLineComprobanteImpuestosFn.apply(inputComprobante.getImpuestos()))
                .withValorVenta(summaryLineComprobanteValorVentaFn.apply(inputComprobante.getValorVenta()))
                .build();

        Function<SummaryDocumentLineInputModel, SummaryDocumentLineOutputModel> summaryLineFn = inputLineFn -> {
            SummaryDocumentLineOutputModel.Builder builder = SummaryDocumentLineOutputModel.Builder.aSummaryDocumentLineOutputModel()
                    .withTipoOperacion(
                            Catalog.valueOfCode(Catalog19.class, inputLineFn.getTipoOperacion()).orElseThrow(Catalog.invalidCatalogValue)
                    )
                    .withComprobante(summaryLineComprobanteFn.apply(inputLineFn.getComprobante()));
            if (inputLineFn.getComprobanteAfectado() != null) {
                builder.withComprobanteAfectado(SummaryDocumentComprobanteAfectadoOutputModel.Builder.aSummaryDocumentComprobanteAfectadoOutputModel()
                        .withSerieNumero(inputLineFn.getComprobanteAfectado().getSerieNumero())
                        .withTipo(
                                Catalog.valueOfCode(Catalog1.class, inputLineFn.getComprobanteAfectado().getTipo()).orElseThrow(Catalog.invalidCatalogValue)
                        )
                        .build()
                );
            }

            return builder.build();
        };


        //


        String fechaEmision = input.getFechaEmision() != null
                ? toGregorianCalendarDate(input.getFechaEmision(), systemClock.getTimeZone())
                : toGregorianCalendarDate(systemClock.getCalendarInstance().getTimeInMillis(), systemClock.getTimeZone());
        String serieNumero = MessageFormat.format("RC-{0}-{1}", fechaEmision.replaceAll("-", ""), input.getNumero());

        return SummaryDocumentOutputModel.Builder.aSummaryDocumentOutputModel()
                .withMoneda(config.getDefaultMoneda())
                .withFechaEmision(fechaEmision)
                .withSerieNumero(serieNumero)
                .withFechaEmisionDeComprobantesAsociados(
                        toGregorianCalendarDate(input.getFechaEmisionDeComprobantesAsociados(), systemClock.getTimeZone())
                )
                .withProveedor(ProveedorOutputModelFactory.getProveedor(input.getProveedor()))
                .withFirmante(
                        input.getFirmante() != null
                                ? FirmanteOutputModelFactory.getFirmante(input.getFirmante())
                                : FirmanteOutputModelFactory.getFirmante(input.getProveedor())
                )
                .withDetalle(input.getDetalle().stream()
                        .map(summaryLineFn)
                        .collect(Collectors.toList())
                )
                .build();
    }
}

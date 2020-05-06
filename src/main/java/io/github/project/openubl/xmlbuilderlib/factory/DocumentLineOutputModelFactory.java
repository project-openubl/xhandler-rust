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
import io.github.project.openubl.xmlbuilderlib.models.catalogs.*;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.*;
import io.github.project.openubl.xmlbuilderlib.utils.SystemClock;

import java.math.BigDecimal;
import java.math.RoundingMode;

public class DocumentLineOutputModelFactory {

    private DocumentLineOutputModelFactory() {
        // Only static methods
    }

    public static DocumentLineOutputModel getDocumentLineOutput(DocumentLineInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        DocumentLineOutputModel.Builder builder = DocumentLineOutputModel.Builder.aDocumentLineOutputModel()
                .withDescripcion(input.getDescripcion())
                .withUnidadMedida(input.getUnidadMedida() != null ? input.getUnidadMedida() : config.getDefaultUnidadMedida())
                .withCantidad(input.getCantidad())
                .withPrecioUnitario(input.getPrecioUnitario())
                .withPrecioConIgv(input.getPrecioConIgv());

        // Impuestos
        DocumentLineImpuestosOutputModel impuestosOutput;
        if (input.getPrecioUnitario() != null) {
            impuestosOutput = getDocumentLineImpuestosOutput_LeftRight(input, config, systemClock);
        } else if (input.getPrecioConIgv() != null) {
            impuestosOutput = getDocumentLineImpuestosOutput_RightLeft(input, config, systemClock);
        } else {
            throw new IllegalStateException("Precio con impuestos y/o sin impuestos no encontrado, no se pueden calcular los impuestos");
        }
        builder.withImpuestos(impuestosOutput);

        // Precio con/sin impuestos
        BigDecimal precioUnitario;
        BigDecimal precioConIgv;

        if (input.getPrecioUnitario() != null) {
            precioUnitario = input.getPrecioUnitario();
            precioConIgv = input.getPrecioUnitario().multiply(input.getCantidad())
                    .add(impuestosOutput.getIgv().getImporte())
                    .divide(input.getCantidad(), 2, RoundingMode.HALF_EVEN);
        } else if (input.getPrecioConIgv() != null) {
            precioUnitario = input.getPrecioConIgv().multiply(input.getCantidad())
                    .subtract(impuestosOutput.getIgv().getImporte())
                    .divide(input.getCantidad(), 2, RoundingMode.HALF_EVEN);
            precioConIgv = input.getPrecioConIgv();
        } else {
            throw new IllegalStateException("Precio con impuestos y/o sin impuestos no encontrado, no se pueden calcular el precion con impuestos");
        }

        builder.withPrecioConIgv(precioConIgv)
                .withPrecioUnitario(
                        // Trick to make <cbc:PriceAmount>0</cbc:PriceAmount>
                        impuestosOutput.getIgv().getTipo().isOperacionOnerosa() ? precioUnitario : BigDecimal.ZERO
                );


        // Precio de referencia
        builder.withPrecioDeReferencia(DocumentLinePrecioReferenciaOutputModel.Builder.aDetallePrecioReferenciaOutputModel()
                .withPrecio(precioConIgv)
                .withTipoPrecio(
                        impuestosOutput.getIgv().getTipo().isOperacionOnerosa()
                                ? Catalog16.PRECIO_UNITARIO_INCLUYE_IGV
                                : Catalog16.VALOR_FERENCIAL_UNITARIO_EN_OPERACIONES_NO_ONEROSAS
                )
                .build()
        );

        // Valor de venta (sin impuestos)
        BigDecimal valorVentaSinImpuestos = input.getCantidad().multiply(precioConIgv).setScale(2, RoundingMode.HALF_EVEN);

        if (impuestosOutput.getIgv().getTipo().isOperacionOnerosa()) {
            valorVentaSinImpuestos = valorVentaSinImpuestos.subtract(impuestosOutput.getIgv().getImporte());
        }
        builder.withValorVentaSinImpuestos(valorVentaSinImpuestos);

        return builder.build();
    }

    private static DocumentLineImpuestosOutputModel getDocumentLineImpuestosOutput_LeftRight(DocumentLineInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        DocumentLineImpuestosOutputModel.Builder builder = DocumentLineImpuestosOutputModel.Builder.aDocumentLineImpuestosOutputModel();


        BigDecimal subtotal = input.getCantidad().multiply(input.getPrecioUnitario()).setScale(2, RoundingMode.HALF_EVEN);

        // IGV
        Catalog7 igvTipo = input.getTipoIgv() != null
                ? Catalog.valueOfCode(Catalog7.class, input.getTipoIgv()).orElseThrow(Catalog.invalidCatalogValue)
                : config.getDefaultTipoIgv();

        BigDecimal igvValor;
        if (igvTipo.getGrupo().equals(Catalog7_1.GRAVADO)) {
            igvValor = igvTipo.equals(Catalog7.GRAVADO_IVAP) ? config.getIvap() : config.getIgv();
        } else {
            igvValor = BigDecimal.ZERO;
        }

        BigDecimal igvBaseImponible = subtotal.add(BigDecimal.ZERO); // Just to copy value
        BigDecimal igvImporte = igvBaseImponible.multiply(igvValor).setScale(2, RoundingMode.HALF_EVEN);

        builder.withIgv(ImpuestoDetalladoIGVOutputModel.Builder.anImpuestoDetalladoIGVOutputModel()
                .withTipo(igvTipo)
                .withCategoria(igvTipo.getTaxCategory())
                .withBaseImponible(igvBaseImponible)
                .withImporte(igvImporte)
                .withPorcentaje(igvValor.multiply(new BigDecimal("100")))
                .build()
        );

        // ICB
        BigDecimal icbImporte = BigDecimal.ZERO;
        if (input.isIcb()) {
            BigDecimal icbValor = config.getDefaultIcb();
            icbImporte = input.getCantidad().multiply(icbValor).setScale(2, RoundingMode.HALF_EVEN);

            builder.withIcb(ImpuestoDetalladoICBOutputModel.Builder.anImpuestoDetalladoICBOutputModel()
                    .withCategoria(Catalog5.ICBPER)
                    .withIcbValor(icbValor)
                    .withImporte(icbImporte)
                    .build());
        }

        return builder.withImporteTotal(
                igvImporte.add(icbImporte)
        ).build();
    }

    private static DocumentLineImpuestosOutputModel getDocumentLineImpuestosOutput_RightLeft(DocumentLineInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        DocumentLineImpuestosOutputModel.Builder builder = DocumentLineImpuestosOutputModel.Builder.aDocumentLineImpuestosOutputModel();


        BigDecimal total = input.getCantidad().multiply(input.getPrecioConIgv()).setScale(2, RoundingMode.HALF_EVEN);

        // ICB
        BigDecimal icbImporte = BigDecimal.ZERO;
        if (input.isIcb()) {
            BigDecimal icbValor = config.getDefaultIcb();
            icbImporte = input.getCantidad().multiply(icbValor).setScale(2, RoundingMode.HALF_EVEN);

            builder.withIcb(ImpuestoDetalladoICBOutputModel.Builder.anImpuestoDetalladoICBOutputModel()
                    .withCategoria(Catalog5.ICBPER)
                    .withIcbValor(icbValor)
                    .withImporte(icbImporte)
                    .build());
        }

        // IGV
        Catalog7 igvTipo = input.getTipoIgv() != null
                ? Catalog.valueOfCode(Catalog7.class, input.getTipoIgv()).orElseThrow(Catalog.invalidCatalogValue)
                : config.getDefaultTipoIgv();

        BigDecimal igvValor;
        if (igvTipo.getGrupo().equals(Catalog7_1.GRAVADO)) {
            igvValor = igvTipo.equals(Catalog7.GRAVADO_IVAP) ? config.getIvap() : config.getIgv();
        } else {
            igvValor = BigDecimal.ZERO;
        }

        BigDecimal igvBaseImponible = total.divide(igvValor.add(BigDecimal.ONE), 2, RoundingMode.HALF_EVEN);
        BigDecimal igvImporte = total.subtract(igvBaseImponible);

        builder.withIgv(ImpuestoDetalladoIGVOutputModel.Builder.anImpuestoDetalladoIGVOutputModel()
                .withTipo(igvTipo)
                .withCategoria(igvTipo.getTaxCategory())
                .withBaseImponible(igvBaseImponible)
                .withImporte(igvImporte)
                .withPorcentaje(igvValor.multiply(new BigDecimal("100")))
                .build()
        );

        return builder.withImporteTotal(
                igvImporte.add(icbImporte)
        ).build();
    }
}

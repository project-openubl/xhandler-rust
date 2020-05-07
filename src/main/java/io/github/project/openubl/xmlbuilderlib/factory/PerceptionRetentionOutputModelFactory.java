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
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog22;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog23;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.PerceptionInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.PerceptionRetentionInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.RetentionInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.*;
import io.github.project.openubl.xmlbuilderlib.utils.SystemClock;

import java.math.BigDecimal;
import java.math.RoundingMode;
import java.util.List;
import java.util.stream.Collectors;

import static io.github.project.openubl.xmlbuilderlib.utils.DateUtils.toGregorianCalendarDate;

public class PerceptionRetentionOutputModelFactory {

    private PerceptionRetentionOutputModelFactory() {
        // Only static methods
    }

    public static PerceptionOutputModel getPerception(PerceptionInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        Catalog22 regimen = Catalog.valueOfCode(Catalog22.class, input.getRegimen()).orElseThrow(Catalog.invalidCatalogValue);

        PerceptionOutputModel.Builder builder = PerceptionOutputModel.Builder.aPerceptionOutputModel()
                .withSerieNumero(input.getSerie() + "-" + input.getNumero())
                .withRegimen(regimen);

        enrichPerceptionRetention(input, builder, config, systemClock);

        // Result
        PerceptionOutputModel result = builder.build();

        // Detalle regimen
        result.getDetalle().forEach(f -> {
            BigDecimal importePercibidoRetenido = f.getComprobante().getImporteTotal()
                    .multiply(regimen.getPercent())
                    .divide(BigDecimal.valueOf(100), 2, RoundingMode.HALF_EVEN);
            BigDecimal importeTotalCobradoPagado = f.getComprobante().getImporteTotal()
                    .add(importePercibidoRetenido);

            f.setImportePercibidoRetenido(importePercibidoRetenido);
            f.setImporteTotalCobradoPagado(importeTotalCobradoPagado);
        });

        // Totales
        BigDecimal importeTotalCobradoPagado = result.getDetalle()
                .stream()
                .map(PerceptionRetentionLineOutputModel::getImporteTotalCobradoPagado)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        BigDecimal importeTotalPercibidoRetenido = result.getDetalle()
                .stream()
                .map(PerceptionRetentionLineOutputModel::getImportePercibidoRetenido)
                .reduce(BigDecimal.ZERO, BigDecimal::add);

        result.setImporteTotalCobradoPagado(importeTotalCobradoPagado);
        result.setImporteTotalPercibidoRetenido(importeTotalPercibidoRetenido);

        return result;
    }

    public static RetentionOutputModel getRetention(RetentionInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        Catalog23 regimen = Catalog.valueOfCode(Catalog23.class, input.getRegimen()).orElseThrow(Catalog.invalidCatalogValue);

        RetentionOutputModel.Builder builder = RetentionOutputModel.Builder.aRetentionOutputModel()
                .withSerieNumero(input.getSerie() + "-" + input.getNumero())
                .withRegimen(regimen);

        enrichPerceptionRetention(input, builder, config, systemClock);

        // Result
        RetentionOutputModel result = builder.build();

        // Detalle regimen
        result.getDetalle().forEach(f -> {
            BigDecimal importePercibidoRetenido = f.getComprobante().getImporteTotal()
                    .multiply(regimen.getPercent())
                    .divide(BigDecimal.valueOf(100), 2, RoundingMode.HALF_EVEN);
            BigDecimal importeTotalCobradoPagado = f.getComprobante().getImporteTotal()
                    .subtract(importePercibidoRetenido);

            f.setImportePercibidoRetenido(importePercibidoRetenido);
            f.setImporteTotalCobradoPagado(importeTotalCobradoPagado);
        });

        // Totales
        BigDecimal importeTotalCobradoPagado = result.getDetalle()
                .stream()
                .map(PerceptionRetentionLineOutputModel::getImporteTotalCobradoPagado)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        BigDecimal importeTotalPercibidoRetenido = result.getDetalle()
                .stream()
                .map(PerceptionRetentionLineOutputModel::getImportePercibidoRetenido)
                .reduce(BigDecimal.ZERO, BigDecimal::add);

        result.setImporteTotalCobradoPagado(importeTotalCobradoPagado);
        result.setImporteTotalPercibidoRetenido(importeTotalPercibidoRetenido);

        return result;
    }

    // Enrich

    private static void enrichPerceptionRetention(PerceptionRetentionInputModel input, PerceptionRetentionOutputModel.Builder builder, XMLBuilderConfig config, SystemClock systemClock) {
        builder
                .withMoneda(config.getDefaultMoneda())
                .withObservacion(input.getObservacion());

        // Fecha y hora de emision
        long fechaEmision = input.getFechaEmision() != null ? input.getFechaEmision() : systemClock.getCalendarInstance().getTimeInMillis();
        builder.withFechaEmision(toGregorianCalendarDate(fechaEmision, systemClock.getTimeZone()));

        // Proveedor
        builder.withProveedor(ProveedorOutputModelFactory.getProveedor(input.getProveedor()));

        // Cliente
        builder.withCliente(ClienteOutputModelFactory.getCliente(input.getCliente()));

        // Firmante
        builder.withFirmante(
                input.getFirmante() != null
                        ? FirmanteOutputModelFactory.getFirmante(input.getFirmante())
                        : FirmanteOutputModelFactory.getFirmante(input.getProveedor())
        );

        // Detalle
        List<PerceptionRetentionLineOutputModel> outputDetalle = input.getDetalle().stream()
                .map(f -> PerceptionRetentionLineOutputModel.Builder.aPerceptionRetentionLineOutputModel()
                        .withFechaCobroPago(f.getFechaCobroPago() != null
                                ? toGregorianCalendarDate(f.getFechaCobroPago(), systemClock.getTimeZone())
                                : toGregorianCalendarDate(fechaEmision, systemClock.getTimeZone())
                        )
                        .withNumeroCobroPago(f.getNumeroCobroPago() != null
                                ? f.getNumeroCobroPago()
                                : 1
                        )
                        .withImporteCobroPago(f.getImporteCobroPago() != null
                                ? f.getImporteCobroPago()
                                : f.getComprobante().getImporteTotal()
                        )
                        .withComprobante(PerceptionRetentionComprobanteOutputModel.Builder.aPerceptionRetentionComprobanteOutputModel()
                                .withTipo(Catalog.valueOfCode(Catalog1.class, f.getComprobante().getTipo()).orElseThrow(Catalog.invalidCatalogValue))
                                .withMoneda(f.getComprobante().getMoneda())
                                .withSerieNumero(f.getComprobante().getSerieNumero())
                                .withImporteTotal(f.getComprobante().getImporteTotal())
                                .withFechaEmision(toGregorianCalendarDate(f.getComprobante().getFechaEmision(), systemClock.getTimeZone()))
                                .build()
                        )
                        .build())
                .collect(Collectors.toList());

        builder.withDetalle(outputDetalle);
    }

}

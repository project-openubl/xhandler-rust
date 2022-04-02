package io.github.project.openubl.xbuilder.enricher.kie.rules.utils;

import io.github.project.openubl.xbuilder.content.catalogs.Catalog;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog5;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog7;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoDetalle;

import java.math.BigDecimal;
import java.util.List;
import java.util.Objects;
import java.util.function.Supplier;
import java.util.stream.Stream;

public class DetalleUtils {

    public static BigDecimal getImporteSinImpuestos(List<DocumentoDetalle> detalles) {
        return detalles.stream()
                .filter(item -> {
                    Catalog7 catalog7 = Catalog.valueOfCode(Catalog7.class, item.getIgvTipo()).orElseThrow(Catalog.invalidCatalogValue);
                    return !catalog7.getTaxCategory().equals(Catalog5.GRATUITO);
                })
                .map(DocumentoDetalle::getIgvBaseImponible)
                .filter(Objects::nonNull)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
    }

    public static BigDecimal getTotalImpuestos(List<DocumentoDetalle> detalles) {
        return detalles.stream()
                .filter(detalle -> {
                    Catalog7 catalog7 = Catalog.valueOfCode(Catalog7.class, detalle.getIgvTipo()).orElseThrow(Catalog.invalidCatalogValue);
                    return !catalog7.getTaxCategory().equals(Catalog5.GRATUITO);
                })
                .map(DocumentoDetalle::getTotalImpuestos)
                .filter(Objects::nonNull)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
    }

    public static Impuesto calImpuestoByTipo(List<DocumentoDetalle> detalle, Catalog5 categoria) {
        Supplier<Stream<DocumentoDetalle>> stream = () -> detalle.stream().filter($il -> {
            Catalog7 catalog7 = Catalog.valueOfCode(Catalog7.class, $il.getIgvTipo()).orElseThrow(Catalog.invalidCatalogValue);
            return catalog7.getTaxCategory().equals(categoria);
        });

        BigDecimal baseImponible = stream.get()
                .map(DocumentoDetalle::getIgvBaseImponible)
                .filter(Objects::nonNull)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        BigDecimal importe = stream.get()
                .map(DocumentoDetalle::getIgv)
                .filter(Objects::nonNull)
                .reduce(BigDecimal.ZERO, BigDecimal::add);

        return Impuesto.builder()
                .importe(importe)
                .baseImponible(baseImponible)
                .build();
    }
}

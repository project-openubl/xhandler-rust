package io.github.project.openubl.xmlbuilderlib.config;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog10;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog7;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog9;

import java.math.BigDecimal;

public interface XMLBuilderConfig {

    BigDecimal getIgv();

    BigDecimal getIvap();

    String getDefaultMoneda();

    String getDefaultUnidadMedida();

    Catalog9 getDefaultTipoNotaCredito();

    Catalog10 getDefaultTipoNotaDebito();

    BigDecimal getDefaultIcb();

    Catalog7 getDefaultTipoIgv();

}

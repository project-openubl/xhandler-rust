package io.github.project.openubl.xmlbuilderlib.config;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog10;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog7;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog9;

import java.math.BigDecimal;

public class DefaultXMLBuilderConfig implements XMLBuilderConfig {

    private BigDecimal igv;
    private BigDecimal ivap;
    private String defaultMoneda;
    private String defaultUnidadMedida;
    private Catalog9 defaultTipoNotaCredito;
    private Catalog10 defaultTipoNotaDebito;
    private BigDecimal defaultIcb;
    private Catalog7 defaultTipoIgv;

    public DefaultXMLBuilderConfig() {
        this.defaultMoneda = "PEN";
        this.defaultUnidadMedida = "NIU";
        this.igv = new BigDecimal("0.18");
        this.ivap = new BigDecimal("0.04");
        this.defaultIcb = new BigDecimal("0.2");
        this.defaultTipoNotaCredito = Catalog9.ANULACION_DE_LA_OPERACION;
        this.defaultTipoNotaDebito = Catalog10.AUMENTO_EN_EL_VALOR;
        this.defaultTipoIgv = Catalog7.GRAVADO_OPERACION_ONEROSA;
    }

    @Override
    public BigDecimal getIgv() {
        return igv;
    }

    @Override
    public BigDecimal getIvap() {
        return ivap;
    }

    @Override
    public String getDefaultMoneda() {
        return defaultMoneda;
    }

    @Override
    public String getDefaultUnidadMedida() {
        return defaultUnidadMedida;
    }

    @Override
    public Catalog9 getDefaultTipoNotaCredito() {
        return defaultTipoNotaCredito;
    }

    @Override
    public Catalog10 getDefaultTipoNotaDebito() {
        return defaultTipoNotaDebito;
    }

    @Override
    public BigDecimal getDefaultIcb() {
        return defaultIcb;
    }

    @Override
    public Catalog7 getDefaultTipoIgv() {
        return defaultTipoIgv;
    }

    public void setIgv(BigDecimal igv) {
        this.igv = igv;
    }

    public void setIvap(BigDecimal ivap) {
        this.ivap = ivap;
    }

    public void setDefaultMoneda(String defaultMoneda) {
        this.defaultMoneda = defaultMoneda;
    }

    public void setDefaultUnidadMedida(String defaultUnidadMedida) {
        this.defaultUnidadMedida = defaultUnidadMedida;
    }

    public void setDefaultTipoNotaCredito(Catalog9 defaultTipoNotaCredito) {
        this.defaultTipoNotaCredito = defaultTipoNotaCredito;
    }

    public void setDefaultTipoNotaDebito(Catalog10 defaultTipoNotaDebito) {
        this.defaultTipoNotaDebito = defaultTipoNotaDebito;
    }

    public void setDefaultIcb(BigDecimal defaultIcb) {
        this.defaultIcb = defaultIcb;
    }

    public void setDefaultTipoIgv(Catalog7 defaultTipoIgv) {
        this.defaultTipoIgv = defaultTipoIgv;
    }

}

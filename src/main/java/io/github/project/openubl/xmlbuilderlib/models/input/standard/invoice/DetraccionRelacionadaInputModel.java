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
package io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog54;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog59;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocTribRelacionadoInputModel;

import javax.validation.constraints.Min;
import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

public class DetraccionRelacionadaInputModel extends DocTribRelacionadoInputModel {

    @CatalogConstraint(value = Catalog59.class)
    private String medioDePago;

    @CatalogConstraint(value = Catalog54.class)
    private String tipoBienServicio;

    @Min(0)
    @NotNull
    private BigDecimal porcentaje;

    private String numeroCuentaBancaria;

    public String getMedioDePago() {
        return medioDePago;
    }

    public void setMedioDePago(String medioDePago) {
        this.medioDePago = medioDePago;
    }

    public String getTipoBienServicio() {
        return tipoBienServicio;
    }

    public void setTipoBienServicio(String tipoBienServicio) {
        this.tipoBienServicio = tipoBienServicio;
    }

    public BigDecimal getPorcentaje() {
        return porcentaje;
    }

    public void setPorcentaje(BigDecimal porcentaje) {
        this.porcentaje = porcentaje;
    }

    public String getNumeroCuentaBancaria() {
        return numeroCuentaBancaria;
    }

    public void setNumeroCuentaBancaria(String numeroCuentaBancaria) {
        this.numeroCuentaBancaria = numeroCuentaBancaria;
    }

}

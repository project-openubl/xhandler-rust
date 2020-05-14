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
package io.github.project.openubl.xmlbuilderlib.models.input.standard;

import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.FirmanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.DocumentInputModel_PuedeCrearComprobanteConSerieFConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.DocumentInputModel_PuedeCrearComprobanteConSerieFGroupValidation;

import javax.validation.Valid;
import javax.validation.constraints.*;
import java.util.List;

@DocumentInputModel_PuedeCrearComprobanteConSerieFConstraint(groups = DocumentInputModel_PuedeCrearComprobanteConSerieFGroupValidation.class)
public abstract class DocumentInputModel {

    @NotNull
    @NotBlank
    @Pattern(regexp = "^[F|f|B|b].*$")
    @Size(min = 4, max = 4)
    protected String serie;

    @NotNull
    @Min(1)
    @Max(99999999)
    private Integer numero;

    private Long fechaEmision;

    @NotNull
    @Valid
    private ClienteInputModel cliente;

    @NotNull
    @Valid
    private ProveedorInputModel proveedor;

    @Valid
    private FirmanteInputModel firmante;

    @NotNull
    @NotEmpty
    @Valid
    private List<DocumentLineInputModel> detalle;

    public String getSerie() {
        return serie;
    }

    public void setSerie(String serie) {
        this.serie = serie;
    }

    public Integer getNumero() {
        return numero;
    }

    public void setNumero(Integer numero) {
        this.numero = numero;
    }

    public Long getFechaEmision() {
        return fechaEmision;
    }

    public void setFechaEmision(Long fechaEmision) {
        this.fechaEmision = fechaEmision;
    }

    public ClienteInputModel getCliente() {
        return cliente;
    }

    public void setCliente(ClienteInputModel cliente) {
        this.cliente = cliente;
    }

    public ProveedorInputModel getProveedor() {
        return proveedor;
    }

    public void setProveedor(ProveedorInputModel proveedor) {
        this.proveedor = proveedor;
    }

    public FirmanteInputModel getFirmante() {
        return firmante;
    }

    public void setFirmante(FirmanteInputModel firmante) {
        this.firmante = firmante;
    }

    public List<DocumentLineInputModel> getDetalle() {
        return detalle;
    }

    public void setDetalle(List<DocumentLineInputModel> detalle) {
        this.detalle = detalle;
    }

}

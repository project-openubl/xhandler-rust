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
package io.github.project.openubl.xmlbuilderlib.models.input.common;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Size;

public class ProveedorInputModel {

    @NotNull
    @NotBlank
    @Size(min = 11, max = 11)
    private String ruc;

    private String nombreComercial;

    @NotNull
    @NotBlank
    private String razonSocial;

    @Valid
    private DireccionInputModel direccion;

    @Valid
    private ContactoInputModel contacto;

    public String getRuc() {
        return ruc;
    }

    public void setRuc(String ruc) {
        this.ruc = ruc;
    }

    public String getNombreComercial() {
        return nombreComercial;
    }

    public void setNombreComercial(String nombreComercial) {
        this.nombreComercial = nombreComercial;
    }

    public String getRazonSocial() {
        return razonSocial;
    }

    public void setRazonSocial(String razonSocial) {
        this.razonSocial = razonSocial;
    }

    public DireccionInputModel getDireccion() {
        return direccion;
    }

    public void setDireccion(DireccionInputModel direccion) {
        this.direccion = direccion;
    }

    public ContactoInputModel getContacto() {
        return contacto;
    }

    public void setContacto(ContactoInputModel contacto) {
        this.contacto = contacto;
    }

    public static final class Builder {
        private String ruc;
        private String nombreComercial;
        private String razonSocial;
        private DireccionInputModel direccion;
        private ContactoInputModel contacto;

        private Builder() {
        }

        public static Builder aProveedorInputModel() {
            return new Builder();
        }

        public Builder withRuc(String ruc) {
            this.ruc = ruc;
            return this;
        }

        public Builder withNombreComercial(String nombreComercial) {
            this.nombreComercial = nombreComercial;
            return this;
        }

        public Builder withRazonSocial(String razonSocial) {
            this.razonSocial = razonSocial;
            return this;
        }

        public Builder withDireccion(DireccionInputModel direccion) {
            this.direccion = direccion;
            return this;
        }

        public Builder withContacto(ContactoInputModel contacto) {
            this.contacto = contacto;
            return this;
        }

        public ProveedorInputModel build() {
            ProveedorInputModel proveedorInputModel = new ProveedorInputModel();
            proveedorInputModel.setRuc(ruc);
            proveedorInputModel.setNombreComercial(nombreComercial);
            proveedorInputModel.setRazonSocial(razonSocial);
            proveedorInputModel.setDireccion(direccion);
            proveedorInputModel.setContacto(contacto);
            return proveedorInputModel;
        }
    }
}

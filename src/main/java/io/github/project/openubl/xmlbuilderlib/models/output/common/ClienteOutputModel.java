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
package io.github.project.openubl.xmlbuilderlib.models.output.common;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;

public class ClienteOutputModel {

    @NotNull
    private Catalog6 tipoDocumentoIdentidad;

    @NotBlank
    private String numeroDocumentoIdentidad;

    @NotBlank
    private String nombre;

    @Valid
    private DireccionOutputModel direccion;

    @Valid
    private ContactoOutputModel contacto;

    public Catalog6 getTipoDocumentoIdentidad() {
        return tipoDocumentoIdentidad;
    }

    public void setTipoDocumentoIdentidad(Catalog6 tipoDocumentoIdentidad) {
        this.tipoDocumentoIdentidad = tipoDocumentoIdentidad;
    }

    public String getNumeroDocumentoIdentidad() {
        return numeroDocumentoIdentidad;
    }

    public void setNumeroDocumentoIdentidad(String numeroDocumentoIdentidad) {
        this.numeroDocumentoIdentidad = numeroDocumentoIdentidad;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public DireccionOutputModel getDireccion() {
        return direccion;
    }

    public void setDireccion(DireccionOutputModel direccion) {
        this.direccion = direccion;
    }

    public ContactoOutputModel getContacto() {
        return contacto;
    }

    public void setContacto(ContactoOutputModel contacto) {
        this.contacto = contacto;
    }

    public static final class Builder {
        private Catalog6 tipoDocumentoIdentidad;
        private String numeroDocumentoIdentidad;
        private String nombre;
        private DireccionOutputModel direccion;
        private ContactoOutputModel contacto;

        private Builder() {
        }

        public static Builder aClienteOutputModel() {
            return new Builder();
        }

        public Builder withTipoDocumentoIdentidad(Catalog6 tipoDocumentoIdentidad) {
            this.tipoDocumentoIdentidad = tipoDocumentoIdentidad;
            return this;
        }

        public Builder withNumeroDocumentoIdentidad(String numeroDocumentoIdentidad) {
            this.numeroDocumentoIdentidad = numeroDocumentoIdentidad;
            return this;
        }

        public Builder withNombre(String nombre) {
            this.nombre = nombre;
            return this;
        }

        public Builder withDireccion(DireccionOutputModel direccion) {
            this.direccion = direccion;
            return this;
        }

        public Builder withContacto(ContactoOutputModel contacto) {
            this.contacto = contacto;
            return this;
        }

        public ClienteOutputModel build() {
            ClienteOutputModel clienteOutputModel = new ClienteOutputModel();
            clienteOutputModel.setTipoDocumentoIdentidad(tipoDocumentoIdentidad);
            clienteOutputModel.setNumeroDocumentoIdentidad(numeroDocumentoIdentidad);
            clienteOutputModel.setNombre(nombre);
            clienteOutputModel.setDireccion(direccion);
            clienteOutputModel.setContacto(contacto);
            return clienteOutputModel;
        }
    }
}

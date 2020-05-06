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

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import org.eclipse.microprofile.openapi.annotations.media.Schema;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;

@Schema(name = "Cliente")
public class ClienteInputModel {

    @NotNull
    @NotBlank
    @CatalogConstraint(value = Catalog6.class)
    @Schema(example = "RUC", description = "Catalogo 06", enumeration = {
            "DOC_TRIB_NO_DOM_SIN_RUC", "0",
            "DNI", "1",
            "EXTRANJERIA", "4",
            "RUC", "6",
            "PASAPORTE", "7",
            "DEC_DIPLOMATICA", "A"
    })
    private String tipoDocumentoIdentidad;

    @NotNull
    @NotBlank
    @Schema(example = "12345678912")
    private String numeroDocumentoIdentidad;

    @NotNull
    @NotBlank
    @Schema(example = "Carlos Feria", description = "Nombre o Razón Social del cliente")
    private String nombre;

    @Valid
    private DireccionInputModel direccion;

    @Valid
    private ContactoInputModel contacto;

    public String getTipoDocumentoIdentidad() {
        return tipoDocumentoIdentidad;
    }

    public void setTipoDocumentoIdentidad(String tipoDocumentoIdentidad) {
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
        private String tipoDocumentoIdentidad;
        private String numeroDocumentoIdentidad;
        private String nombre;
        private DireccionInputModel direccion;
        private ContactoInputModel contacto;

        private Builder() {
        }

        public static Builder aClienteInputModel() {
            return new Builder();
        }

        public Builder withTipoDocumentoIdentidad(String tipoDocumentoIdentidad) {
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

        public Builder withDireccion(DireccionInputModel direccion) {
            this.direccion = direccion;
            return this;
        }

        public Builder withContacto(ContactoInputModel contacto) {
            this.contacto = contacto;
            return this;
        }

        public ClienteInputModel build() {
            ClienteInputModel clienteInputModel = new ClienteInputModel();
            clienteInputModel.setTipoDocumentoIdentidad(tipoDocumentoIdentidad);
            clienteInputModel.setNumeroDocumentoIdentidad(numeroDocumentoIdentidad);
            clienteInputModel.setNombre(nombre);
            clienteInputModel.setDireccion(direccion);
            clienteInputModel.setContacto(contacto);
            return clienteInputModel;
        }
    }
}

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
package io.github.project.openubl.xmlbuilderlib.factory.common;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ClienteOutputModel;

public class ClienteOutputModelFactory {

    private ClienteOutputModelFactory() {
        // Only static methods
    }

    public static ClienteOutputModel getCliente(ClienteInputModel input) {
        return ClienteOutputModel.Builder.aClienteOutputModel()
                .withNombre(input.getNombre())
                .withNumeroDocumentoIdentidad(input.getNumeroDocumentoIdentidad())
                .withTipoDocumentoIdentidad(Catalog.valueOfCode(Catalog6.class, input.getTipoDocumentoIdentidad()).orElseThrow(Catalog.invalidCatalogValue))
                .withContacto(input.getContacto() != null ? ContactoOutputModelFactory.getContacto(input.getContacto()) : null)
                .withDireccion(input.getDireccion() != null ? DireccionOutputModelFactory.getDireccion(input.getDireccion()) : null)
                .build();
    }

}

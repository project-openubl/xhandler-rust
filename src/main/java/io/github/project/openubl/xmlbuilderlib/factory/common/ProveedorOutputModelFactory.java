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
package io.github.project.openubl.xmlbuilderlib.factory.common;

import io.github.project.openubl.xmlbuilderlib.config.Constants;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.DireccionOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ProveedorOutputModel;

public class ProveedorOutputModelFactory {

    private ProveedorOutputModelFactory() {
        // Only static methods
    }

    public static ProveedorOutputModel getProveedor(ProveedorInputModel input) {
        return ProveedorOutputModel.Builder.aProveedorOutputModel()
                .withRuc(input.getRuc())
                .withRazonSocial(input.getRazonSocial())
                .withNombreComercial(input.getNombreComercial())
                .withDireccion(input.getDireccion() != null ? DireccionOutputModelFactory.getDireccion(input.getDireccion()) : getDefaultDireccion())
                .withContacto(input.getContacto() != null ? ContactoOutputModelFactory.getContacto(input.getContacto()) : null)
                .build();

    }

    private static DireccionOutputModel getDefaultDireccion() {
        return DireccionOutputModel.Builder.aDireccionOutputModel()
                .withCodigoLocal(Constants.DEFAULT_CODIGO_LOCAL)
                .build();
    }
}

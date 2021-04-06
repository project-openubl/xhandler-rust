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
        DireccionOutputModel direccionOutput = input.getDireccion() != null ?
                DireccionOutputModelFactory.getDireccion(input.getDireccion()) :
                new DireccionOutputModel();
        enrichDireccionOutput(direccionOutput);

        return ProveedorOutputModel.Builder.aProveedorOutputModel()
                .withRuc(input.getRuc())
                .withRazonSocial(input.getRazonSocial())
                .withNombreComercial(input.getNombreComercial())
                .withDireccion(direccionOutput)
                .withContacto(input.getContacto() != null ? ContactoOutputModelFactory.getContacto(input.getContacto()) : null)
                .build();

    }

    private static void enrichDireccionOutput(DireccionOutputModel direccionOutput) {
        if (direccionOutput.getCodigoLocal() == null) {
            direccionOutput.setCodigoLocal(Constants.DEFAULT_CODIGO_LOCAL);
        }
    }
}

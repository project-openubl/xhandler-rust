/*
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 *
 * Licensed under the Apache License - 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xbuilder.content.models.common;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * Dirección
 *
 * @author <a href="mailto:carlosthe19916@gmail.com">Carlos Feria</a>
 */
@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class Direccion {

    /**
     * Código de 6 dígitos que corresponden al Departamento, Provincia, y Distrito
     */
    private String ubigeo;

    /**
     * Código de cuatro dígitos asignado por SUNAT, que identifica al
     * establecimiento anexo. Dicho código se genera al momento la respectiva comunicación del
     * establecimiento. Tratándose del domicilio fiscal y en el caso de no poder determinar el lugar
     * de la venta, informar “0000”.
     */
    private String codigoLocal;

    /**
     * Nombre de la urbanización.
     */
    private String urbanizacion;

    /**
     * Nombre del Departamento o Región. Ejemplo: Ayacucho
     */
    private String departamento;

    /**
     * Nombre de la Provincia. Ejemplo: Huamanga
     */
    private String provincia;

    /**
     * Nombre del Distrito. Ejemplo: Quinua
     */
    private String distrito;

    /**
     * Dirección. Ejemplo: Jirón las piedras 123
     */
    private String direccion;

    /**
     * Código de 2 dígitos que corresponde al país a la que pertenece la dirección. Ejemplo: PE
     */
    private String codigoPais;
}

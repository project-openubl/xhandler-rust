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
package io.github.project.openubl.xbuilder.content.models.standard.general;

import java.math.BigDecimal;

import lombok.Builder;
import lombok.Data;

/**
 * Anticipo realizado.
 * <p>
 * Un anticipo se realiza cuando el cliente a entregado al proveedor un pago por anticipado a la prestación del servicio o la entrega.
 *
 * @author <a href="mailto:carlosthe19916@gmail.com">Carlos Feria</a>
 */
@Data
@Builder
public class Anticipo {

    /**
     * Tipo de anticipo realizado.
     * <p>
     * Catalogo 53.
     * <p>
     * Valores válidos: "04", "05", "06"
     */
    private String tipo;

    /**
     * Serie y número de comprobante del anticipo, por ejemplo "F123-4"
     */
    private String comprobanteSerieNumero;

    /**
     * Código de tipo de documento del {@link #comprobanteSerieNumero}.
     */
    private String comprobanteTipo;

    /**
     * Monto prepagado o anticipado
     */
    private BigDecimal monto;
}

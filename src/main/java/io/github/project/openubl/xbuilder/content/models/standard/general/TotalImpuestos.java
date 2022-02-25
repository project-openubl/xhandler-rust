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

public class TotalImpuestos {
    public BigDecimal total;
    public BigDecimal ivapImporte;
    public BigDecimal ivapBaseImponible;
    public BigDecimal gravadoImporte;
    public BigDecimal gravadoBaseImponible;
    public BigDecimal inafectoImporte;
    public BigDecimal inafectoBaseImponible;
    public BigDecimal exoneradoImporte;
    public BigDecimal exoneradoBaseImponible;
    public BigDecimal gratuitoImporte;
    public BigDecimal gratuitoBaseImponible;
    public BigDecimal icbImporte;
}

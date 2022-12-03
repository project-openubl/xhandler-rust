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
package io.github.project.openubl.quarkus.xbuilder.runtime;

import io.quarkus.runtime.annotations.ConfigItem;
import io.quarkus.runtime.annotations.ConfigPhase;
import io.quarkus.runtime.annotations.ConfigRoot;

import java.math.BigDecimal;
import java.util.Optional;

@ConfigRoot(name = "xbuilder", phase = ConfigPhase.RUN_TIME)
public class XBuilderConfig {

    /**
     * Default moneda
     */
    @ConfigItem
    public Optional<String> moneda;

    /**
     * Default unidadMedida
     */
    @ConfigItem
    public Optional<String> unidadMedida;

    /**
     * Default igvTasa
     */
    @ConfigItem
    public Optional<BigDecimal> igvTasa;

    /**
     * Default icbTasa
     */
    @ConfigItem
    public Optional<BigDecimal> icbTasa;
}

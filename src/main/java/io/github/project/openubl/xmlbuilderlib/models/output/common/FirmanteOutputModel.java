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

import javax.validation.constraints.NotBlank;

public class FirmanteOutputModel {

    @NotBlank
    private String ruc;

    @NotBlank
    private String razonSocial;

    public String getRuc() {
        return ruc;
    }

    public void setRuc(String ruc) {
        this.ruc = ruc;
    }

    public String getRazonSocial() {
        return razonSocial;
    }

    public void setRazonSocial(String razonSocial) {
        this.razonSocial = razonSocial;
    }

    public static final class Builder {
        private String ruc;
        private String razonSocial;

        private Builder() {
        }

        public static Builder aFirmanteOutputModel() {
            return new Builder();
        }

        public Builder withRuc(String ruc) {
            this.ruc = ruc;
            return this;
        }

        public Builder withRazonSocial(String razonSocial) {
            this.razonSocial = razonSocial;
            return this;
        }

        public FirmanteOutputModel build() {
            FirmanteOutputModel firmanteOutputModel = new FirmanteOutputModel();
            firmanteOutputModel.setRuc(ruc);
            firmanteOutputModel.setRazonSocial(razonSocial);
            return firmanteOutputModel;
        }
    }
}

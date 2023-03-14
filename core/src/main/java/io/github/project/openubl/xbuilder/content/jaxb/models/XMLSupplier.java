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
package io.github.project.openubl.xbuilder.content.jaxb.models;

import lombok.Data;
import lombok.NoArgsConstructor;


@Data
@NoArgsConstructor
public class XMLSupplier {
    @XmlPath("cac:PartyIdentification/cbc:ID/text()")
    private String partyIdentification_id;

    @XmlPath("cac:PartyName/cbc:Name/text()")
    private String partyName;

    @XmlPath("cac:PartyLegalEntity/cbc:RegistrationName/text()")
    private String registrationName;

    @XmlPath("cac:PartyLegalEntity/cac:RegistrationAddress")
    private XMLAddress address;

    @XmlPath("cac:Contact")
    private XMLContact contact;
}

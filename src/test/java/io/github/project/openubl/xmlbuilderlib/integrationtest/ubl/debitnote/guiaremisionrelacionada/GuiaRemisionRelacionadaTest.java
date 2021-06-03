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
package io.github.project.openubl.xmlbuilderlib.integrationtest.ubl.debitnote.guiaremisionrelacionada;

import io.github.project.openubl.xmlbuilderlib.facade.DocumentManager;
import io.github.project.openubl.xmlbuilderlib.facade.DocumentWrapper;
import io.github.project.openubl.xmlbuilderlib.integrationtest.AbstractUBLTest;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1_Guia;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.GuiaRemisionRelacionadaInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.creditNote.CreditNoteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.debitNote.DebitNoteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.creditNote.CreditNoteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.debitNote.DebitNoteOutputModel;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.util.Arrays;

public class GuiaRemisionRelacionadaTest extends AbstractUBLTest {

    public GuiaRemisionRelacionadaTest() throws Exception {
    }

    @Test
    void testGuiaRemisionRemitente() throws Exception {
        // Given
        DebitNoteInputModel input = DebitNoteInputModel.Builder.aDebitNoteInputModel()
                .withSerie("FD01")
                .withNumero(1)
                .withSerieNumeroComprobanteAfectado("F001-1")
                .withDescripcionSustento("mi sustento")
                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                        .withRuc("12345678912")
                        .withRazonSocial("Softgreen S.A.C.")
                        .build()
                )
                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
                        .withNombre("Carlos Feria")
                        .withNumeroDocumentoIdentidad("12121212121")
                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
                        .build()
                )
                .withDetalle(Arrays.asList(
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item1")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build())
                )
                .withGuiasRemisionRelacionadas(Arrays.asList(
                        GuiaRemisionRelacionadaInputModel.Builder.aGuiaRemisionRelacionadaInputModel()
                                .withSerieNumero("0001-002020")
                                .withTipoDocumento(Catalog1_Guia.GUIA_REMISION_REMITENTE.toString())
                                .build(),
                        GuiaRemisionRelacionadaInputModel.Builder.aGuiaRemisionRelacionadaInputModel()
                                .withSerieNumero("0002-002020")
                                .withTipoDocumento(Catalog1_Guia.GUIA_REMISION_REMITENTE.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<DebitNoteOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        DebitNoteOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/debitnote/guiaremisionrelacionada/guiaRemisionRemitente.xml");
        assertSendSunat(xml);
    }

    @Test
    void testGuiaRemisionTransportista() throws Exception {
        // Given
        DebitNoteInputModel input = DebitNoteInputModel.Builder.aDebitNoteInputModel()
                .withSerie("FD01")
                .withNumero(1)
                .withSerieNumeroComprobanteAfectado("F001-1")
                .withDescripcionSustento("mi sustento")
                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                        .withRuc("12345678912")
                        .withRazonSocial("Softgreen S.A.C.")
                        .build()
                )
                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
                        .withNombre("Carlos Feria")
                        .withNumeroDocumentoIdentidad("12121212121")
                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
                        .build()
                )
                .withDetalle(Arrays.asList(
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item1")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build())
                )
                .withGuiasRemisionRelacionadas(Arrays.asList(
                        GuiaRemisionRelacionadaInputModel.Builder.aGuiaRemisionRelacionadaInputModel()
                                .withSerieNumero("0001-002020")
                                .withTipoDocumento(Catalog1_Guia.GUIA_REMISION_TRANSPORTISTA.toString())
                                .build(),
                        GuiaRemisionRelacionadaInputModel.Builder.aGuiaRemisionRelacionadaInputModel()
                                .withSerieNumero("0002-002020")
                                .withTipoDocumento(Catalog1_Guia.GUIA_REMISION_TRANSPORTISTA.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<DebitNoteOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        DebitNoteOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/debitnote/guiaremisionrelacionada/guiaRemisionTransportista.xml");
        assertSendSunat(xml);
    }

}

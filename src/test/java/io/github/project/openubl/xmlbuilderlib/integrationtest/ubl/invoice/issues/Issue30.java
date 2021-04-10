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
package io.github.project.openubl.xmlbuilderlib.integrationtest.ubl.invoice.issues;

import io.github.project.openubl.xmlbuilderlib.facade.DocumentManager;
import io.github.project.openubl.xmlbuilderlib.facade.DocumentWrapper;
import io.github.project.openubl.xmlbuilderlib.integrationtest.AbstractUBLTest;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.DireccionInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.FirmanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice.InvoiceInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice.InvoiceOutputModel;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.util.Collections;

/**
 * Description: La base imponible a nivel de linea difiere de la información consiganada en el comprobante
 * Issue: https://github.com/project-openubl/xbuilder/issues/30
 */
public class Issue30 extends AbstractUBLTest {

    public Issue30() throws Exception {
    }

    @Test
    void testInvoice_withPrecioUnitario() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                        .withRuc("12345678912")
                        .withRazonSocial("Project OpenUBL S.A.C.")
                        .withDireccion(DireccionInputModel.Builder.aDireccionInputModel()
                                .withDireccion("Jr. las flores 123")
                                .build()
                        )
                        .build()
                )
                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
                        .withNombre("Carlos Feria")
                        .withNumeroDocumentoIdentidad("12121212121")
                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
                        .build()
                )
                .withFirmante(FirmanteInputModel.Builder.aFirmanteInputModel()
                        .withRuc("000000000000")
                        .withRazonSocial("Wolsnut4 S.A.C.")
                        .build()
                )
                .withDetalle(Collections.singletonList(
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item1")
                                .withCantidad(new BigDecimal("10"))
                                .withPrecioUnitario(new BigDecimal("6.68"))
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/issues/issue-30-with-precioUnitario.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }

    @Test
    void testInvoice_withPrecioConIgv() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                        .withRuc("12345678912")
                        .withRazonSocial("Project OpenUBL S.A.C.")
                        .withDireccion(DireccionInputModel.Builder.aDireccionInputModel()
                                .withDireccion("Jr. las flores 123")
                                .build()
                        )
                        .build()
                )
                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
                        .withNombre("Carlos Feria")
                        .withNumeroDocumentoIdentidad("12121212121")
                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
                        .build()
                )
                .withFirmante(FirmanteInputModel.Builder.aFirmanteInputModel()
                        .withRuc("000000000000")
                        .withRazonSocial("Wolsnut4 S.A.C.")
                        .build()
                )
                .withDetalle(Collections.singletonList(
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item1")
                                .withCantidad(new BigDecimal("10"))
                                .withPrecioConIgv(new BigDecimal("7.88"))
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/issues/issue-30-with-precioConIgv.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }

    @Test
    void testInvoice_withPrecioUnitario_andICB() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                        .withRuc("12345678912")
                        .withRazonSocial("Project OpenUBL S.A.C.")
                        .withDireccion(DireccionInputModel.Builder.aDireccionInputModel()
                                .withDireccion("Jr. las flores 123")
                                .build()
                        )
                        .build()
                )
                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
                        .withNombre("Carlos Feria")
                        .withNumeroDocumentoIdentidad("12121212121")
                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
                        .build()
                )
                .withFirmante(FirmanteInputModel.Builder.aFirmanteInputModel()
                        .withRuc("000000000000")
                        .withRazonSocial("Wolsnut4 S.A.C.")
                        .build()
                )
                .withDetalle(Collections.singletonList(
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item1")
                                .withCantidad(new BigDecimal("10"))
                                .withPrecioUnitario(new BigDecimal("6.68"))
                                .withIcb(true)
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/issues/issue-30-with-precioUnitario_andICB.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }

    @Test
    void testInvoice_withPrecioConIgv_andICB() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                        .withRuc("12345678912")
                        .withRazonSocial("Project OpenUBL S.A.C.")
                        .withDireccion(DireccionInputModel.Builder.aDireccionInputModel()
                                .withDireccion("Jr. las flores 123")
                                .build()
                        )
                        .build()
                )
                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
                        .withNombre("Carlos Feria")
                        .withNumeroDocumentoIdentidad("12121212121")
                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
                        .build()
                )
                .withFirmante(FirmanteInputModel.Builder.aFirmanteInputModel()
                        .withRuc("000000000000")
                        .withRazonSocial("Wolsnut4 S.A.C.")
                        .build()
                )
                .withDetalle(Collections.singletonList(
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item1")
                                .withCantidad(new BigDecimal("10"))
                                .withPrecioConIgv(new BigDecimal("7.88"))
                                .withIcb(true)
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/issues/issue-30-with-precioConIgv_andICB.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }
}

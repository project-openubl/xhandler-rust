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
package io.github.project.openubl.xmlbuilderlib.integrationtest.ubl.invoice;

import io.github.project.openubl.xmlbuilderlib.facade.DocumentManager;
import io.github.project.openubl.xmlbuilderlib.facade.DocumentWrapper;
import io.github.project.openubl.xmlbuilderlib.integrationtest.AbstractUBLTest;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.input.common.*;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice.InvoiceInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice.InvoiceOutputModel;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.util.Arrays;
import java.util.Calendar;

public class InvoiceTest extends AbstractUBLTest {

    public InvoiceTest() throws Exception {
    }

    @Test
    void testInvoiceWithCustomUnidadMedida() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
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
                                .withUnidadMedida("KGM")
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withUnidadMedida("KGM")
                                .build())
                )
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/customUnidadMedida.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }

    @Test
    void testInvoiceWithCustomFechaEmision() throws Exception {
        // Given
        Calendar calendar = Calendar.getInstance();
        calendar.set(2019, Calendar.JANUARY, 6, 0, 0, 0);
        calendar.setTimeZone(timeZone);

        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
                .withFechaEmision(calendar.getTimeInMillis())
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
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/customFechaEmision.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }

    @Test
    void testInvoiceWithCustomClienteDireccionAndContacto() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                        .withRuc("12345678912")
                        .withRazonSocial("Softgreen S.A.C.")
                        .build()
                )
                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
                        .withNombre("Carlos Feria")
                        .withNumeroDocumentoIdentidad("12121212121")
                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
                        .withContacto(ContactoInputModel.Builder.aContactoInputModel()
                                .withEmail("carlos@gmail.com")
                                .withTelefono("+123456789")
                                .build())
                        .withDireccion(DireccionInputModel.Builder.aDireccionInputModel()
                                .withUbigeo("050101")
                                .withDepartamento("Ayacucho")
                                .withProvincia("Huamanga")
                                .withDistrito("Jesus Nazareno")
                                .withCodigoLocal("0101")
                                .withUrbanizacion("000000")
                                .withDireccion("Jr. Las piedras 123")
                                .withCodigoPais("PE")
                                .build())
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
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/customClienteDireccionAndContacto.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }

    @Test
    void testInvoiceWithCustomProveedorDireccionAndContacto() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                        .withRuc("12345678912")
                        .withRazonSocial("Softgreen S.A.C.")
                        .withContacto(ContactoInputModel.Builder.aContactoInputModel()
                                .withEmail("carlos@gmail.com")
                                .withTelefono("+123456789")
                                .build())
                        .withDireccion(DireccionInputModel.Builder.aDireccionInputModel()
                                .withUbigeo("050101")
                                .withDepartamento("Ayacucho")
                                .withProvincia("Huamanga")
                                .withDistrito("Jesus Nazareno")
                                .withCodigoLocal("0001")
                                .withUrbanizacion("000000")
                                .withDireccion("Jr. Las piedras 123")
                                .withCodigoPais("PE")
                                .build())
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
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/customProveedorDireccionAndContacto.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceWithCustomFirmante() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
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
                .withFirmante(FirmanteInputModel.Builder.aFirmanteInputModel()
                        .withRuc("000000000000")
                        .withRazonSocial("Wolsnut4 S.A.C.")
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
                .build();



        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/customFirmante.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }

    @Test
    void testInvoiceWithICB_precioUnitario() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
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
                .withFirmante(FirmanteInputModel.Builder.aFirmanteInputModel()
                        .withRuc("000000000000")
                        .withRazonSocial("Wolsnut4 S.A.C.")
                        .build()
                )
                .withDetalle(Arrays.asList(
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item1")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withIcb(true)
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withIcb(true)
                                .build())
                )
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/icb.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }

    @Test
    void testInvoiceWithICB_precioConIgv() throws Exception {
        // Given
        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
                .withSerie("F001")
                .withNumero(1)
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
                .withFirmante(FirmanteInputModel.Builder.aFirmanteInputModel()
                        .withRuc("000000000000")
                        .withRazonSocial("Wolsnut4 S.A.C.")
                        .build()
                )
                .withDetalle(Arrays.asList(
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item1")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withIcb(true)
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withIcb(true)
                                .build())
                )
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/icb.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }

    @Test
    void testInvoiceWithCustomProveedor_direccionNotNullAndCodigoLocalNull() throws Exception {
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
                .withDetalle(Arrays.asList(
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item1")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withIcb(true)
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withIcb(true)
                                .build())
                )
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/customCodigoLocal.xml");
        assertSendSunat(xml, PROVIDER_WITHOUT_ADDRESS_NOTE);
    }
}

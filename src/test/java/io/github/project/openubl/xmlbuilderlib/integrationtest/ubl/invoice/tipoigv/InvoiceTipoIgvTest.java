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
package io.github.project.openubl.xmlbuilderlib.integrationtest.ubl.invoice.tipoigv;

import io.github.project.openubl.xmlbuilderlib.facade.DocumentFacade;
import io.github.project.openubl.xmlbuilderlib.integrationtest.AbstractUBLTest;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog7;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice.InvoiceInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice.InvoiceOutputModel;
import io.github.project.openubl.xmlbuilderlib.utils.InputToOutput;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.util.Arrays;

public class InvoiceTipoIgvTest extends AbstractUBLTest {

    public InvoiceTipoIgvTest() throws Exception {
    }

    @Test
    void testInvoiceTipoIgv_GravadoOnerosa_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                                .build())
                )
                .build();



        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoOnerosa.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoOnerosa_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoOnerosa.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiroPorPremio_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_PREMIO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_PREMIO.toString())
                                .build())
                )
                .build();



        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiroPorPremio.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiroPorPremio_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_PREMIO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_PREMIO.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiroPorPremio.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiroPorDonacion_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_DONACION.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_DONACION.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiroPorDonacion.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiroPorDonacion_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_DONACION.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_DONACION.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiroPorDonacion.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiro_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiro.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiro_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiro.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiroPorPublicidad_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_PUBLICIDAD.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_PUBLICIDAD.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiroPorPublicidad.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiroPorPublicidad_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_PUBLICIDAD.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_PUBLICIDAD.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiroPorPublicidad.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoBonificaciones_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.GRAVADO_BONIFICACIONES.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.GRAVADO_BONIFICACIONES.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoBonificaciones.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoBonificaciones_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_BONIFICACIONES.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_BONIFICACIONES.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoBonificaciones.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiroPorEntregaATrabajadores_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_ENTREGA_A_TRABAJADORES.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_ENTREGA_A_TRABAJADORES.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiroPorEntregaATrabajadores.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoRetiroPorEntregaATrabajadores_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_ENTREGA_A_TRABAJADORES.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(118))
                                .withTipoIgv(Catalog7.GRAVADO_RETIRO_POR_ENTREGA_A_TRABAJADORES.toString())
                                .build())
                )
                .build();



        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoRetiroPorEntregaATrabajadores.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoIVAP_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.GRAVADO_IVAP.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.GRAVADO_IVAP.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoIVAP.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_GravadoIVAP_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(104))
                                .withTipoIgv(Catalog7.GRAVADO_IVAP.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(104))
                                .withTipoIgv(Catalog7.GRAVADO_IVAP.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/gravadoIVAP.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_ExoneradoOperacionOnerosa_precionSinImpuestos() throws Exception {
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
                                .withTipoIgv(Catalog7.EXONERADO_OPERACION_ONEROSA.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.EXONERADO_OPERACION_ONEROSA.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/exoneradoOperacionOnerosa.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_ExoneradoOperacionOnerosa_precionConImpuestos() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.EXONERADO_OPERACION_ONEROSA.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.EXONERADO_OPERACION_ONEROSA.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/exoneradoOperacionOnerosa.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_ExoneradoTransferenciaGratuita_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.EXONERADO_TRANSFERENCIA_GRATUITA.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.EXONERADO_TRANSFERENCIA_GRATUITA.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/exoneradoTransferenciaGratuita.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_ExoneradoTransferenciaGratuita_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.EXONERADO_TRANSFERENCIA_GRATUITA.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.EXONERADO_TRANSFERENCIA_GRATUITA.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/exoneradoTransferenciaGratuita.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoOperacionOnerosa_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.INAFECTO_OPERACION_ONEROSA.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_OPERACION_ONEROSA.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoOperacionOnerosa.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoOperacionOnerosa_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_OPERACION_ONEROSA.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_OPERACION_ONEROSA.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoOperacionOnerosa.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorBonificacion_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiroPorBonificacion.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorBonificacion_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiroPorBonificacion.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiro_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiro.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiro_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiro.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorMuestrasMedicas_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoPorMuestrasMedicas.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorMuestrasMedicas_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoPorMuestrasMedicas.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorConvenioColectivo_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiroPorConvenioColectivo.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorConvenioColectivo_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiroPorConvenioColectivo.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorPremio_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_PREMIO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_PREMIO.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiroPorPremio.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorPremio_precioConIgv() throws Exception {
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
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_PREMIO.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_PREMIO.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiroPorPremio.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorPublicidad_precioUnitario() throws Exception {
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
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_PUBLICIDAD.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_PUBLICIDAD.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiroPorPublicidad.xml");
        assertSendSunat(xml);
    }

    @Test
    void testInvoiceTipoIgv_InafectoRetiroPorPublicidad_precioConIgv() throws Exception {
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
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_PUBLICIDAD.toString())
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioConIgv(new BigDecimal(100))
                                .withTipoIgv(Catalog7.INAFECTO_RETIRO_POR_PUBLICIDAD.toString())
                                .build())
                )
                .build();

        // When
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = DocumentFacade.createXML(input, config, systemClock);

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/tipoigv/inafectoRetiroPorPublicidad.xml");
        assertSendSunat(xml);
    }

//    @Test
//    void testInvoiceTipoIgv_Exportacion_precioUnitario() throws Exception {
//        // Given
//        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
//                .withSerie("F001")
//                .withNumero(1)
//                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
//                        .withRuc("12345678912")
//                        .withRazonSocial("Softgreen S.A.C.")
//                        .build()
//                )
//                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
//                        .withNombre("Carlos Feria")
//                        .withNumeroDocumentoIdentidad("12121212121")
//                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
//                        .build()
//                )
//                .withDetalle(Arrays.asList(
//                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
//                                .withDescripcion("Item1")
//                                .withCantidad(new BigDecimal(10))
//                                .withPrecioUnitario(new BigDecimal(100))
//                                .withTipoIgv(Catalog7.EXPORTACION.toString())
//                                .build(),
//                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
//                                .withDescripcion("Item2")
//                                .withCantidad(new BigDecimal(10))
//                                .withPrecioUnitario(new BigDecimal(100))
//                                .withTipoIgv(Catalog7.EXPORTACION.toString())
//                                .build())
//                )
//                .build();
//
//        // When
//        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
//        String xml = DocumentFacade.createXML(input, config, systemClock);
//
//        // Then
//        assertOutputHasNoConstraintViolations(validator, output);
//        assertSnapshot(xml, "xml/invoice/tipoigv/exportacion.xml");
//        assertSendSunat(xml);
//    }
//
//    @Test
//    void testInvoiceTipoIgv_Exportacion_precioConIgv() throws Exception {
//        // Given
//        InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
//                .withSerie("F001")
//                .withNumero(1)
//                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
//                        .withRuc("12345678912")
//                        .withRazonSocial("Softgreen S.A.C.")
//                        .build()
//                )
//                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
//                        .withNombre("Carlos Feria")
//                        .withNumeroDocumentoIdentidad("12121212121")
//                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
//                        .build()
//                )
//                .withDetalle(Arrays.asList(
//                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
//                                .withDescripcion("Item1")
//                                .withCantidad(new BigDecimal(10))
//                                .withPrecioConIgv(new BigDecimal(100))
//                                .withTipoIgv(Catalog7.EXPORTACION.toString())
//                                .build(),
//                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
//                                .withDescripcion("Item2")
//                                .withCantidad(new BigDecimal(10))
//                                .withPrecioConIgv(new BigDecimal(100))
//                                .withTipoIgv(Catalog7.EXPORTACION.toString())
//                                .build())
//                )
//                .build();
//
//        // When
//        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
//        String xml = DocumentFacade.createXML(input, config, systemClock);
//
//        // Then
//        assertOutputHasNoConstraintViolations(validator, output);
//        assertSnapshot(xml, "xml/invoice/tipoigv/exportacion.xml");
//        assertSendSunat(xml);
//    }
}

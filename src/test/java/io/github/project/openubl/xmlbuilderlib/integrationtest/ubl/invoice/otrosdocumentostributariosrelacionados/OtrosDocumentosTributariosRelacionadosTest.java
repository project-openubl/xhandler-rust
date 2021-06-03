package io.github.project.openubl.xmlbuilderlib.integrationtest.ubl.invoice.otrosdocumentostributariosrelacionados;

import io.github.project.openubl.xmlbuilderlib.facade.DocumentManager;
import io.github.project.openubl.xmlbuilderlib.facade.DocumentWrapper;
import io.github.project.openubl.xmlbuilderlib.integrationtest.AbstractUBLTest;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog12_Doc_Trib_Relacionado_Invoice;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice.DocTribRelacionadoInputModel_Invoice;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice.InvoiceInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice.InvoiceOutputModel;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.util.Arrays;

public class OtrosDocumentosTributariosRelacionadosTest extends AbstractUBLTest {

    public OtrosDocumentosTributariosRelacionadosTest() throws Exception {
    }

    @Test
    void testTicketDeSalida() throws Exception {
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
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build())
                )
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.TICKET_DE_SALIDA.toString())
                                .build(),
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.TICKET_DE_SALIDA.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/otrosdocumentostributariosrelacionados/ticketDeSalida.xml");
        assertSendSunat(xml);
    }

    @Test
    void testCodigoScop() throws Exception {
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
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build())
                )
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.CODIGO_SCOP.toString())
                                .build(),
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.CODIGO_SCOP.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/otrosdocumentostributariosrelacionados/codigoScop.xml");
        assertSendSunat(xml);
    }

    @Test
    void testFacturaElectronicaRemitente() throws Exception {
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
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build())
                )
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.FACTURA_ELECTRONICA_REMITENTE.toString())
                                .build(),
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.FACTURA_ELECTRONICA_REMITENTE.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/otrosdocumentostributariosrelacionados/facturaElectronicaRemitente.xml");
        assertSendSunat(xml);
    }

    @Test
    void testGuiaRemisionRemitente() throws Exception {
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
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build())
                )
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.GUIA_DE_REMISION_REMITENTE.toString())
                                .build(),
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.GUIA_DE_REMISION_REMITENTE.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/otrosdocumentostributariosrelacionados/guiaRemisionRemitente.xml");
        assertSendSunat(xml);
    }

    @Test
    void testDeclaracionDeSalidaDelDepositoFranco() throws Exception {
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
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build())
                )
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.DECLARACION_DE_SALIDA_DEL_DEPOSITO_FRANCO.toString())
                                .build(),
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.DECLARACION_DE_SALIDA_DEL_DEPOSITO_FRANCO.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/otrosdocumentostributariosrelacionados/declaracionDeSalidaDelDepositoFranco.xml");
        assertSendSunat(xml);
    }

    @Test
    void testDeclaracionSimplificadaDeImportacion() throws Exception {
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
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build())
                )
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.DECLARACION_SIMPLIFICADA_DE_IMPORTACION.toString())
                                .build(),
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.DECLARACION_SIMPLIFICADA_DE_IMPORTACION.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/otrosdocumentostributariosrelacionados/declaracionSimplificadaDeImportacion.xml");
        assertSendSunat(xml);
    }

    @Test
    void testOtros() throws Exception {
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
                                .build(),
                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
                                .withDescripcion("Item2")
                                .withCantidad(new BigDecimal(10))
                                .withPrecioUnitario(new BigDecimal(100))
                                .build())
                )
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.OTROS.toString())
                                .build(),
                        DocTribRelacionadoInputModel_Invoice.Builder.aDocTribRelacionadoInputModel_Invoice()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_Invoice.OTROS.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/otrosdocumentostributariosrelacionados/otros.xml");
        assertSendSunat(xml);
    }

}

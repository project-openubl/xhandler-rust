package io.github.project.openubl.xmlbuilderlib.integrationtest.ubl.creditnote.otrosdocumentostributariosrelacionados;

import io.github.project.openubl.xmlbuilderlib.facade.DocumentManager;
import io.github.project.openubl.xmlbuilderlib.facade.DocumentWrapper;
import io.github.project.openubl.xmlbuilderlib.integrationtest.AbstractUBLTest;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog12_Doc_Trib_Relacionado_CreditNote;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.creditNote.CreditNoteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.creditNote.DocTribRelacionadoInputModel_CreditNote;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.creditNote.CreditNoteOutputModel;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.util.Arrays;

public class OtrosDocumentosTributariosRelacionadosTest extends AbstractUBLTest {

    public OtrosDocumentosTributariosRelacionadosTest() throws Exception {
    }

    @Test
    void testFacturaEmitidaParaCorregirErrorEnElRuc() throws Exception {
        // Given
        CreditNoteInputModel input = CreditNoteInputModel.Builder.aCreditNoteInputModel()
                .withSerie("FC01")
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
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_CreditNote.Builder.aDocTribRelacionadoInputModel_CreditNote()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_CreditNote.FACTURA_EMITIDA_PARA_CORREGIR_ERROR_EN_EL_RUC.toString())
                                .build(),
                        DocTribRelacionadoInputModel_CreditNote.Builder.aDocTribRelacionadoInputModel_CreditNote()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_CreditNote.FACTURA_EMITIDA_PARA_CORREGIR_ERROR_EN_EL_RUC.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<CreditNoteOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        CreditNoteOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/creditnote/otrosdocumentostributariosrelacionados/facturaEmitidaParaCorregirErrorEnElRuc.xml");
        assertSendSunat(xml);
    }

    @Test
    void testTicketDeSalida() throws Exception {
        // Given
        CreditNoteInputModel input = CreditNoteInputModel.Builder.aCreditNoteInputModel()
                .withSerie("FC01")
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
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_CreditNote.Builder.aDocTribRelacionadoInputModel_CreditNote()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_CreditNote.TICKET_DE_SALIDA.toString())
                                .build(),
                        DocTribRelacionadoInputModel_CreditNote.Builder.aDocTribRelacionadoInputModel_CreditNote()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_CreditNote.TICKET_DE_SALIDA.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<CreditNoteOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        CreditNoteOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/creditnote/otrosdocumentostributariosrelacionados/ticketDeSalida.xml");
        assertSendSunat(xml);
    }

    @Test
    void testCodigoScop() throws Exception {
        // Given
        CreditNoteInputModel input = CreditNoteInputModel.Builder.aCreditNoteInputModel()
                .withSerie("FC01")
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
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_CreditNote.Builder.aDocTribRelacionadoInputModel_CreditNote()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_CreditNote.CODIGO_SCOP.toString())
                                .build(),
                        DocTribRelacionadoInputModel_CreditNote.Builder.aDocTribRelacionadoInputModel_CreditNote()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_CreditNote.CODIGO_SCOP.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<CreditNoteOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        CreditNoteOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/creditnote/otrosdocumentostributariosrelacionados/codigoScop.xml");
        assertSendSunat(xml);
    }

    @Test
    void testOtros() throws Exception {
        // Given
        CreditNoteInputModel input = CreditNoteInputModel.Builder.aCreditNoteInputModel()
                .withSerie("FC01")
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
                .withOtrosDocumentosTributariosRelacionados(Arrays.asList(
                        DocTribRelacionadoInputModel_CreditNote.Builder.aDocTribRelacionadoInputModel_CreditNote()
                                .withSerieNumero("F001-1")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_CreditNote.OTROS.toString())
                                .build(),
                        DocTribRelacionadoInputModel_CreditNote.Builder.aDocTribRelacionadoInputModel_CreditNote()
                                .withSerieNumero("F002-2")
                                .withTipoDocumento(Catalog12_Doc_Trib_Relacionado_CreditNote.OTROS.toString())
                                .build()
                ))
                .build();

        // When
        DocumentWrapper<CreditNoteOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        CreditNoteOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/creditnote/otrosdocumentostributariosrelacionados/otros.xml");
        assertSendSunat(xml);
    }
}

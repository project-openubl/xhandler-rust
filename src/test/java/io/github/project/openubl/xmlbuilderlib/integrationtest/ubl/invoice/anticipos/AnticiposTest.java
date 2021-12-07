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
package io.github.project.openubl.xmlbuilderlib.integrationtest.ubl.invoice.anticipos;

import io.github.project.openubl.xmlbuilderlib.facade.DocumentManager;
import io.github.project.openubl.xmlbuilderlib.facade.DocumentWrapper;
import io.github.project.openubl.xmlbuilderlib.integrationtest.AbstractUBLTest;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog12_Anticipo;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice.AnticipoInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice.InvoiceInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice.InvoiceOutputModel;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.util.Arrays;

public class AnticiposTest extends AbstractUBLTest {

    public AnticiposTest() throws Exception {
    }

    @Test
    void testFacturaEmitidaPorAnticipos() throws Exception {
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
                .withAnticipos(Arrays.asList(
                        AnticipoInputModel.Builder.anAnticipoInputModel()
                                .withSerieNumero("F999-1")
                                .withTipoDocumento(Catalog12_Anticipo.FACTURA_EMITIDA_POR_ANTICIPOS.toString())
                                .withMontoTotal(new BigDecimal("180"))
                                .build(),
                        AnticipoInputModel.Builder.anAnticipoInputModel()
                                .withSerieNumero("F999-2")
                                .withTipoDocumento(Catalog12_Anticipo.FACTURA_EMITIDA_POR_ANTICIPOS.toString())
                                .withMontoTotal(new BigDecimal("180"))
                                .build()
                ))
                .build();


        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/anticipos/facturaEmitidaPorAnticipos.xml");
        assertSendSunat(xml);
    }

    @Test
    void testBoletaEmitidaPorAnticipos() throws Exception {
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
                .withAnticipos(Arrays.asList(
                        AnticipoInputModel.Builder.anAnticipoInputModel()
                                .withSerieNumero("B999-1")
                                .withTipoDocumento(Catalog12_Anticipo.BOLETA_DE_VENTA_EMITIDA_POR_ANTICIPOS.toString())
                                .withMontoTotal(new BigDecimal("180"))
                                .build(),
                        AnticipoInputModel.Builder.anAnticipoInputModel()
                                .withSerieNumero("B999-2")
                                .withTipoDocumento(Catalog12_Anticipo.BOLETA_DE_VENTA_EMITIDA_POR_ANTICIPOS.toString())
                                .withMontoTotal(new BigDecimal("180"))
                                .build()
                ))
                .build();


        // When
        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
        InvoiceOutputModel output = result.getOutput();
        String xml = result.getXml();

        // Then
        assertOutputHasNoConstraintViolations(validator, output);
        assertSnapshot(xml, "xml/invoice/anticipos/boletaEmitidaPorAnticipos.xml");
        assertSendSunat(xml);
    }

//    @Test
//    void testFacturaEmitidaPorAnticipos_otros() throws Exception {
//        // Given
//        InvoiceInputModel input = input = InvoiceInputModel.Builder.anInvoiceInputModel()
//                .withSerie("F001")
//                .withNumero(1)
//                .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
//                        .withRuc("20100066603")
//                        .withRazonSocial("Softgreen S.A.C.")
//                        .withDireccion(DireccionInputModel.Builder.aDireccionInputModel()
//                                .withUbigeo("150134")
//                                .withDepartamento("LIMA")
//                                .withProvincia("LIMA")
//                                .withDistrito("LA VICTORIA")
//                                .withDireccion("JR. MARISCAL AGUSTIN GAMARRA NRO. 1160 INT. 603 URB. SAN GERMAN")
//                                .withCodigoPais("PE")
//                                .build())
//                        .build()
//                )
//                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
//                        .withNombre("INVERSIONES GOSMEL S.A.C.")
//                        .withNumeroDocumentoIdentidad("20603348991")
//                        .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
//                        .build()
//                )
//
//                .withDetalle(Arrays.asList(
//                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
//                                .withDescripcion("100% TELA POLYESTER FABRIC BJ2206")
//                                .withCantidad(new BigDecimal(10))
//                                .withPrecioUnitario(new BigDecimal(100))
//                                .build(),
//                        DocumentLineInputModel.Builder.aDocumentLineInputModel()
//                                .withDescripcion("MEDIAS POLYESTER")
//                                .withCantidad(new BigDecimal(10))
//                                .withPrecioUnitario(new BigDecimal(100))
//                                .build())
//                )
//                .withAnticipos(Arrays.asList(
//                                AnticipoInputModel.Builder.anAnticipoInputModel()
//                                        .withSerieNumero("F999-1")
//                                        .withTipoDocumento(Catalog12_Anticipo.FACTURA_EMITIDA_POR_ANTICIPOS.toString())
//                                        .withMontoTotal(new BigDecimal(2360))
//                                        .build()
//                        )
//                )
//                .build();
//
//
//        // When
//        DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);
//        InvoiceOutputModel output = result.getOutput();
//        String xml = result.getXml();
//
//        // Then
//        assertOutputHasNoConstraintViolations(validator, output);
//        assertSnapshot(xml, "xml/invoice/anticipos/facturaEmitidaPorAnticipos_otros.xml");
//        assertSendSunat(xml);
//    }
}

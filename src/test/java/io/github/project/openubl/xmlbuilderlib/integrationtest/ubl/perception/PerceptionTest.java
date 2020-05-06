/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 * <p>
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * <p>
 * https://www.eclipse.org/legal/epl-2.0/
 * <p>
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xmlbuilderlib.integrationtest.ubl.perception;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog22;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog6;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.PerceptionInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.PerceptionRetentionComprobanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.PerceptionRetentionLineInputModel;
import org.junit.jupiter.api.Test;
import io.github.project.openubl.xmlbuilderlib.integrationtest.AbstractUBLTest;

import java.math.BigDecimal;
import java.util.Arrays;
import java.util.Calendar;


public class PerceptionTest extends AbstractUBLTest {

    public PerceptionTest() throws Exception {
    }

    @Test
    void testPerception_minData() throws Exception {
        // Given
        Calendar calendar = Calendar.getInstance();
        calendar.set(2019, Calendar.DECEMBER, 1, 20, 30, 59);

        PerceptionInputModel input = PerceptionInputModel.Builder.aPerceptionInputModel()
                .withSerie("P001")
                .withNumero(1)
                .withRegimen(Catalog22.VENTA_INTERNA.toString())
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
                        PerceptionRetentionLineInputModel.Builder.aPerceptionRetentionLineInputModel()
                                .withComprobante(PerceptionRetentionComprobanteInputModel.Builder.aPerceptionRetentionComprobanteInputModel()
                                        .withMoneda("PEN")
                                        .withTipo(Catalog1.FACTURA.toString())
                                        .withSerieNumero("F001-1")
                                        .withFechaEmision(calendar.getTimeInMillis())
                                        .withImporteTotal(new BigDecimal("100"))
                                        .build()
                                )
                                .build(),
                        PerceptionRetentionLineInputModel.Builder.aPerceptionRetentionLineInputModel()
                                .withComprobante(PerceptionRetentionComprobanteInputModel.Builder.aPerceptionRetentionComprobanteInputModel()
                                        .withMoneda("PEN")
                                        .withTipo(Catalog1.FACTURA.toString())
                                        .withSerieNumero("F001-1")
                                        .withFechaEmision(calendar.getTimeInMillis())
                                        .withImporteTotal(new BigDecimal("100"))
                                        .build()
                                )
                                .build()
                ))
                .build();



        // When
//        PerceptionOutputModel output = InputToOutput.toOutput(input, config, systemClock);
//        String xml = DocumentFacade.createXML(input, config, systemClock);
//
//        // Then
//        assertOutputHasNoConstraintViolations(validator, output);
//        assertSnapshot(xml, "xml/perception/perception_min.xml");
//        assertSendSunat(xml);
    }

}

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
package e2e.homologacion;

import e2e.AbstractTest;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog7;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;

public class Group2InafectoTest extends AbstractTest {

    @Order(1)
    @Test
    public void factura1Con1Items() throws Exception {
        Invoice input = Invoice.builder()
                .serie("FF12")
                .numero(1)
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .build();

        assertInput(input, "factura1Con1Items.xml");
    }

    @Order(2)
    @Test
    public void factura2Con4Items() throws Exception {
        Invoice input = Invoice.builder()
                .serie("FF12")
                .numero(2)
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item2")
                        .cantidad(new BigDecimal("2"))
                        .precio(new BigDecimal("200"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item3")
                        .cantidad(new BigDecimal("3"))
                        .precio(new BigDecimal("300"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item4")
                        .cantidad(new BigDecimal("4"))
                        .precio(new BigDecimal("400"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.getCode())
                        .build()
                )
                .build();

        assertInput(input, "factura2Con4Items.xml");
    }

    @Order(3)
    @Test
    public void factura3Con7Items() throws Exception {
        Invoice input = Invoice.builder()
                .serie("FF12")
                .numero(3)
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item2")
                        .cantidad(new BigDecimal("2"))
                        .precio(new BigDecimal("200"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item3")
                        .cantidad(new BigDecimal("3"))
                        .precio(new BigDecimal("300"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item4")
                        .cantidad(new BigDecimal("4"))
                        .precio(new BigDecimal("400"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item5")
                        .cantidad(new BigDecimal("5"))
                        .precio(new BigDecimal("500"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item6")
                        .cantidad(new BigDecimal("6"))
                        .precio(new BigDecimal("600"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_PREMIO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item7")
                        .cantidad(new BigDecimal("7"))
                        .precio(new BigDecimal("700"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_PUBLICIDAD.getCode())
                        .build()
                )
                .build();

        assertInput(input, "factura3Con7Items.xml");
    }

    @Order(4)
    @Test
    public void factura4Con5Items() throws Exception {
        Invoice input = Invoice.builder()
                .serie("FF12")
                .numero(4)
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item2")
                        .cantidad(new BigDecimal("2"))
                        .precio(new BigDecimal("200"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item3")
                        .cantidad(new BigDecimal("3"))
                        .precio(new BigDecimal("300"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item4")
                        .cantidad(new BigDecimal("4"))
                        .precio(new BigDecimal("400"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item5")
                        .cantidad(new BigDecimal("5"))
                        .precio(new BigDecimal("500"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.getCode())
                        .build()
                )
                .build();

        assertInput(input, "factura4Con5Items.xml");
    }

    @Order(5)
    @Test
    public void factura5Con6Items() throws Exception {
        Invoice input = Invoice.builder()
                .serie("FF12")
                .numero(5)
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item2")
                        .cantidad(new BigDecimal("2"))
                        .precio(new BigDecimal("200"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item3")
                        .cantidad(new BigDecimal("3"))
                        .precio(new BigDecimal("300"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item4")
                        .cantidad(new BigDecimal("4"))
                        .precio(new BigDecimal("400"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item5")
                        .cantidad(new BigDecimal("5"))
                        .precio(new BigDecimal("500"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item6")
                        .cantidad(new BigDecimal("6"))
                        .precio(new BigDecimal("600"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_PREMIO.getCode())
                        .build()
                )
                .build();

        assertInput(input, "factura5Con6Items.xml");
    }

    @Order(6)
    @Test
    public void notaDeCreditoDeFactura1() throws Exception {
        CreditNote input = CreditNote.builder()
                .serie("FF12")
                .numero(1)
                .comprobanteAfectadoSerieNumero("FF12-1")
                .sustentoDescripcion("Homologacion")
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .build();

        assertInput(input, "notaDeCreditoDeFactura1.xml");
    }

    @Order(7)
    @Test
    public void notaDeCreditoDeFactura3() throws Exception {
        CreditNote input = CreditNote.builder()
                .serie("FF12")
                .numero(2)
                .comprobanteAfectadoSerieNumero("FF12-3")
                .sustentoDescripcion("Homologacion")
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item2")
                        .cantidad(new BigDecimal("2"))
                        .precio(new BigDecimal("200"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item3")
                        .cantidad(new BigDecimal("3"))
                        .precio(new BigDecimal("300"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item4")
                        .cantidad(new BigDecimal("4"))
                        .precio(new BigDecimal("400"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item5")
                        .cantidad(new BigDecimal("5"))
                        .precio(new BigDecimal("500"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item6")
                        .cantidad(new BigDecimal("6"))
                        .precio(new BigDecimal("600"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_PREMIO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item7")
                        .cantidad(new BigDecimal("7"))
                        .precio(new BigDecimal("700"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_PUBLICIDAD.getCode())
                        .build()
                )
                .build();

        assertInput(input, "notaDeCreditoDeFactura3.xml");
    }

    @Order(8)
    @Test
    public void notaDeCreditoDeFactura5() throws Exception {
        CreditNote input = CreditNote.builder()
                .serie("FF12")
                .numero(3)
                .comprobanteAfectadoSerieNumero("FF12-5")
                .sustentoDescripcion("Homologacion")
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item2")
                        .cantidad(new BigDecimal("2"))
                        .precio(new BigDecimal("200"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item3")
                        .cantidad(new BigDecimal("3"))
                        .precio(new BigDecimal("300"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item4")
                        .cantidad(new BigDecimal("4"))
                        .precio(new BigDecimal("400"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item5")
                        .cantidad(new BigDecimal("5"))
                        .precio(new BigDecimal("500"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.getCode())
                        .build()
                )
                .build();

        assertInput(input, "notaDeCreditoDeFactura5.xml");
    }

    @Order(9)
    @Test
    public void notaDeDebitoDeFactura1() throws Exception {
        DebitNote input = DebitNote.builder()
                .serie("FF12")
                .numero(1)
                .comprobanteAfectadoSerieNumero("FF12-1")
                .sustentoDescripcion("Homologacion")
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .build();

        assertInput(input, "notaDeDebitoDeFactura1.xml");
    }

    @Order(10)
    @Test
    public void notaDeDebitoDeFactura3() throws Exception {
        DebitNote input = DebitNote.builder()
                .serie("FF12")
                .numero(2)
                .comprobanteAfectadoSerieNumero("FF12-3")
                .sustentoDescripcion("Homologacion")
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item2")
                        .cantidad(new BigDecimal("2"))
                        .precio(new BigDecimal("200"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item3")
                        .cantidad(new BigDecimal("3"))
                        .precio(new BigDecimal("300"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item4")
                        .cantidad(new BigDecimal("4"))
                        .precio(new BigDecimal("400"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item5")
                        .cantidad(new BigDecimal("5"))
                        .precio(new BigDecimal("500"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item6")
                        .cantidad(new BigDecimal("6"))
                        .precio(new BigDecimal("600"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_PREMIO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item7")
                        .cantidad(new BigDecimal("7"))
                        .precio(new BigDecimal("700"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_PUBLICIDAD.getCode())
                        .build()
                )
                .build();

        assertInput(input, "notaDeDebitoDeFactura3.xml");
    }

    @Order(11)
    @Test
    public void notaDeDebitoDeFactura5() throws Exception {
        DebitNote input = DebitNote.builder()
                .serie("FF12")
                .numero(3)
                .comprobanteAfectadoSerieNumero("FF12-5")
                .sustentoDescripcion("Homologacion")
                .proveedor(HomologacionConstants.proveedor)
                .cliente(HomologacionConstants.cliente)
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item1")
                        .cantidad(new BigDecimal("1"))
                        .precio(new BigDecimal("100"))
                        .igvTipo(Catalog7.INAFECTO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item2")
                        .cantidad(new BigDecimal("2"))
                        .precio(new BigDecimal("200"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_BONIFICACION.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item3")
                        .cantidad(new BigDecimal("3"))
                        .precio(new BigDecimal("300"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item4")
                        .cantidad(new BigDecimal("4"))
                        .precio(new BigDecimal("400"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item5")
                        .cantidad(new BigDecimal("5"))
                        .precio(new BigDecimal("500"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO.getCode())
                        .build()
                )
                .detalle(DocumentoVentaDetalle.builder()
                        .descripcion("Item6")
                        .cantidad(new BigDecimal("6"))
                        .precio(new BigDecimal("600"))
                        .igvTipo(Catalog7.INAFECTO_RETIRO_POR_PREMIO.getCode())
                        .build()
                )
                .build();

        assertInput(input, "notaDeDebitoDeFactura5.xml");
    }
}

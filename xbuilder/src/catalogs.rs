use std::fmt;
use std::fmt::{Display, Formatter};

use rust_decimal::Decimal;

/// A trait for providing the 'code' of a catalog
pub trait Catalog {
    fn code(&self) -> &str;
}

/// A trait for getting a catalog from its 'code'
pub trait FromCode: Sized {
    fn from_code(code: &str) -> Result<Self, String>;
}

/// A trait for providing a 'label' representation
pub trait Label {
    fn label(&self) -> &str;
}

/// Declares a SUNAT catalog: the enum, its codes, and a `from_code` that
/// accepts either the code (`"1"`) or the variant name, ignoring case and
/// underscores, as xhandler-java does (`"DNI"`, `"GRAVADO_OPERACION_ONEROSA"`).
macro_rules! catalog {
    ($(#[$meta:meta])* $name:ident { $($variant:ident = $code:literal,)* }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant,)*
        }

        impl Catalog for $name {
            fn code(&self) -> &str {
                match self {
                    $(Self::$variant => $code,)*
                }
            }
        }

        impl FromCode for $name {
            fn from_code(value: &str) -> Result<Self, String> {
                let name = normalize_name(value);
                [$(Self::$variant,)*]
                    .into_iter()
                    .zip([$(stringify!($variant),)*])
                    .find(|(variant, variant_name)| {
                        variant.code() == value || normalize_name(variant_name) == name
                    })
                    .map(|(variant, _)| variant)
                    .ok_or_else(|| format!("{value} is not a valid {}", stringify!($name)))
            }
        }
    };
}

/// The code of a catalog value given by name, e.g. `code_for_name::<Catalog6>("DNI")`
/// is `Some("1")`. `None` when the value is already a code or isn't in the catalog.
pub fn code_for_name<C: Catalog + FromCode>(value: &str) -> Option<String> {
    C::from_code(value)
        .ok()
        .map(|catalog| catalog.code().to_string())
        .filter(|code| code != value)
}

fn normalize_name(value: &str) -> String {
    value.replace('_', "").to_lowercase()
}

catalog! {
    Catalog1 {
        Factura = "01",
        Boleta = "03",
        NotaCredito = "07",
        NotaDebito = "08",
        GuiaRemisionRemitente = "09",
        TicketMaquinaRegistradora = "12",
        DocumentosFinancieros = "13",
        DocumentosAfp = "18",
        Retencion = "20",
        PercepcionVentaInterna = "21",
        GuiaRemisionTransportista = "31",
        Percepcion = "40",
        ComprobantePagoSeae = "56",
        GuiaRemisionRemitenteComplementaria = "71",
        GuiaRemisionTransportistaComplementaria = "72",
    }
}

catalog! {
    Catalog5 {
        Igv = "1000",
        ImpuestoArrozPilado = "1016",
        Isc = "2000",
        Exportacion = "9995",
        Gratuito = "9996",
        Exonerado = "9997",
        Inafecto = "9998",
        IcbPer = "7152",
        Otros = "9999",
    }
}

impl Catalog5 {
    /// Tax category (UN/ECE 5305). SUNAT uses "S" for every tax.
    pub fn categoria(&self) -> &'static str {
        "S"
    }

    pub fn tipo(&self) -> &'static str {
        match self {
            Catalog5::Igv => "VAT",
            Catalog5::ImpuestoArrozPilado => "VAT",
            Catalog5::Isc => "EXC",
            Catalog5::Exportacion => "FRE",
            Catalog5::Gratuito => "FRE",
            Catalog5::Exonerado => "VAT",
            Catalog5::Inafecto => "FRE",
            Catalog5::IcbPer => "OTH",
            Catalog5::Otros => "OTH",
        }
    }

    pub fn nombre(&self) -> &'static str {
        match self {
            Catalog5::Igv => "IGV",
            Catalog5::ImpuestoArrozPilado => "IVAP",
            Catalog5::Isc => "ISC",
            Catalog5::Exportacion => "EXP",
            Catalog5::Gratuito => "GRA",
            Catalog5::Exonerado => "EXO",
            Catalog5::Inafecto => "INA",
            Catalog5::IcbPer => "ICBPER",
            Catalog5::Otros => "OTROS",
        }
    }
}

catalog! {
    Catalog6 {
        DocTribNoDomSinRuc = "0",
        DNI = "1",
        Extranjeria = "4",
        RUC = "6",
        Pasaporte = "7",
        DecDiplomatica = "A",
        DocIdentPaisResidenciaNoDomiciliado = "B",
        TaxIdentificationNumberTinDocTribPpNn = "C",
        IdentificationNumberInDocTribPpJj = "D",
        TamTarjetaAndinaDeMigracion = "E",
        PermisoTemporalDePermanenciaPtp = "F",
        SalvoConducto = "G",
    }
}

catalog! {
    Catalog7 {
        GravadoOperacionOnerosa = "10",
        GravadoRetiroPorPremio = "11",
        GravadoRetiroPorDonacion = "12",
        GravadoRetiro = "13",
        GravadoRetiroPorPublicidad = "14",
        GravadoBonificaciones = "15",
        GravadoRetiroPorEntregaATrabajadores = "16",
        GravadoIvap = "17",
        ExoneradoOperacionOnerosa = "20",
        ExoneradoTransferenciaGratuita = "21",
        InafectoOperacionOnerosa = "30",
        InafectoRetiroPorBonificacion = "31",
        InafectoRetiro = "32",
        InafectoRetiroPorMuestrasMedicas = "33",
        InafectoRetiroPorConvenioColectivo = "34",
        InafectoRetiroPorPremio = "35",
        InafectoRetiroPorPublicidad = "36",
        Exportacion = "40",
    }
}

catalog! {
    Catalog7Group {
        Gravado = "01",
        Exonerado = "02",
        Inafecto = "03",
        Exportacion = "04",
        Gratuita = "05",
    }
}

impl Catalog7 {
    pub fn group(&self) -> Catalog7Group {
        match &self {
            Catalog7::GravadoOperacionOnerosa => Catalog7Group::Gravado,
            Catalog7::GravadoRetiroPorPremio => Catalog7Group::Gravado,
            Catalog7::GravadoRetiroPorDonacion => Catalog7Group::Gravado,
            Catalog7::GravadoRetiro => Catalog7Group::Gravado,
            Catalog7::GravadoRetiroPorPublicidad => Catalog7Group::Gravado,
            Catalog7::GravadoBonificaciones => Catalog7Group::Gravado,
            Catalog7::GravadoRetiroPorEntregaATrabajadores => Catalog7Group::Gravado,
            Catalog7::GravadoIvap => Catalog7Group::Gravado,

            Catalog7::ExoneradoOperacionOnerosa => Catalog7Group::Exonerado,
            Catalog7::ExoneradoTransferenciaGratuita => Catalog7Group::Exonerado,

            Catalog7::InafectoOperacionOnerosa => Catalog7Group::Inafecto,
            Catalog7::InafectoRetiroPorBonificacion => Catalog7Group::Inafecto,
            Catalog7::InafectoRetiro => Catalog7Group::Inafecto,
            Catalog7::InafectoRetiroPorMuestrasMedicas => Catalog7Group::Inafecto,
            Catalog7::InafectoRetiroPorConvenioColectivo => Catalog7Group::Inafecto,
            Catalog7::InafectoRetiroPorPremio => Catalog7Group::Inafecto,
            Catalog7::InafectoRetiroPorPublicidad => Catalog7Group::Inafecto,

            Catalog7::Exportacion => Catalog7Group::Exportacion,
        }
    }

    pub fn onerosa(&self) -> bool {
        match &self {
            Catalog7::GravadoOperacionOnerosa => true,
            Catalog7::GravadoRetiroPorPremio => false,
            Catalog7::GravadoRetiroPorDonacion => false,
            Catalog7::GravadoRetiro => false,
            Catalog7::GravadoRetiroPorPublicidad => false,
            Catalog7::GravadoBonificaciones => false,
            Catalog7::GravadoRetiroPorEntregaATrabajadores => false,
            Catalog7::GravadoIvap => true,

            Catalog7::ExoneradoOperacionOnerosa => true,
            Catalog7::ExoneradoTransferenciaGratuita => false,

            Catalog7::InafectoOperacionOnerosa => true,
            Catalog7::InafectoRetiroPorBonificacion => false,
            Catalog7::InafectoRetiro => false,
            Catalog7::InafectoRetiroPorMuestrasMedicas => false,
            Catalog7::InafectoRetiroPorConvenioColectivo => false,
            Catalog7::InafectoRetiroPorPremio => false,
            Catalog7::InafectoRetiroPorPublicidad => false,

            Catalog7::Exportacion => true,
        }
    }

    pub fn tax_category(&self) -> Catalog5 {
        match &self {
            Catalog7::GravadoOperacionOnerosa => Catalog5::Igv,
            Catalog7::GravadoRetiroPorPremio => Catalog5::Gratuito,
            Catalog7::GravadoRetiroPorDonacion => Catalog5::Gratuito,
            Catalog7::GravadoRetiro => Catalog5::Gratuito,
            Catalog7::GravadoRetiroPorPublicidad => Catalog5::Gratuito,
            Catalog7::GravadoBonificaciones => Catalog5::Gratuito,
            Catalog7::GravadoRetiroPorEntregaATrabajadores => Catalog5::Gratuito,
            Catalog7::GravadoIvap => Catalog5::ImpuestoArrozPilado,

            Catalog7::ExoneradoOperacionOnerosa => Catalog5::Exonerado,
            Catalog7::ExoneradoTransferenciaGratuita => Catalog5::Gratuito,

            Catalog7::InafectoOperacionOnerosa => Catalog5::Inafecto,
            Catalog7::InafectoRetiroPorBonificacion => Catalog5::Gratuito,
            Catalog7::InafectoRetiro => Catalog5::Gratuito,
            Catalog7::InafectoRetiroPorMuestrasMedicas => Catalog5::Gratuito,
            Catalog7::InafectoRetiroPorConvenioColectivo => Catalog5::Gratuito,
            Catalog7::InafectoRetiroPorPremio => Catalog5::Gratuito,
            Catalog7::InafectoRetiroPorPublicidad => Catalog5::Gratuito,

            Catalog7::Exportacion => Catalog5::Exportacion,
        }
    }
}

impl Display for Catalog7 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Self::GravadoOperacionOnerosa => write!(f, "GRAVADO_OPERACION_ONEROSA"),
            Self::GravadoRetiroPorPremio => write!(f, "GRAVADO_RETIRO_POR_PREMIO"),
            Self::GravadoRetiroPorDonacion => write!(f, "GRAVADO_RETIRO_POR_DONACION"),
            Self::GravadoRetiro => write!(f, "GRAVADO_RETIRO"),
            Self::GravadoRetiroPorPublicidad => write!(f, "GRAVADO_RETIRO_POR_PUBLICIDAD"),
            Self::GravadoBonificaciones => write!(f, "GRAVADO_BONIFICACIONES"),
            Self::GravadoRetiroPorEntregaATrabajadores => {
                write!(f, "GRAVADO_RETIRO_POR_ENTREGA_A_TRABAJADORES")
            }
            Self::GravadoIvap => write!(f, "GRAVADO_IVAP"),

            Self::ExoneradoOperacionOnerosa => write!(f, "EXONERADO_OPERACION_ONEROSA"),
            Self::ExoneradoTransferenciaGratuita => write!(f, "EXONERADO_TRANSFERENCIA_GRATUITA"),

            Self::InafectoOperacionOnerosa => write!(f, "INAFECTO_OPERACION_ONEROSA"),
            Self::InafectoRetiroPorBonificacion => write!(f, "INAFECTO_RETIRO_POR_BONIFICACION"),
            Self::InafectoRetiro => write!(f, "INAFECTO_RETIRO"),
            Self::InafectoRetiroPorMuestrasMedicas => {
                write!(f, "INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS")
            }
            Self::InafectoRetiroPorConvenioColectivo => {
                write!(f, "INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO")
            }
            Self::InafectoRetiroPorPremio => write!(f, "INAFECTO_RETIRO_POR_PREMIO"),
            Self::InafectoRetiroPorPublicidad => write!(f, "INAFECTO_RETIRO_POR_PUBLICIDAD"),

            Self::Exportacion => write!(f, "EXPORTACION"),
        }
    }
}

catalog! {
    Catalog8 {
        SistemaAlValor = "01",
        AplicacionAlMontoFijo = "02",
        SistemaDePreciosDeVentaAlPublico = "03",
    }
}

catalog! {
    Catalog9 {
        AnulacionDeLaOperacion = "01",
        AnulacionPorErrorEnElRuc = "02",
        CorreccionPorErrorEnLaDescripcion = "03",
        DescuentoGlobal = "04",
        DescuentoPorItem = "05",
        DevolucionTotal = "06",
        DevolucionPorItem = "07",
        Bonificacion = "08",
        DisminucionEnElValor = "09",
        OtrosConceptos = "10",
    }
}

catalog! {
    Catalog10 {
        InteresPorMora = "01",
        AumentoEnElValor = "02",
        PenalidadOtrosConceptos = "03",
    }
}

catalog! {
    Catalog12 {
        FacturaEmitidaParaCorregirErrorEnElRuc = "01",
        FacturaEmitidaPorAnticipos = "02",
        BoletaDeVentaEmitidaPorAnticipos = "03",
        TicketDeSalida = "04",
        CodigoScop = "05",
        FacturaElectronicaRemitente = "06",
        GuiaDeRemisionRemitente = "07",
        DeclaracionDeSalidaDelDepositoFranco = "08",
        DeclaracionSimplificadaDeImportacion = "09",
        LiquidacionDeCompraEmitidaPorAnticipos = "10",
        Otros = "99",
    }
}

catalog! {
    Catalog16 {
        PrecioUnitarioIncluyeIgv = "01",
        ValorReferencialUnitarioEnOperacionesNoOnerosas = "02",
    }
}

catalog! {
    /// Modalidad de traslado
    Catalog18 {
        TransportePublico = "01",
        TransportePrivado = "02",
    }
}

catalog! {
    /// Estado del ítem en un resumen diario
    Catalog19 {
        Adicionar = "1",
        Modificar = "2",
        Anulado = "3",
    }
}

catalog! {
    /// Motivo de traslado
    Catalog20 {
        Venta = "01",
        Compra = "02",
        TrasladoEntreEstablecimientosDeLaMismaEmpresa = "04",
        Importacion = "08",
        Exportacion = "09",
        Otros = "13",
        VentaSujetaAConfirmacionDelComprador = "14",
        TrasladoEmisorItineranteCp = "18",
        TrasladoAZonaPrimaria = "19",
    }
}

catalog! {
    /// Documentos relacionados al transporte de bienes
    Catalog21 {
        NumeracionDam = "01",
        NumeroDeOrdenDeEntrega = "02",
        NumeroScop = "03",
        NumeroDeManifiestoDeCarga = "04",
        NumeroDeConstanciaDeDetraccion = "05",
        Otros = "06",
    }
}

catalog! {
    /// Régimen de percepción
    Catalog22 {
        VentaInterna = "01",
        AdquisicionDeCombustible = "02",
        AgenteDePercepcionConTasaEspecial = "03",
    }
}

impl Catalog22 {
    /// Percentage of the regime, e.g. 2 for 2%
    pub fn percent(&self) -> Decimal {
        match self {
            Self::VentaInterna => Decimal::TWO,
            Self::AdquisicionDeCombustible => Decimal::ONE,
            Self::AgenteDePercepcionConTasaEspecial => Decimal::new(5, 1),
        }
    }
}

catalog! {
    /// Régimen de retención
    Catalog23 {
        TasaTres = "01",
    }
}

impl Catalog23 {
    /// Percentage of the regime, e.g. 3 for 3%
    pub fn percent(&self) -> Decimal {
        match self {
            Self::TasaTres => Decimal::from(3),
        }
    }
}

catalog! {
    Catalog51 {
        VentaInterna = "0101",
        OperacionSujetaADetraccion = "1001",
        OperacionSujetaAPercepcion = "2001",
    }
}

catalog! {
    Catalog52 {
        MontoEnLetras = "1000",
        ComprobanteDePercepcion = "2000",
        VentaRealizadaPorEmisorItinerante = "2005",
        OperacionSujetaADetraccion = "2006",
    }
}

impl Label for Catalog52 {
    fn label(&self) -> &str {
        match &self {
            Self::MontoEnLetras => "MONTO EN LETRAS",
            Self::ComprobanteDePercepcion => "COMPROBANTE DE PERCEPCION",
            Self::VentaRealizadaPorEmisorItinerante => "VENTA REALIZADA POR EMISOR ITINERANTE",
            Self::OperacionSujetaADetraccion => "OPERACION SUJETA A DETRACCION",
        }
    }
}

catalog! {
    Catalog53 {
        DescuentoAfectaBaseImponibleIgvIvap = "00",
        DescuentoNoAfectaBaseImponibleIgvIvap = "01",
        DescuentoGlobalAfectaBaseImponibleIgvIvap = "02",
        DescuentoGlobalNoAfectaBaseImponibleIgvIvap = "03",
        DescuentoGlobalPorAnticiposGravadosAfectaBaseImponibleIgvIvap = "04",
        DescuentoGlobalPorAnticiposExonerados = "05",
        DescuentoGlobalPorAnticiposInafectos = "06",
        FactorDeCompensacion = "07",
        AnticipoDeIsc = "20",
        FISE = "45",
        RecargoAlConsumoYOPropinas = "46",
        CargosQueAfectanBaseImponibleIgvIvap = "47",
        CargosQueNoAfectanBaseImponibleIgvIvap = "48",
        CargosGlobalesQueAfectanBaseImponibleIgvIvap = "49",
        CargosGlobalesQueNoAfectanBaseImponibleIgvIvap = "50",
        PercepcionVentaInterna = "51",
        PercepcionALaAdquisicionDeCombustible = "52",
        PercepcionRealizadaAlAgenteDePercepcionConTasaEspecial = "53",
        FactorDeAportacion = "54",
        RetencionDeRentaPorAnticipos = "61",
        RetencionDelIgv = "62",
        RetencionDeRentaDeSegundaCategoria = "63",
    }
}

catalog! {
    Catalog54 {
        Azucar = "001",
        AlcoholEtilico = "003",
        RecursosHidrobiologicos = "004",
    }
}

catalog! {
    Catalog59 {
        DepositoEnCuenta = "001",
        Giro = "002",
        TransferenciaDeFondos = "003",
        OrdenDePago = "004",
        TarjetaDeDebito = "005",
        TarjetaDeCreditoEmitidaEnElPaisPorUnaEmpresaDelSistemaFinanciero = "006",
        ChequesConLaClausulaDeNonegociableIntransferibleNoalaordenUOtraEquivalente = "007",
        EfectivoPorOperacionesEnLasQueNoExisteObligacionDeUtilizarMedioDePago = "008",
        EfectivoEnLosDemasCasos = "009",
        MediosDePagoUsadosEnComercioExterior = "010",
        DocumentosEmitidosPorLasEdpymesYLasCooperativas = "011",
        TarjetaDeCreditoEmitidaEnElPaisOEnExteriorEmitidaPorEmpresaNoPertenecienteAlSistemaFinanciero = "012",
        TarjetasDeCreditoEmitidasEnElExteriorPorEmpresasBancariasOFinancierasNoDomiciliadas = "013",
        TransferenciasComercioExterior = "101",
        ChequesBancariosComercioExterior = "102",
        OrdenDePagoSimpleComercioExterior = "103",
        OrdenDePagoDocumentarioComercioExterior = "104",
        RemesaSimpleComercioExterior = "105",
        RemesaDocumentariaComercioExterior = "106",
        CartaDeCreditoSimpleComercioExterior = "107",
        CartaDeCreditoDocumentarioComercioExterior = "108",
        OtrosMediosDePago = "999",
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn from_code_accepts_code_or_name() {
        assert_eq!(Catalog6::from_code("1"), Ok(Catalog6::DNI));
        assert_eq!(Catalog6::from_code("DNI"), Ok(Catalog6::DNI));
        assert_eq!(Catalog6::from_code("dni"), Ok(Catalog6::DNI));
        assert_eq!(
            Catalog7::from_code("GRAVADO_OPERACION_ONEROSA"),
            Ok(Catalog7::GravadoOperacionOnerosa)
        );
        assert_eq!(Catalog19::from_code("adicionar"), Ok(Catalog19::Adicionar));
        assert_eq!(Catalog1::from_code("40"), Ok(Catalog1::Percepcion));
        assert!(Catalog7::from_code("99").is_err());
        assert!(Catalog7::from_code("").is_err());
    }

    #[test]
    fn code_for_name_only_converts_names() {
        assert_eq!(code_for_name::<Catalog6>("RUC").as_deref(), Some("6"));
        assert_eq!(code_for_name::<Catalog6>("6"), None);
        assert_eq!(code_for_name::<Catalog6>("unknown"), None);
    }

    #[test]
    fn regime_percentages() {
        assert_eq!(Catalog22::VentaInterna.percent(), Decimal::TWO);
        assert_eq!(
            Catalog22::AgenteDePercepcionConTasaEspecial.percent(),
            Decimal::new(5, 1)
        );
        assert_eq!(Catalog23::TasaTres.percent(), Decimal::from(3));
    }
}

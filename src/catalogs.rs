use std::fmt;
use std::fmt::{Display, Formatter};

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

#[derive(Clone, Debug)]
pub enum Catalog1 {
    Factura,
    Boleta,
    NotaCredito,
    NotaDebito,
    GuiaRemisionRemitente,
}

impl Catalog for Catalog1 {
    fn code(&self) -> &str {
        match &self {
            Self::Factura => "01",
            Self::Boleta => "03",
            Self::NotaCredito => "07",
            Self::NotaDebito => "08",
            Self::GuiaRemisionRemitente => "09",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Catalog5 {
    Igv,
    ImpuestoArrozPilado,
    Isc,
    Exportacion,
    Gratuito,
    Exonerado,
    Inafecto,
    IcbPer,
    Otros,
}

impl Catalog for Catalog5 {
    fn code(&self) -> &str {
        match &self {
            Self::Igv => "1000",
            Self::ImpuestoArrozPilado => "1016",
            Self::Isc => "2000",
            Self::Exportacion => "9995",
            Self::Gratuito => "9996",
            Self::Exonerado => "9997",
            Self::Inafecto => "9998",
            Self::IcbPer => "7152",
            Self::Otros => "9999",
        }
    }
}

impl Catalog5 {
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

#[derive(Clone, Debug)]
pub enum Catalog6 {
    DocTribNoDomSinRuc,
    DNI,
    Extranjeria,
    RUC,
    Pasaporte,
    DecDiplomatica,
    DocIdentPaisResidenciaNoDomiciliado,
    TaxIdentificationNumberTinDocTribPpNn,
    IdentificationNumberInDocTribPpJj,
    TamTarjetaAndinaDeMigracion,
    PermisoTemporalDePermanenciaPtp,
    SalvoConducto,
}

impl Catalog for Catalog6 {
    fn code(&self) -> &str {
        match &self {
            Self::DocTribNoDomSinRuc => "0",
            Self::DNI => "1",
            Self::Extranjeria => "4",
            Self::RUC => "6",
            Self::Pasaporte => "7",
            Self::DecDiplomatica => "A",
            Self::DocIdentPaisResidenciaNoDomiciliado => "B",
            Self::TaxIdentificationNumberTinDocTribPpNn => "C",
            Self::IdentificationNumberInDocTribPpJj => "D",
            Self::TamTarjetaAndinaDeMigracion => "E",
            Self::PermisoTemporalDePermanenciaPtp => "F",
            Self::SalvoConducto => "G",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Catalog7 {
    GravadoOperacionOnerosa,
    GravadoRetiroPorPremio,
    GravadoRetiroPorDonacion,
    GravadoRetiro,
    GravadoRetiroPorPublicidad,
    GravadoBonificaciones,
    GravadoRetiroPorEntregaATrabajadores,
    GravadoIvap,

    ExoneradoOperacionOnerosa,
    ExoneradoTransferenciaGratuita,

    InafectoOperacionOnerosa,
    InafectoRetiroPorBonificacion,
    InafectoRetiro,
    InafectoRetiroPorMuestrasMedicas,
    InafectoRetiroPorConvenioColectivo,
    InafectoRetiroPorPremio,
    InafectoRetiroPorPublicidad,

    Exportacion,
}

impl Catalog for Catalog7 {
    fn code(&self) -> &str {
        match &self {
            Self::GravadoOperacionOnerosa => "10",
            Self::GravadoRetiroPorPremio => "11",
            Self::GravadoRetiroPorDonacion => "12",
            Self::GravadoRetiro => "13",
            Self::GravadoRetiroPorPublicidad => "14",
            Self::GravadoBonificaciones => "15",
            Self::GravadoRetiroPorEntregaATrabajadores => "16",
            Self::GravadoIvap => "17",

            Self::ExoneradoOperacionOnerosa => "20",
            Self::ExoneradoTransferenciaGratuita => "21",

            Self::InafectoOperacionOnerosa => "30",
            Self::InafectoRetiroPorBonificacion => "31",
            Self::InafectoRetiro => "32",
            Self::InafectoRetiroPorMuestrasMedicas => "33",
            Self::InafectoRetiroPorConvenioColectivo => "34",
            Self::InafectoRetiroPorPremio => "35",
            Self::InafectoRetiroPorPublicidad => "36",

            Self::Exportacion => "40",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog7Group {
    Gravado,
    Exonerado,
    Inafecto,
    Exportacion,
    Gratuita,
}

impl Catalog for Catalog7Group {
    fn code(&self) -> &str {
        match &self {
            Self::Gravado => "01",
            Self::Exonerado => "02",
            Self::Inafecto => "03",
            Self::Exportacion => "04",
            Self::Gratuita => "05",
        }
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

            Catalog7::Exportacion => Catalog7Group::Exonerado,
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

impl FromCode for Catalog7 {
    fn from_code(code: &str) -> Result<Self, String> {
        match code {
            "10" => Ok(Catalog7::GravadoOperacionOnerosa),
            "11" => Ok(Catalog7::GravadoRetiroPorPremio),
            "12" => Ok(Catalog7::GravadoRetiroPorDonacion),
            "13" => Ok(Catalog7::GravadoRetiro),
            "14" => Ok(Catalog7::GravadoRetiroPorPublicidad),
            "15" => Ok(Catalog7::GravadoBonificaciones),
            "16" => Ok(Catalog7::GravadoRetiroPorEntregaATrabajadores),
            "17" => Ok(Catalog7::GravadoIvap),

            "20" => Ok(Catalog7::ExoneradoOperacionOnerosa),
            "21" => Ok(Catalog7::ExoneradoTransferenciaGratuita),

            "30" => Ok(Catalog7::InafectoOperacionOnerosa),
            "31" => Ok(Catalog7::InafectoRetiroPorBonificacion),
            "32" => Ok(Catalog7::InafectoRetiro),
            "33" => Ok(Catalog7::InafectoRetiroPorMuestrasMedicas),
            "34" => Ok(Catalog7::InafectoRetiroPorConvenioColectivo),
            "35" => Ok(Catalog7::InafectoRetiroPorPremio),
            "36" => Ok(Catalog7::InafectoRetiroPorPublicidad),

            "40" => Ok(Catalog7::Exportacion),

            _ => Err(format!("code {code} did not match")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog8 {
    SistemaAlValor,
    AplicacionAlMontoFijo,
    SistemaDePreciosDeVentaAlPublico,
}

impl Catalog for Catalog8 {
    fn code(&self) -> &str {
        match &self {
            Self::SistemaAlValor => "01",
            Self::AplicacionAlMontoFijo => "02",
            Self::SistemaDePreciosDeVentaAlPublico => "03",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog9 {
    AnulacionDeLaOperacion,
    AnulacionPorErrorEnElRuc,
    CorreccionPorErrorEnLaDescripcion,
    DescuentoGlobal,
    DescuentoPorItem,
    DevolucionTotal,
    DevolucionPorItem,
    Bonificacion,
    DisminucionEnElValor,
    OtrosConceptos,
}

impl Catalog for Catalog9 {
    fn code(&self) -> &str {
        match &self {
            Self::AnulacionDeLaOperacion => "01",
            Self::AnulacionPorErrorEnElRuc => "02",
            Self::CorreccionPorErrorEnLaDescripcion => "03",
            Self::DescuentoGlobal => "04",
            Self::DescuentoPorItem => "05",
            Self::DevolucionTotal => "06",
            Self::DevolucionPorItem => "06",
            Self::Bonificacion => "08",
            Self::DisminucionEnElValor => "09",
            Self::OtrosConceptos => "10",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog10 {
    InteresPorMora,
    AumentoEnElValor,
    PenalidadOtrosConceptos,
}

impl Catalog for Catalog10 {
    fn code(&self) -> &str {
        match &self {
            Self::InteresPorMora => "01",
            Self::AumentoEnElValor => "02",
            Self::PenalidadOtrosConceptos => "03",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog12 {
    FacturaEmitidaParaCorregirErrorEnElRuc,
    FacturaEmitidaPorAnticipos,
    BoletaDeVentaEmitidaPorAnticipos,
    TicketDeSalida,
    CodigoScop,
    FacturaElectronicaRemitente,
    GuiaDeRemisionRemitente,
    DeclaracionDeSalidaDelDepositoFranco,
    DeclaracionSimplificadaDeImportacion,
    LiquidacionDeCompraEmitidaPorAnticipos,
    Otros,
}

impl Catalog for Catalog12 {
    fn code(&self) -> &str {
        match &self {
            Self::FacturaEmitidaParaCorregirErrorEnElRuc => "01",
            Self::FacturaEmitidaPorAnticipos => "02",
            Self::BoletaDeVentaEmitidaPorAnticipos => "03",
            Self::TicketDeSalida => "04",
            Self::CodigoScop => "05",
            Self::FacturaElectronicaRemitente => "06",
            Self::GuiaDeRemisionRemitente => "07",
            Self::DeclaracionDeSalidaDelDepositoFranco => "08",
            Self::DeclaracionSimplificadaDeImportacion => "09",
            Self::LiquidacionDeCompraEmitidaPorAnticipos => "10",
            Self::Otros => "99",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog16 {
    PrecioUnitarioIncluyeIgv,
    ValorReferencialUnitarioEnOperacionesNoOnerosas,
}

impl Catalog for Catalog16 {
    fn code(&self) -> &str {
        match &self {
            Self::PrecioUnitarioIncluyeIgv => "01",
            Self::ValorReferencialUnitarioEnOperacionesNoOnerosas => "02",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog51 {
    VentaInterna,
    OperacionSujetaADetraccion,
    OperacionSujetaAPercepcion,
}

impl Catalog for Catalog51 {
    fn code(&self) -> &str {
        match &self {
            Self::VentaInterna => "0101",
            Self::OperacionSujetaADetraccion => "1001",
            Self::OperacionSujetaAPercepcion => "2001",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog52 {
    MontoEnLetras,
    ComprobanteDePercepcion,
    VentaRealizadaPorEmisorItinerante,
    OperacionSujetaADetraccion,
}

impl Catalog for Catalog52 {
    fn code(&self) -> &str {
        match &self {
            Self::MontoEnLetras => "1000",
            Self::ComprobanteDePercepcion => "2000",
            Self::VentaRealizadaPorEmisorItinerante => "2005",
            Self::OperacionSujetaADetraccion => "2006",
        }
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

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Catalog53 {
    DescuentoAfectaBaseImponibleIgvIvap,
    DescuentoNoAfectaBaseImponibleIgvIvap,
    DescuentoGlobalAfectaBaseImponibleIgvIvap,
    DescuentoGlobalNoAfectaBaseImponibleIgvIvap,
    DescuentoGlobalPorAnticiposGravadosAfectaBaseImponibleIgvIvap,
    DescuentoGlobalPorAnticiposExonerados,
    DescuentoGlobalPorAnticiposInafectos,
    FactorDeCompensacion,
    AnticipoDeIsc,
    FISE,
    RecargoAlConsumoYOPropinas,
    CargosQueAfectanBaseImponibleIgvIvap,
    CargosQueNoAfectanBaseImponibleIgvIvap,
    CargosGlobalesQueAfectanBaseImponibleIgvIvap,
    CargosGlobalesQueNoAfectanBaseImponibleIgvIvap,
    PercepcionVentaInterna,
    PercepcionALaAdquisicionDeCombustible,
    PercepcionRealizadaAlAgenteDePercepcionConTasaEspecial,
    FactorDeAportacion,
    RetencionDeRentaPorAnticipos,
    RetencionDelIgv,
    RetencionDeRentaDeSegundaCategoria,
}

impl Catalog for Catalog53 {
    fn code(&self) -> &str {
        match &self {
            Self::DescuentoAfectaBaseImponibleIgvIvap => "00",
            Self::DescuentoNoAfectaBaseImponibleIgvIvap => "01",
            Self::DescuentoGlobalAfectaBaseImponibleIgvIvap => "02",
            Self::DescuentoGlobalNoAfectaBaseImponibleIgvIvap => "03",
            Self::DescuentoGlobalPorAnticiposGravadosAfectaBaseImponibleIgvIvap => "04",
            Self::DescuentoGlobalPorAnticiposExonerados => "05",
            Self::DescuentoGlobalPorAnticiposInafectos => "06",
            Self::FactorDeCompensacion => "07",
            Self::AnticipoDeIsc => "20",
            Self::FISE => "45",
            Self::RecargoAlConsumoYOPropinas => "46",
            Self::CargosQueAfectanBaseImponibleIgvIvap => "47",
            Self::CargosQueNoAfectanBaseImponibleIgvIvap => "48",
            Self::CargosGlobalesQueAfectanBaseImponibleIgvIvap => "49",
            Self::CargosGlobalesQueNoAfectanBaseImponibleIgvIvap => "50",
            Self::PercepcionVentaInterna => "51",
            Self::PercepcionALaAdquisicionDeCombustible => "52",
            Self::PercepcionRealizadaAlAgenteDePercepcionConTasaEspecial => "53",
            Self::FactorDeAportacion => "61",
            Self::RetencionDeRentaPorAnticipos => "62",
            Self::RetencionDelIgv => "63",
            Self::RetencionDeRentaDeSegundaCategoria => "64",
        }
    }
}

impl FromCode for Catalog53 {
    fn from_code(code: &str) -> Result<Self, String> {
        match code {
            "00" => Ok(Catalog53::DescuentoAfectaBaseImponibleIgvIvap),
            "01" => Ok(Catalog53::DescuentoNoAfectaBaseImponibleIgvIvap),
            "02" => Ok(Catalog53::DescuentoGlobalAfectaBaseImponibleIgvIvap),
            "03" => Ok(Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap),
            "04" => Ok(Catalog53::DescuentoGlobalPorAnticiposGravadosAfectaBaseImponibleIgvIvap),
            "05" => Ok(Catalog53::DescuentoGlobalPorAnticiposExonerados),
            "06" => Ok(Catalog53::DescuentoGlobalPorAnticiposInafectos),
            "07" => Ok(Catalog53::FactorDeCompensacion),
            "20" => Ok(Catalog53::AnticipoDeIsc),
            "45" => Ok(Catalog53::FISE),
            "46" => Ok(Catalog53::RecargoAlConsumoYOPropinas),
            "47" => Ok(Catalog53::CargosQueAfectanBaseImponibleIgvIvap),
            "48" => Ok(Catalog53::CargosQueNoAfectanBaseImponibleIgvIvap),
            "49" => Ok(Catalog53::CargosGlobalesQueAfectanBaseImponibleIgvIvap),
            "50" => Ok(Catalog53::CargosGlobalesQueNoAfectanBaseImponibleIgvIvap),
            "51" => Ok(Catalog53::PercepcionVentaInterna),
            "52" => Ok(Catalog53::PercepcionALaAdquisicionDeCombustible),
            "53" => Ok(Catalog53::PercepcionRealizadaAlAgenteDePercepcionConTasaEspecial),
            "61" => Ok(Catalog53::FactorDeAportacion),
            "62" => Ok(Catalog53::RetencionDeRentaPorAnticipos),
            "63" => Ok(Catalog53::RetencionDelIgv),
            "64" => Ok(Catalog53::RetencionDeRentaDeSegundaCategoria),
            _ => Err(format!("code {code} did not match")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog54 {
    Azucar,
    AlcoholEtilico,
    RecursosHidrobiologicos,
}

impl Catalog for Catalog54 {
    fn code(&self) -> &str {
        match &self {
            Self::Azucar => "001",
            Self::AlcoholEtilico => "003",
            Self::RecursosHidrobiologicos => "004",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Catalog59 {
    DepositoEnCuenta,
    Giro,
    TransferenciaDeFondos,
    OrdenDePago,
    TarjetaDeDebito,
    TarjetaDeCreditoEmitidaEnElPaisPorUnaEmpresaDelSistemaFinanciero,
    ChequesConLaClausulaDeNonegociableIntransferibleNoalaordenUOtraEquivalente,
    EfectivoPorOperacionesEnLasQueNoExisteObligacionDeUtilizarMedioDePago,
    EfectivoEnLosDemasCasos,
    MediosDePagoUsadosEnComercioExterior,
    DocumentosEmitidosPorLasEdpymesYLasCooperativas,
    TarjetaDeCreditoEmitidaEnElPaisOEnExteriorEmitidaPorEmpresaNoPertenecienteAlSistemaFinanciero,
    TarjetasDeCreditoEmitidasEnElExteriorPorEmpresasBancariasOFinancierasNoDomiciliadas,
    TransferenciasComercioExterior,
    ChequesBancariosComercioExterior,
    OrdenDePagoSimpleComercioExterior,
    OrdenDePagoDocumentarioComercioExterior,
    RemesaSimpleComercioExterior,
    RemesaDocumentariaComercioExterior,
    CartaDeCreditoSimpleComercioExterior,
    CartaDeCreditoDocumentarioComercioExterior,
    OtrosMediosDePago,
}

impl Catalog for Catalog59 {
    fn code(&self) -> &str {
        match &self {
            Self::DepositoEnCuenta => "001",
            Self::Giro => "002",
            Self::TransferenciaDeFondos => "003",
            Self::OrdenDePago => "004",
            Self::TarjetaDeDebito => "005",
            Self::TarjetaDeCreditoEmitidaEnElPaisPorUnaEmpresaDelSistemaFinanciero => "006",
            Self::ChequesConLaClausulaDeNonegociableIntransferibleNoalaordenUOtraEquivalente => "007",
            Self::EfectivoPorOperacionesEnLasQueNoExisteObligacionDeUtilizarMedioDePago => "008",
            Self::EfectivoEnLosDemasCasos => "009",
            Self::MediosDePagoUsadosEnComercioExterior => "010",
            Self::DocumentosEmitidosPorLasEdpymesYLasCooperativas => "011",
            Self::TarjetaDeCreditoEmitidaEnElPaisOEnExteriorEmitidaPorEmpresaNoPertenecienteAlSistemaFinanciero => "012",
            Self::TarjetasDeCreditoEmitidasEnElExteriorPorEmpresasBancariasOFinancierasNoDomiciliadas => "013",
            Self::TransferenciasComercioExterior => "101",
            Self::ChequesBancariosComercioExterior => "102",
            Self::OrdenDePagoSimpleComercioExterior => "103",
            Self::OrdenDePagoDocumentarioComercioExterior => "104",
            Self::RemesaSimpleComercioExterior => "105",
            Self::RemesaDocumentariaComercioExterior => "106",
            Self::CartaDeCreditoSimpleComercioExterior => "107",
            Self::CartaDeCreditoDocumentarioComercioExterior => "108",
            Self::OtrosMediosDePago => "999"
        }
    }
}

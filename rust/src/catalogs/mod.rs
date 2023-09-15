pub trait Catalog {
    fn code(&self) -> &str;
}

pub trait Label {
    fn label(&self) -> &str;
}

#[derive(Clone, Debug)]
pub enum Catalog1 {
    Factura,
    Boleta,
    NotaCredito,
    NotaDebito,
}

impl Catalog for Catalog1 {
    fn code(&self) -> &str {
        match &self {
            Self::Factura => "01",
            Self::Boleta => "03",
            Self::NotaCredito => "07",
            Self::NotaDebito => "08",
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

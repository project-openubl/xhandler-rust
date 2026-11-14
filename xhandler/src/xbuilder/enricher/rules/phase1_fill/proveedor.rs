use anyhow::Result;

use crate::xbuilder::enricher::bounds::proveedor::{ProveedorDireccionSetter, ProveedorGetter};
use crate::xbuilder::models::common::Direccion;

pub trait ProveedorFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> ProveedorFillRule for T
where
    T: ProveedorGetter + ProveedorDireccionSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.proveedor().direccion {
            Some(direccion) => match direccion.codigo_local {
                None => {
                    self.set_proveedor_direccion(Direccion {
                        codigo_local: Some("0000".to_string()),
                        ..direccion.clone()
                    });
                    Ok(true)
                }
                Some(_) => Ok(false),
            },
            None => {
                self.set_proveedor_direccion(Direccion {
                    codigo_pais: None,
                    departamento: None,
                    provincia: None,
                    distrito: None,
                    direccion: None,
                    urbanizacion: None,
                    ubigeo: None,
                    codigo_local: Some("0000".to_string()),
                });
                Ok(true)
            }
        }
    }
}

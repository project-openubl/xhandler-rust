use crate::enricher::bounds::proveedor::{ProveedorGetter, ProveedorSetter};
use crate::models::common::Direccion;

pub trait ProveedorFillRule {
    fn fill(&mut self) -> bool;
}

impl<T> ProveedorFillRule for T
where
    T: ProveedorGetter + ProveedorSetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_proveedor().direccion {
            Some(direccion) => match direccion.codigo_local {
                None => {
                    self.set_proveedor_direccion(Direccion {
                        codigo_local: Some("0000"),
                        ..*direccion
                    });
                    true
                }
                Some(_) => false,
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
                    codigo_local: Some("0000"),
                });
                true
            }
        }
    }
}

use crate::models::common::Direccion;
use crate::models::traits::proveedor::{ProveedorGetter, ProveedorSetter};

pub trait ProveedorEnrichRule {
    fn fill(&mut self) -> bool;
}

impl<T> ProveedorEnrichRule for T
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
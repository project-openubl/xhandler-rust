pub mod phase1_fill;
pub mod phase2_process;
pub mod phase3_summary;

use crate::xbuilder::enricher::bounds::leyendas::{LeyendasGetter, LeyendasSetter};

/// Adds the leyenda unless the document already has one with that code.
/// Returns whether it was added.
pub(crate) fn insert_leyenda_if_absent<T>(doc: &mut T, code: &str, label: &str) -> bool
where
    T: LeyendasGetter + LeyendasSetter,
{
    if doc.leyendas().contains_key(code) {
        return false;
    }
    doc.insert_leyenda(code, label);
    true
}

// ES: Sonda de consumidor externo.
// EN: External consumer probe.
use sv_bis_i0205::AdmittedDelivery;
pub fn replace(a:&mut AdmittedDelivery) { a.context().value=sv_bis_i0205::json::J::Null; }

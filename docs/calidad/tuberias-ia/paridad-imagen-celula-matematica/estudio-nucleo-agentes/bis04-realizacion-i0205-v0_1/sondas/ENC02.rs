// ES: Sonda de consumidor externo.
// EN: External consumer probe.
use sv_bis_i0205::AdmittedDelivery;
pub fn forge(a:AdmittedDelivery)->AdmittedDelivery { AdmittedDelivery { ..a } }

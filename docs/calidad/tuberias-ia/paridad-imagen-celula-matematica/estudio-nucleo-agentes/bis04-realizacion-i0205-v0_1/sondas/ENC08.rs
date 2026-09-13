// ES: Sonda de consumidor externo.
// EN: External consumer probe.
use sv_bis_i0205::AdmittedDelivery;
pub fn read(a:&AdmittedDelivery)->usize { a.vector().len()+a.descriptor().len() }

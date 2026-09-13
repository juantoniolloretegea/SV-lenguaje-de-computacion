from pathlib import Path
import json,hashlib,subprocess,shutil
D=Path('manifiesto-sv/checkout/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-realizacion-i0205-v0_1');p=D/'proyecto/src/lib.rs'
s=p.read_text();(D/'evidencias/lib-campana-01.rs').write_text(s)
s=s.replace('let b = json::receive(&mut r, max)', '''// ES: Acotar reserva y lectura por el saldo agregado antes de recibir.
            // EN: Bound reservation and reading by aggregate balance before receiving.
            let remaining = 16384usize.checked_sub(total).ok_or_else(|| err("R01", "aggregate balance", &[]))?;
            let b = json::receive(&mut r, max.min(remaining))''')
s+='''
#[cfg(test)]
mod aggregate_read_test {
    use super::*;
    use std::io::Cursor;
    // ES: El último canal sólo puede leer el saldo más un byte detector.
    // EN: The last channel may read only the balance plus one detection byte.
    #[test]
    fn aggregate_balance_bounds_actual_read() {
        let mut meta=Cursor::new(vec![b' ';4096]);
        let mut source=Cursor::new(vec![b' ';4096]);
        let mut state=Cursor::new(vec![b' ';1024]);
        let mut support=Cursor::new(vec![b' ';1024]);
        let mut geometry=Cursor::new(vec![b' ';8192]);
        let result=ReceivedBytes::read(&mut meta,&mut source,&mut state,Some(&mut support),&mut geometry);
        match result { Err(e)=>assert_eq!(e.guard,"R01"), Ok(_)=>panic!("aggregate overflow accepted") }
        assert_eq!(geometry.position(),6145);
    }
}
'''
p.write_text(s);subprocess.run(['rustfmt','--edition','2021',str(p)],check=True)
shutil.copyfile(p,'bis-realizacion/lib.rs');shutil.copyfile(p,'bis-realizacion/montaje/proyecto/src/lib.rs')
(D/'ADENDA_R01_PREVIA.json').write_text(json.dumps({'id':'R01-LECTURA-AGREGADA-01','motivo':'Revisión estática: lectura de canal limitada por cuota individual pero no por saldo agregado antes de reservar/leer.','entradas_bytes':[4096,4096,1024,1024,8192],'esperado':{'guarda':'R01','bytes_geometria_leidos':6145,'nota':'6144 de saldo y un byte detector; no leer el resto del canal'},'oraculos_integrados':'sin cambios','fuente_previa_sha256':hashlib.sha256((D/'evidencias/lib-campana-01.rs').read_bytes()).hexdigest(),'fuente_corregida_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'ejecucion':'PENDIENTE'},ensure_ascii=False,indent=2)+'\n')

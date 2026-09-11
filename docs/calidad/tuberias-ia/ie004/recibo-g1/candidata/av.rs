#![allow(dead_code)]
include!("../../semantica-a/gramatica.rs");
include!("../../semantica-a/recursos.rs");
include!("../../semantica-a/sintaxis.rs");
include!("../../semantica-a/semantica.rs");
include!("../../semantica-a/servicio.rs");
include!("../../recepcion-av/json_estricto.rs");
include!("../../recepcion-av/sha256.rs");
include!("../../recepcion-av/recepcion_a.rs");
include!("../../recepcion-av/verificar_v.rs");
fn ejecutar_v<Ra:Read,Rd:Read>(original_input:&mut Ra,input:&mut Rd,lote:&str,cuerpo:&str,max:u64)->RT<SalidaBytes>{
    if !es_huella(lote)||!es_huella(cuerpo)||max>1_000_000{return Err(FalloT::Esquema)}
    let mut c=CuentaT::nueva(true);c.max_trabajo=max;c.reservado=65536+8192; // Instantánea en argv, anexo y metadatos acotados.
    let recibido=recibir(original_input,65536,&mut c);c.instantanea_bytes=c.recibidos;c.recibidos=0;
    let original=match recibido{Ok(b)=>b,Err(e)=>return anexo_v("V_RECHAZADA",Some(e),None,cuerpo,lote,&c,None)};
    let j=match decodificar(&original,&mut c){Ok(j)=>j,Err(e)=>return anexo_v("V_RECHAZADA",Some(e),None,cuerpo,lote,&c,None)};
    let s=match solicitud(&j,&mut c){Ok(s)=>s,Err(e)=>return anexo_v("V_RECHAZADA",Some(e),None,cuerpo,lote,&c,None)};
    if max!=1_000_000&&!s.id.starts_with("PUBLICO-"){return Err(FalloT::Esquema)}
    let b=match recibir(input,262144,&mut c){Ok(b)=>b,Err(e)=>return anexo_v("V_RECHAZADA",Some(e),Some(s.id),cuerpo,lote,&c,None)};
    if b.is_empty(){return anexo_v("V_AUSENTE",None,Some(s.id),cuerpo,lote,&c,None)}
    let check=(||{let v=decodificar(&b,&mut c)?;verificar_propuesta(&v,&s,lote,&mut c)})();
    match check{Ok(proof)=>anexo_v("DERIVACION_SINTACTICA_COMPROBADA",None,Some(s.id),cuerpo,lote,&c,Some(proof)),Err(e)=>anexo_v("V_RECHAZADA",Some(e),Some(s.id),cuerpo,lote,&c,None)}
}

pub(super) fn hash(b:&[u8])->Result<[u8;32],super::Fallo>{sha256(b).map_err(|_|super::Fallo::Integridad)}
pub(super) fn hex(b:&[u8;32])->[u8;64]{hex_sha(b)}
pub(super) fn frame(b:&[u8])->Result<&[u8],super::Fallo>{validar_marco(b).map_err(|_|super::Fallo::Integridad)}
pub(super) fn a(wire:&[u8],vigente:bool)->Result<(Vec<u8>,Vec<u8>),super::Fallo>{
 let mut c=CuentaT::nueva(false);let bytes=recibir(&mut &wire[..],65536,&mut c).map_err(|_|super::Fallo::Productor)?;
 let p=resolver_a(&bytes,&mut c,vigente).map_err(|_|super::Fallo::Productor)?;Ok((p.marco,p.traza.b))
}
pub(super) struct SolicitudCanonica {pub id:Vec<u8>,pub original:Vec<u8>,pub contexto:Vec<u8>}
pub(super) fn solicitud_canonica(wire:&[u8])->Result<SolicitudCanonica,super::Fallo>{
 let mut c=CuentaT::nueva(false);let j=decodificar(wire,&mut c).map_err(|_|super::Fallo::Esquema)?;
 let s=solicitud(&j,&mut c).map_err(|_|super::Fallo::Esquema)?;
 let mut id=SalidaBytes::nueva(128).map_err(|_|super::Fallo::Reserva)?;id.str(s.id).map_err(|_|super::Fallo::Esquema)?;
 let mut original=SalidaBytes::nueva(49154).map_err(|_|super::Fallo::Reserva)?;original.str(s.pregunta).map_err(|_|super::Fallo::Esquema)?;
 let mut contexto=SalidaBytes::nueva(64).map_err(|_|super::Fallo::Reserva)?;array_u8(&mut contexto,&s.contexto.campos).map_err(|_|super::Fallo::Esquema)?;
 Ok(SolicitudCanonica{id:id.b,original:original.b,contexto:contexto.b})
}
pub(super) fn v(wire:&[u8],proposal:&[u8],lote:&str,body:&str)->Result<Vec<u8>,super::Fallo>{
 ejecutar_v(&mut &wire[..],&mut &proposal[..],lote,body,1000000).map(|a|a.b).map_err(|_|super::Fallo::Productor)
}
pub(super) fn versiones()->[&'static str;5]{[MONTAJE,PERFIL,BASE,POLITICA,FUENTE_FIJADA]}

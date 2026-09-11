#![forbid(unsafe_code)]
// Alternativa experimental al calendario de /2: no corrector semántico P3.
include!("comun.rs");
const NULO:u32=u32::MAX;
#[derive(Clone,Copy)]
enum Op{Lit(&'static str),Dig,Ref(usize),Par(u32,u32),Alt(u32,u32),Rep(u32),Vacio}
struct Gramatica{ops:Vec<Op>,dueno:Vec<u32>,reglas:[u32;25]}
impl Gramatica{
 fn poner(&mut self,o:Op,g:u32)->R<u32>{
  if self.ops.len()==512{return Err(Error::Limite)}
  let id=u32::try_from(self.ops.len()).map_err(|_|Error::Conversion)?;
  self.ops.push(o);self.dueno.push(g);Ok(id)
 }
 fn compilar(&mut self,e:E,g:u32)->R<u32>{match e{
  E::L(x)=>self.poner(Op::Lit(x),g),E::D=>self.poner(Op::Dig,g),E::R(x)=>self.poner(Op::Ref(x),g),
  E::O(x)=>{let a=self.poner(Op::Vacio,g)?;let b=self.compilar(*x,g)?;self.poner(Op::Alt(a,b),g)},
  E::M(x)=>{let a=self.compilar(*x,g)?;self.poner(Op::Rep(a),g)},
  E::S(xs)=>{let mut a=self.poner(Op::Vacio,g)?;for x in xs.iter().rev(){let b=self.compilar(*x,g)?;a=self.poner(Op::Par(b,a),g)?;}Ok(a)},
  E::A(xs)=>{let mut it=xs.iter().rev();let mut a=self.compilar(*it.next().ok_or(Error::Rango)?,g)?;for x in it{let b=self.compilar(*x,g)?;a=self.poner(Op::Alt(b,a),g)?;}Ok(a)}
 }}
 fn nueva()->R<Self>{
  let mut s=Self{ops:Vec::new(),dueno:Vec::new(),reglas:[0;25]};
  s.ops.try_reserve_exact(512).map_err(|_|Error::Reserva)?;s.dueno.try_reserve_exact(512).map_err(|_|Error::Reserva)?;
  for (i,e)in G.iter().enumerate(){s.reglas[i]=s.compilar(*e,(i+1)as u32)?;}
  let mut color=[0u8;25];for i in 0..25{dag_regla(i,&mut color)?;}
  let mut minimos=[None;25];for e in G{verificar_repeticiones(e,&mut minimos)?;}Ok(s)
 }
}
fn dag_regla(g:usize,c:&mut[u8;25])->R<()>{if c[g]==1{return Err(Error::Rango)}if c[g]==2{return Ok(())}c[g]=1;dag_expr(G[g],c)?;c[g]=2;Ok(())}
fn dag_expr(e:E,c:&mut[u8;25])->R<()>{match e{E::R(g)=>dag_regla(g,c),E::S(xs)|E::A(xs)=>{for e in xs{dag_expr(*e,c)?;}Ok(())},E::O(e)|E::M(e)=>dag_expr(*e,c),_=>Ok(())}}
fn minimo(e:E,m:&mut[Option<u64>;25])->R<u64>{match e{
 E::L(x)=>Ok(x.split(' ').count()as u64),E::D=>Ok(1),E::O(_)|E::M(_)=>Ok(0),
 E::R(g)=>{if let Some(x)=m[g]{return Ok(x)}let x=minimo(G[g],m)?;m[g]=Some(x);Ok(x)},
 E::S(xs)=>{let mut n=0;for e in xs{n=suma(n,minimo(*e,m)?)?;}Ok(n)},
 E::A(xs)=>{let mut n=u64::MAX;for e in xs{n=n.min(minimo(*e,m)?);}Ok(n)}
}}
fn verificar_repeticiones(e:E,m:&mut[Option<u64>;25])->R<()>{match e{
 E::M(x)=>{if minimo(*x,m)?==0{return Err(Error::Rango)}verificar_repeticiones(*x,m)},
 E::S(xs)|E::A(xs)=>{for e in xs{verificar_repeticiones(*e,m)?;}Ok(())},E::O(x)=>verificar_repeticiones(*x,m),_=>Ok(())
}}
#[derive(Clone,Copy)]
struct Nodo{expr:u32,inicio:u32,fin:u32,primera:u32,ultima:u32}
#[derive(Clone,Copy)]
struct Familia{rama:u32,a:u32,b:u32,siguiente:u32}
struct Memoria{usado:u64,pico_solicitado:u64}
impl Memoria{
 fn capacidad<T>(&mut self,v:&mut Vec<T>,n:usize)->R<()>{
  if n<=v.capacity(){return Ok(())}
  let vieja=producto(v.capacity()as u64,std::mem::size_of::<T>()as u64)?;
  let nueva=producto(n as u64,std::mem::size_of::<T>()as u64)?;
  let pico=suma(self.usado,nueva)?;if pico>32*1024*1024{return Err(Error::Limite)}
  self.pico_solicitado=self.pico_solicitado.max(pico);
  v.try_reserve_exact(n-v.len()).map_err(|_|Error::Reserva)?;
  let real=producto(v.capacity()as u64,std::mem::size_of::<T>()as u64)?;
  self.usado=suma(self.usado-vieja,real)?;if self.usado>32*1024*1024{return Err(Error::Limite)}Ok(())
 }
 fn push<T>(&mut self,v:&mut Vec<T>,x:T)->R<()>{
  let n=indice(suma(v.len()as u64,1)?)?;
  if n>v.capacity(){self.capacidad(v,indice(producto(n as u64,2)?)?)?;}v.push(x);Ok(())
 }
}
struct Motor<'a>{g:&'a Gramatica,t:&'a[Token],ancho:usize,intervalos:usize,tabla:Vec<u32>,estado:Vec<u8>,salidas:Vec<Vec<u32>>,nodos:Vec<Nodo>,familias:Vec<Familia>,mem:Memoria,cuenta:Cuenta,tipos:[u64;7]}
impl<'a> Motor<'a>{
 fn nuevo(g:&'a Gramatica,t:&'a[Token])->R<Self>{
  let a=suma(t.len()as u64,1)?;let ints=producto(a,suma(a,1)?)?/2;
  let q=indice(producto(g.ops.len()as u64,a)?)?;let sz=indice(producto(g.ops.len()as u64,ints)?)?;
  let mut mem=Memoria{usado:0,pico_solicitado:0};let mut tabla=Vec::new();let mut estado=Vec::new();let mut salidas=Vec::new();
  mem.capacidad(&mut tabla,sz)?;tabla.resize(sz,NULO);mem.capacidad(&mut estado,q)?;estado.resize(q,0);
  mem.capacidad(&mut salidas,q)?;for _ in 0..q{salidas.push(Vec::new());}
  Ok(Self{g,t,ancho:indice(a)?,intervalos:indice(ints)?,tabla,estado,salidas,nodos:Vec::new(),familias:Vec::new(),mem,cuenta:Cuenta{usado:0,limite:1_000_000},tipos:[0;7]})
 }
 // Unidades: evaluación, consulta memo, byte comparado, par examinado,
 // cotejo de familia duplicada, inserción de familia, inserción de nodo.
 fn cobrar(&mut self,k:usize)->R<()>{self.cuenta.cobrar(1)?;self.tipos[k]=suma(self.tipos[k],1)?;Ok(())}
 fn clave(&self,e:u32,i:usize)->R<usize>{if i>=self.ancho{return Err(Error::Rango)}indice(suma(producto(e as u64,self.ancho as u64)?,i as u64)?)}
 fn celda(&self,e:u32,i:usize,j:usize)->R<usize>{
  if i>j||j>=self.ancho{return Err(Error::Rango)}
  let pref=producto(i as u64,suma(producto(2,self.ancho as u64)?,1)?-i as u64)?/2;
  indice(suma(producto(e as u64,self.intervalos as u64)?,suma(pref,(j-i)as u64)?)?)
 }
 fn emitir(&mut self,e:u32,i:usize,j:usize,rama:u32,a:u32,b:u32)->R<()>{
  let pos=self.celda(e,i,j)?;let mut id=*self.tabla.get(pos).ok_or(Error::Rango)?;
  if id==NULO{
   self.cobrar(6)?;id=u32::try_from(self.nodos.len()).map_err(|_|Error::Conversion)?;
   self.mem.push(&mut self.nodos,Nodo{expr:e,inicio:u32::try_from(i).map_err(|_|Error::Conversion)?,fin:u32::try_from(j).map_err(|_|Error::Conversion)?,primera:NULO,ultima:NULO})?;
   *self.tabla.get_mut(pos).ok_or(Error::Rango)?=id;
   let k=self.clave(e,i)?;self.mem.push(self.salidas.get_mut(k).ok_or(Error::Rango)?,id)?;
  }
  let mut f=self.nodos[id as usize].primera;
  while f!=NULO{self.cobrar(4)?;let viejo=self.familias[f as usize];if viejo.rama==rama&&viejo.a==a&&viejo.b==b{return Ok(())}f=viejo.siguiente;}
  self.cobrar(5)?;let nuevo=u32::try_from(self.familias.len()).map_err(|_|Error::Conversion)?;
  self.mem.push(&mut self.familias,Familia{rama,a,b,siguiente:NULO})?;
  let n=&mut self.nodos[id as usize];if n.ultima==NULO{n.primera=nuevo}else{self.familias[n.ultima as usize].siguiente=nuevo}n.ultima=nuevo;Ok(())
 }
 fn evaluar(&mut self,e:u32,i:usize)->R<()>{
  self.cobrar(1)?;let k=self.clave(e,i)?;
  match *self.estado.get(k).ok_or(Error::Rango)?{2=>return Ok(()),1=>return Err(Error::Rango),_=>()}
  self.cobrar(0)?;self.estado[k]=1;
  match *self.g.ops.get(e as usize).ok_or(Error::Rango)?{
   Op::Vacio=>self.emitir(e,i,i,0,NULO,NULO)?,
   Op::Lit(l)=>{
    let n=l.split(' ').count();let fin=indice(suma(i as u64,n as u64)?)?;
    if fin<self.ancho{
     let mut igual=true;
     for (d,p)in l.split(' ').enumerate(){let t=&self.t[i+d].normal;let tb=t.as_bytes();let pb=p.as_bytes();if tb.len()!=pb.len(){igual=false;continue}
      for z in 0..pb.len(){self.cobrar(2)?;if tb[z]!=pb[z]{igual=false}}
     }
     if igual{self.emitir(e,i,fin,0,NULO,NULO)?}
    }
   },
   Op::Dig=>{if let Some(t)=self.t.get(i){let bs=t.normal.as_bytes();let mut valido=(1..=10).contains(&bs.len());if valido{for b in bs{self.cobrar(2)?;valido&=b.is_ascii_digit()}}if valido{self.emitir(e,i,i+1,0,NULO,NULO)?}}},
   Op::Ref(g)=>{let c=self.g.reglas[g];self.evaluar(c,i)?;let ck=self.clave(c,i)?;for ix in 0..self.salidas[ck].len(){self.cobrar(3)?;let a=self.salidas[ck][ix];let j=self.nodos[a as usize].fin as usize;self.emitir(e,i,j,0,a,NULO)?;}},
   Op::Alt(a,b)=>{for (rama,c)in [(0,a),(1,b)]{self.evaluar(c,i)?;let ck=self.clave(c,i)?;for ix in 0..self.salidas[ck].len(){self.cobrar(3)?;let a=self.salidas[ck][ix];let j=self.nodos[a as usize].fin as usize;self.emitir(e,i,j,rama,a,NULO)?;}}},
   Op::Par(a,b)=>{
    self.evaluar(a,i)?;let ak=self.clave(a,i)?;
    for ai in 0..self.salidas[ak].len(){let an=self.salidas[ak][ai];let j=self.nodos[an as usize].fin as usize;self.evaluar(b,j)?;let bk=self.clave(b,j)?;
     for bi in 0..self.salidas[bk].len(){self.cobrar(3)?;let bn=self.salidas[bk][bi];let fin=self.nodos[bn as usize].fin as usize;self.emitir(e,i,fin,0,an,bn)?;}
    }
   },
   Op::Rep(c)=>{
    self.emitir(e,i,i,0,NULO,NULO)?;self.evaluar(c,i)?;let ck=self.clave(c,i)?;
    for ci in 0..self.salidas[ck].len(){let cn=self.salidas[ck][ci];let j=self.nodos[cn as usize].fin as usize;if j<=i{return Err(Error::Rango)}
     self.evaluar(e,j)?;let rk=self.clave(e,j)?;
     for ri in 0..self.salidas[rk].len(){self.cobrar(3)?;let rn=self.salidas[rk][ri];let fin=self.nodos[rn as usize].fin as usize;self.emitir(e,i,fin,1,cn,rn)?;}
    }
   }
  }
  self.estado[k]=2;Ok(())
 }
 fn raiz(&self)->R<u32>{Ok(self.tabla[self.celda(self.g.reglas[0],0,self.t.len())?])}
 // Exportación exacta del bosque compacto; los nodos comparten subestructuras,
 // pero cada familia alternativa conserva sus hijos y su identidad local.
 fn imprimir_bosque(&self){
  print!("\"nodos\":[");for(i,n)in self.nodos.iter().enumerate(){if i>0{print!(",")}print!("[{},{},{},{},{}]",n.expr,n.inicio,n.fin,n.primera,n.ultima)}
  print!("],\"familias\":[");for(i,f)in self.familias.iter().enumerate(){if i>0{print!(",")}print!("[{},{},{},{}]",f.rama,f.a,f.b,f.siguiente)}print!("]");
 }
}
include!("casos.rs");
fn main(){
 let g=Gramatica::nueva().expect("gramatica finita DAG");
 let alternativas:u64=G.iter().map(|e|match e{E::A(x)=>x.len()as u64,_=>1}).sum();
 println!("{{\"instrumento\":\"IE004-R10-MEMO/1\",\"primitivas_correctas\":{},\"usize_bits\":{},\"expresiones\":{},\"alternativas_raiz\":{},\"dependencias_dag\":true,\"dos_barridos_127\":{},\"dos_barridos_128\":{}}}",primitivas(),usize::BITS,g.ops.len(),alternativas,2*alternativas*128*129/2,2*alternativas*129*130/2);
 for(id,q,_)in CASOS{
  match lexicalizar(q){Err(e)=>println!("{{\"id\":\"{}\",\"bytes\":{},\"estado\":\"{:?}\",\"unidades\":0}}",id,q.len(),e),Ok(t)=>{
   for token in &t{assert!(vista(q,token.inicio,token.fin).is_ok());}
   match Motor::nuevo(&g,&t){Err(e)=>println!("{{\"id\":\"{}\",\"estado\":\"{:?}\",\"unidades\":0}}",id,e),Ok(mut m)=>{
    let ejec=m.evaluar(g.reglas[0],0);let raiz=m.raiz().expect("indice raiz");let estado=match ejec{Ok(())=>if raiz==NULO{"NO_DERIVA"}else{"DERIVA"},Err(Error::Limite)=>"PRESUPUESTO_AGOTADO",Err(_)=>"ERROR_TECNICO"};
    print!("{{\"id\":\"{}\",\"bytes\":{},\"tokens\":{},\"estado\":\"{}\",\"unidades\":{},\"categorias\":{:?},\"nodos_total\":{},\"familias_total\":{},\"capacidad_bytes\":{},\"pico_solicitado_bytes\":{},\"raiz\":{},",id,q.len(),t.len(),estado,m.cuenta.usado,m.tipos,m.nodos.len(),m.familias.len(),m.mem.usado,m.mem.pico_solicitado,raiz);
    m.imprimir_bosque();println!("}}");
   }}
  }}
 }
}

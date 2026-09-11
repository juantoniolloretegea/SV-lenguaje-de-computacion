#[derive(Clone,Copy)]
enum Op { Lit(&'static str), Dig, Ref(usize), Par(u32,u32), Disy(u32,u32), Alt(u32,u32), Rep(u32), Negar(u32), Vacio }
struct Gramatica { ops: [Op;512], dueno: [u8;512], n: usize, reglas: [u32;25] }
impl Gramatica {
    fn poner(&mut self,o:Op,g:u8)->R<u32> {
        if self.n==512 { return Err(Error::Constitucion); }
        let i=u32::try_from(self.n).map_err(|_|Error::Conversion)?;
        self.ops[self.n]=o; self.dueno[self.n]=g; self.n+=1; Ok(i)
    }
    fn compilar(&mut self,e:E,g:u8)->R<u32> { match e {
        E::L(x)=>self.poner(Op::Lit(x),g), E::D=>self.poner(Op::Dig,g), E::R(x)=>self.poner(Op::Ref(x),g),
        E::O(x)=>{let a=self.poner(Op::Vacio,g)?;let b=self.compilar(*x,g)?;self.poner(Op::Alt(a,b),g)},
        E::M(x)=>{let a=self.compilar(*x,g)?;self.poner(Op::Rep(a),g)},
        E::A(xs)=>{let mut it=xs.iter().rev();let mut a=self.compilar(*it.next().ok_or(Error::Constitucion)?,g)?;for x in it{let b=self.compilar(*x,g)?;a=self.poner(Op::Alt(b,a),g)?;}Ok(a)},
        E::S(xs)=>{
            let mut a=self.poner(Op::Vacio,g)?;
            for (i,x) in xs.iter().enumerate().rev() {
                let mut b=self.compilar(*x,g)?;
                // G02 alternativa 6: el primer recorte, y sólo ése, es negativo.
                if g==2 && xs.len()==6 && matches!(xs[1],E::L("no")) && i==0 { b=self.poner(Op::Negar(b),g)?; }
                a=self.poner(Op::Par(b,a),g)?;
            }
            // G10 segunda alternativa: la secuencia completa deriva; sus dos átomos son alternativas semánticas.
            if g==10 && xs.len()==3 && matches!(xs[1],E::L("o")) {
                let Op::Par(x,y)=self.ops[a as usize] else { return Err(Error::Constitucion); };
                self.ops[a as usize]=Op::Disy(x,y);
            }
            Ok(a)
        }
    }}
    fn nueva()->R<Self> {
        let mut s=Self{ops:[Op::Vacio;512],dueno:[0;512],n:0,reglas:[0;25]};
        let mut color=[0u8;25]; for i in 0..25 { dag_regla(i,&mut color)?; }
        let mut minimos=[None;25];for e in G { verificar_repeticiones(e,&mut minimos)?; }
        for (i,e) in G.iter().enumerate(){s.reglas[i]=s.compilar(*e,(i+1) as u8)?;} Ok(s)
    }
}
fn dag_regla(g:usize,c:&mut[u8;25])->R<()> { if c[g]==1{return Err(Error::Constitucion)}if c[g]==2{return Ok(())}c[g]=1;dag_expr(G[g],c)?;c[g]=2;Ok(()) }
fn dag_expr(e:E,c:&mut[u8;25])->R<()> {match e {E::R(g)=>dag_regla(g,c),E::S(xs)|E::A(xs)=>{for e in xs{dag_expr(*e,c)?;}Ok(())},E::O(e)|E::M(e)=>dag_expr(*e,c),_=>Ok(())}}
fn minimo(e:E,m:&mut[Option<u64>;25])->R<u64>{match e{
 E::L(x)=>Ok(x.split(' ').count() as u64),E::D=>Ok(1),E::O(_)|E::M(_)=>Ok(0),
 E::R(g)=>{if let Some(x)=m[g]{return Ok(x)}let x=minimo(G[g],m)?;m[g]=Some(x);Ok(x)},
 E::S(xs)=>{let mut n=0;for e in xs{n=suma(n,minimo(*e,m)?)?;}Ok(n)},
 E::A(xs)=>{let mut n=u64::MAX;for e in xs{n=n.min(minimo(*e,m)?);}Ok(n)}
}}
fn verificar_repeticiones(e:E,m:&mut[Option<u64>;25])->R<()>{match e{
 E::M(x)=>{if minimo(*x,m)?==0{return Err(Error::Constitucion)}verificar_repeticiones(*x,m)},
 E::S(xs)|E::A(xs)=>{for e in xs{verificar_repeticiones(*e,m)?;}Ok(())},E::O(x)=>verificar_repeticiones(*x,m),_=>Ok(())
}}
#[derive(Clone,Copy)]
struct Celda { primera:u32,ultima:u32,estado:u8 }
const CELDA:Celda=Celda{primera:NULO,ultima:NULO,estado:0};
#[derive(Clone,Copy)]
struct Nodo { expr:u32,inicio:u32,fin:u32,primera:u32,ultima:u32,siguiente:u32 }
#[derive(Clone,Copy)]
struct Familia { rama:u32,a:u32,b:u32,siguiente:u32 }
struct Motor<'a> {
    g:&'a Gramatica, original:&'a str, lexico:Lexico, ancho:usize, c:Cupos, r:Recursos,
    celdas:Vec<Celda>, nodos:Vec<Nodo>, familias:Vec<Familia>,
    sem_celdas:Vec<Celda>, sem_estados:Vec<EstadoSem>, raices:Vec<Significado>,
    profundidad_sintactica:usize, profundidad_semantica:usize,
}
impl<'a> Motor<'a> {
    fn nuevo(g:&'a Gramatica,q:&'a str,c:Cupos)->Result<Self,(Error,Recursos)> {
        let mut r=Recursos::default();
        let preparacion=(|| -> R<(Lexico,usize,Vec<Celda>)> {
        let lexico=Lexico::nueva(q,&mut r,c)?;
        let ancho=indice(suma(lexico.tokens.len() as u64,1)?)?;
        let cantidad=producto(g.n as u64,ancho as u64)?;let mut celdas=Vec::new();r.reservar(c,2,&mut celdas,cantidad)?;
        // Se cobra cada celda inicializada, también si nunca resulta útil.
        r.cobrar(c,7,cantidad)?;celdas.resize(indice(cantidad)?,CELDA);
        Ok((lexico,ancho,celdas))})();
        let (lexico,ancho,celdas)=match preparacion{Ok(x)=>x,Err(e)=>return Err((e,r))};
        Ok(Self{g,original:q,lexico,ancho,c,r,celdas,nodos:Vec::new(),familias:Vec::new(),sem_celdas:Vec::new(),sem_estados:Vec::new(),raices:Vec::new(),profundidad_sintactica:0,profundidad_semantica:0})
    }
    fn cobrar(&mut self,k:usize,n:u64)->R<()> {self.r.cobrar(self.c,k,n)}
    fn clave(&self,e:u32,i:usize)->R<usize> {
        if e as usize>=self.g.n||i>=self.ancho{return Err(Error::Rango)}
        indice(suma(producto(e as u64,self.ancho as u64)?,i as u64)?)
    }
    fn buscar(&mut self,e:u32,i:usize,j:usize)->R<u32> {
        let mut n=self.celdas[self.clave(e,i)?].primera;
        while n!=NULO {self.cobrar(2,1)?;let x=self.nodos[n as usize];if x.fin as usize==j{return Ok(n)}n=x.siguiente;} Ok(NULO)
    }
    fn emitir(&mut self,e:u32,i:usize,j:usize,rama:u32,a:u32,b:u32)->R<()> {
        if i>j||j>=self.ancho{return Err(Error::Rango)}let k=self.clave(e,i)?;let mut id=self.buscar(e,i,j)?;
        if id==NULO {
            id=u32::try_from(self.nodos.len()).map_err(|_|Error::Conversion)?;
            self.r.poner(self.c,3,&mut self.nodos,Nodo{expr:e,inicio:u32::try_from(i).map_err(|_|Error::Conversion)?,fin:u32::try_from(j).map_err(|_|Error::Conversion)?,primera:NULO,ultima:NULO,siguiente:NULO},u32::MAX as u64-1)?;
            let c=&mut self.celdas[k];if c.ultima==NULO{c.primera=id}else{self.nodos[c.ultima as usize].siguiente=id}c.ultima=id;
        }
        let mut f=self.nodos[id as usize].primera;
        while f!=NULO {self.cobrar(4,1)?;let z=self.familias[f as usize];if(z.rama,z.a,z.b)==(rama,a,b){return Ok(())}f=z.siguiente;}
        let nuevo=u32::try_from(self.familias.len()).map_err(|_|Error::Conversion)?;
        self.r.poner(self.c,4,&mut self.familias,Familia{rama,a,b,siguiente:NULO},u32::MAX as u64-1)?;
        let n=&mut self.nodos[id as usize];if n.ultima==NULO{n.primera=nuevo}else{self.familias[n.ultima as usize].siguiente=nuevo}n.ultima=nuevo;Ok(())
    }
    fn copiar_hijos(&mut self,e:u32,i:usize,c:u32,rama:u32)->R<()> {
        let mut a=self.celdas[self.clave(c,i)?].primera;
        while a!=NULO {self.cobrar(3,1)?;let n=self.nodos[a as usize];self.emitir(e,i,n.fin as usize,rama,a,NULO)?;a=n.siguiente;}Ok(())
    }
    fn evaluar(&mut self,e:u32,i:usize,d:usize)->R<()> {
        if d>512{return Err(Error::Profundidad)}self.profundidad_sintactica=self.profundidad_sintactica.max(d);
        self.cobrar(1,1)?;let k=self.clave(e,i)?;
        match self.celdas[k].estado {2=>return Ok(()),1=>return Err(Error::Constitucion),_=>()}
        self.celdas[k].estado=1;
        match self.g.ops[e as usize] {
            Op::Vacio=>self.emitir(e,i,i,0,NULO,NULO)?,
            Op::Lit(l)=>{
                let fin=indice(suma(i as u64,l.split(' ').count() as u64)?)?;
                if fin<self.ancho {
                    let mut igual=true;
                    for (dx,p) in l.split(' ').enumerate() {
                        let t=self.lexico.tokens[i+dx];let len=t.fin-t.inicio;
                        self.cobrar(0,1)?;
                        if len!=p.len() as u64 {igual=false;continue;}
                        for(z,pb)in p.bytes().enumerate(){self.cobrar(0,1)?;if self.lexico.normal[indice(t.inicio)?+z]!=pb{igual=false;}}
                    }
                    if igual{self.emitir(e,i,fin,0,NULO,NULO)?;}
                }
            },
            Op::Dig=>{
                if let Some(t)=self.lexico.tokens.get(i).copied(){let len=t.fin-t.inicio;let mut bien=(1..=10).contains(&len);
                    if bien{for j in t.inicio..t.fin{self.cobrar(0,1)?;bien&=self.lexico.normal[indice(j)?].is_ascii_digit();}}
                    if bien{self.emitir(e,i,i+1,0,NULO,NULO)?;}
                }
            },
            Op::Ref(g)=>{let c=self.g.reglas[g];self.evaluar(c,i,d+1)?;self.copiar_hijos(e,i,c,0)?;},
            Op::Negar(c)=>{self.evaluar(c,i,d+1)?;self.copiar_hijos(e,i,c,0)?;},
            Op::Alt(a,b)=>{for(rama,c)in[(0,a),(1,b)]{self.evaluar(c,i,d+1)?;self.copiar_hijos(e,i,c,rama)?;}},
            Op::Par(a,b)|Op::Disy(a,b)=>{
                self.evaluar(a,i,d+1)?;let mut an=self.celdas[self.clave(a,i)?].primera;
                while an!=NULO {let n=self.nodos[an as usize];let j=n.fin as usize;self.evaluar(b,j,d+1)?;
                    let mut bn=self.celdas[self.clave(b,j)?].primera;
                    while bn!=NULO{self.cobrar(3,1)?;let child=self.nodos[bn as usize];self.emitir(e,i,child.fin as usize,0,an,bn)?;bn=child.siguiente;}
                    an=n.siguiente;
                }
            },
            Op::Rep(c)=>{
                self.emitir(e,i,i,0,NULO,NULO)?;self.evaluar(c,i,d+1)?;let mut cn=self.celdas[self.clave(c,i)?].primera;
                while cn!=NULO{let n=self.nodos[cn as usize];let j=n.fin as usize;if j<=i{return Err(Error::Constitucion)}self.evaluar(e,j,d+1)?;
                    let mut rn=self.celdas[self.clave(e,j)?].primera;
                    while rn!=NULO{self.cobrar(3,1)?;let r=self.nodos[rn as usize];self.emitir(e,i,r.fin as usize,1,cn,rn)?;rn=r.siguiente;}cn=n.siguiente;
                }
            }
        }
        self.celdas[k].estado=2;Ok(())
    }
}

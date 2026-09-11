// Los cinco conjuntos son de identificadores del puesto; nunca longitudes de palabras.
// OP: LEER=1, ESCRIBIR=2, ELIMINAR=4. Objeto A=1,B=2. Parámetro G=1,A=2,M=4,H=8,HG=16.
// Momento ACTUAL=1,ANTERIOR=2. Campo VALOR=1,UNIDAD=2,ESTADO=4,FUENTE=8,ALCANCE=16,INTERVALO=32.
#[derive(Clone,Copy,Default,Debug,PartialEq,Eq,PartialOrd,Ord)]
struct Texto { bytes:[u8;10], n:u8 }
#[derive(Clone,Copy,Default,Debug,PartialEq,Eq,PartialOrd,Ord)]
struct Restriccion { campos:[u8;5], pendientes:u8, textos:[Texto;2], ntextos:u8 }
#[derive(Clone,Copy,Default,Debug,PartialEq,Eq,PartialOrd,Ord)]
struct Significado { pos:Restriccion, neg:[Restriccion;2], nneg:u8, positiva:bool, fuera:bool, modificador_glicosilada:bool }
#[derive(Clone,Copy)]
struct EstadoSem { significado:Significado, nodo:u32, siguiente:u32 }
#[derive(Clone,Copy)]
struct Contexto { campos:[u8;5] }
impl Contexto {
    fn validar(self)->R<()> {
        for(v,mask)in self.campos.into_iter().zip([3u8,3,7,3,31]) {
            if v&!mask!=0 || v.count_ones()>1 {return Err(Error::Contexto)}
        } Ok(())
    }
}
impl Restriccion {
    fn agregar_texto(&mut self,t:Texto)->R<()> {
        if t.n==0{return Ok(())}
        for i in 0..self.ntextos as usize{if self.textos[i]==t{return Ok(())}}
        if self.ntextos==2{return Err(Error::Constitucion)}
        self.textos[self.ntextos as usize]=t;self.ntextos+=1;
        if self.ntextos==2&&self.textos[1]<self.textos[0]{self.textos.swap(0,1)}Ok(())
    }
    fn unir(mut self,b:Self)->R<Self>{
        for i in 0..5{self.campos[i]|=b.campos[i];}self.pendientes|=b.pendientes;
        for i in 0..b.ntextos as usize{self.agregar_texto(b.textos[i])?;}Ok(self)
    }
    fn demanda(&mut self,ctx:Contexto,i:usize){if ctx.campos[i]==0{self.pendientes|=1<<i}else{self.campos[i]|=ctx.campos[i]}}
    fn fuera(&self)->bool {self.campos[2]&24!=0||self.campos[4]&32!=0}
    fn contradictoria(&self)->bool {self.campos.iter().any(|v|v.count_ones()>1)||self.ntextos>1}
}
impl Significado {
    fn agregar_negacion(&mut self,p:Restriccion)->R<()> {
        for i in 0..self.nneg as usize{if self.neg[i]==p{return Ok(())}}
        if self.nneg==2{return Err(Error::Constitucion)}
        self.neg[self.nneg as usize]=p;self.nneg+=1;
        if self.nneg==2&&self.neg[1]<self.neg[0]{self.neg.swap(0,1)}Ok(())
    }
    fn unir(mut self,b:Self)->R<Self>{
        self.pos=self.pos.unir(b.pos)?;self.positiva|=b.positiva;self.fuera|=b.fuera;
        self.modificador_glicosilada|=b.modificador_glicosilada;
        for i in 0..b.nneg as usize{self.agregar_negacion(b.neg[i])?;}Ok(self)
    }
    fn negar(mut self)->R<Self>{
        if self.nneg>0 {if self.pos!=Restriccion::default()||self.positiva{return Err(Error::Constitucion)}return Ok(self)}
        self.fuera|=self.pos.fuera();self.agregar_negacion(self.pos)?;self.pos=Restriccion::default();self.positiva=false;Ok(self)
    }
    fn pendientes(&self)->u8{let mut p=self.pos.pendientes;for i in 0..self.nneg as usize{p|=self.neg[i].pendientes;}p}
    fn completar(mut self,ctx:Contexto)->R<Self>{
        self.fuera|=self.pos.fuera();
        if self.positiva && !self.fuera && self.pendientes()==0 {
            for i in 0..5 {if self.pos.campos[i]==0 {if ctx.campos[i]==0{self.pos.pendientes|=1<<i}else{self.pos.campos[i]=ctx.campos[i]}}}
        }
        // Si un patrón compatible exige operando, no inventarlo cuando la escritura positiva carece de él.
        if self.positiva&&self.pos.campos[0]==2&&self.pos.ntextos==0{
            for i in 0..self.nneg as usize{let p=self.neg[i];
                if p.ntextos>0&&(0..5).all(|j|p.campos[j]==0||(self.pos.campos[j]&p.campos[j])==p.campos[j]){self.pos.pendientes|=32;}
            }
        }
        Ok(self)
    }
    fn causas(&self)->u32{
        let mut contra=self.pos.contradictoria();
        if self.positiva && !contra {
            for i in 0..self.nneg as usize {let p=self.neg[i];
                let campos=(0..5).all(|j|p.campos[j]==0||(self.pos.campos[j]&p.campos[j])==p.campos[j]);
                let texto=p.ntextos==0 || (p.ntextos==1&&self.pos.ntextos==1&&p.textos[0]==self.pos.textos[0]);
                if p.pendientes==0&&campos&&texto{contra=true;}
            }
        }
        u32::from(contra) | (u32::from(!self.positiva)<<1) | (u32::from(self.pos.campos[0]&4!=0)<<2) |
        (u32::from(self.fuera)<<3) | (u32::from(self.pendientes()!=0)<<4)
    }
}
impl<'a> Motor<'a> {
    fn terminal_sem(&mut self,n:Nodo,l:&str,ctx:Contexto)->R<Significado> {
        self.cobrar(5,l.len() as u64+1)?;let g=self.g.dueno[n.expr as usize];let mut s=Significado::default();
        match (g,l) {
            (9,"valor"|"dato"|"cifra")=>s.pos.campos[4]=1,
            (9,"unidad")|(4,"en qué unidad")=>s.pos.campos[4]=2,
            (9,"estado")=>s.pos.campos[4]=4,
            (9,"fuente"|"procedencia")=>s.pos.campos[4]=8,
            (9,"límites"|"alcance")=>s.pos.campos[4]=16,
            (9,"intervalo de referencia")=>{s.pos.campos[4]=32;s.fuera=true;},
            (11,"igg")=>s.pos.campos[2]=1,(11,"iga")=>s.pos.campos[2]=2,(11,"igm")=>s.pos.campos[2]=4,
            (11,"hemoglobina")=>s.pos.campos[2]=8,
            (11,"glicosilada")=>s.modificador_glicosilada=true,
            (11,"inmunoglobulina"|"inmunoglobina"|"linmunoglobina")=>s.pos.demanda(ctx,2),
            (12|17,"actual"|"ahora")=>s.pos.campos[3]=1,
            (12|17,"anterior"|"previo")=>s.pos.campos[3]=2,
            (15,"caso-a")=>s.pos.campos[1]=1,(15,"caso-b")=>s.pos.campos[1]=2,
            (24,"consulte"|"lea"|"traiga"|"dígame"|"necesito")=>s.pos.campos[0]=1,
            (24,"cambie")|(4,"escriba")|(18,"no escriba")=>s.pos.campos[0]=2,
            (24,"elimine")=>s.pos.campos[0]=4,
            (18,"no cambie nada")=>{for op in[2,4]{let mut p=Restriccion::default();p.campos[0]=op;s.agregar_negacion(p)?;}},
            _=>()
        }
        Ok(s)
    }
    fn regla_sem(&mut self,n:Nodo,mut s:Significado,ctx:Contexto)->R<Significado>{
        let g=self.g.dueno[n.expr as usize] as usize;
        if self.g.reglas[g-1]!=n.expr{return Ok(s)}
        self.cobrar(5,1)?;
        match g{
            4=>{s.positiva=true;if s.pos.campos[0]==0{s.pos.campos[0]=1;}},
            11=>{if s.modificador_glicosilada{if s.pos.campos[2]!=8{return Err(Error::Constitucion)}s.pos.campos[2]=16;s.modificador_glicosilada=false;}s.fuera|=s.pos.fuera();},
            16=>s.pos.demanda(ctx,1),
            18=>s=s.negar()?,
            25=>{if s.pos.campos[1]==0&&s.pos.pendientes&2==0{s.pos.demanda(ctx,1);}},
            _=>()
        }Ok(s)
    }
    fn insertar_sem(&mut self,id:u32,s:Significado,ctx:Contexto)->R<()> {
        let n=self.nodos[id as usize];let s=self.regla_sem(n,s,ctx)?;let mut cursor=self.sem_celdas[id as usize].primera;
        while cursor!=NULO {
            self.cobrar(6,std::mem::size_of::<Significado>() as u64)?;
            let viejo=self.sem_estados[cursor as usize];if viejo.significado==s{return Ok(())}cursor=viejo.siguiente;
        }
        let nuevos=suma(self.r.estados,1)?;if nuevos>self.c.estados{return Err(Error::Estados)}
        let idx=u32::try_from(self.sem_estados.len()).map_err(|_|Error::Conversion)?;
        self.r.poner(self.c,6,&mut self.sem_estados,EstadoSem{significado:s,nodo:id,siguiente:NULO},self.c.estados)?;
        self.r.estados=nuevos;
        let c=&mut self.sem_celdas[id as usize];if c.ultima==NULO{c.primera=idx}else{self.sem_estados[c.ultima as usize].siguiente=idx;}c.ultima=idx;Ok(())
    }
    fn sem_copiar(&mut self,id:u32,hijo:u32,ctx:Contexto,negar:bool)->R<()> {
        let mut a=self.sem_celdas[hijo as usize].primera;
        while a!=NULO{self.cobrar(5,1)?;let z=self.sem_estados[a as usize];let s=if negar{z.significado.negar()?}else{z.significado};self.insertar_sem(id,s,ctx)?;a=z.siguiente;}Ok(())
    }
    fn sem_evaluar(&mut self,id:u32,ctx:Contexto,d:usize)->R<()> {
        if d>512{return Err(Error::Profundidad)}self.profundidad_semantica=self.profundidad_semantica.max(d);
        self.cobrar(5,1)?;
        match self.sem_celdas.get(id as usize).ok_or(Error::Rango)?.estado {2=>return Ok(()),1=>return Err(Error::Constitucion),_=>()}
        self.sem_celdas[id as usize].estado=1;let n=self.nodos[id as usize];let op=self.g.ops[n.expr as usize];let mut f=n.primera;
        while f!=NULO {
            self.cobrar(5,1)?;let z=self.familias[f as usize];
            match op {
                Op::Vacio=>self.insertar_sem(id,Significado::default(),ctx)?,
                Op::Lit(l)=>{let s=self.terminal_sem(n,l,ctx)?;self.insertar_sem(id,s,ctx)?;},
                Op::Dig=>{
                    let t=self.lexico.token(n.inicio as usize)?;if t.is_empty()||t.len()>10{return Err(Error::Constitucion)}
                    let mut txt=Texto::default();txt.n=t.len() as u8;txt.bytes[..t.len()].copy_from_slice(t.as_bytes());
                    self.cobrar(5,txt.n as u64)?;let mut s=Significado::default();s.pos.agregar_texto(txt)?;self.insertar_sem(id,s,ctx)?;
                },
                Op::Ref(_)|Op::Alt(_,_)|Op::Negar(_)=>{
                    self.sem_evaluar(z.a,ctx,d+1)?;self.sem_copiar(id,z.a,ctx,matches!(op,Op::Negar(_)))?;
                },
                Op::Rep(_) if z.a==NULO=>self.insertar_sem(id,Significado::default(),ctx)?,
                Op::Par(_,_)|Op::Disy(_,_)|Op::Rep(_)=>{
                    self.sem_evaluar(z.a,ctx,d+1)?;self.sem_evaluar(z.b,ctx,d+1)?;
                    if matches!(op,Op::Disy(_,_)){self.sem_copiar(id,z.a,ctx,false)?;self.sem_copiar(id,z.b,ctx,false)?;}
                    else {
                        let mut a=self.sem_celdas[z.a as usize].primera;
                        while a!=NULO{let x=self.sem_estados[a as usize];let mut b=self.sem_celdas[z.b as usize].primera;
                            while b!=NULO{self.cobrar(5,std::mem::size_of::<Significado>() as u64)?;let y=self.sem_estados[b as usize];let s=x.significado.unir(y.significado)?;self.insertar_sem(id,s,ctx)?;b=y.siguiente;}a=x.siguiente;
                        }
                    }
                }
            }
            f=z.siguiente;
        }
        self.sem_celdas[id as usize].estado=2;Ok(())
    }
    fn significados(&mut self,raiz:u32,ctx:Contexto)->R<()> {
        self.r.reservar(self.c,5,&mut self.sem_celdas,self.nodos.len() as u64)?;
        self.cobrar(7,self.nodos.len() as u64)?;self.sem_celdas.resize(self.nodos.len(),CELDA);
        self.sem_evaluar(raiz,ctx,0)?;let mut a=self.sem_celdas[raiz as usize].primera;
        while a!=NULO{let x=self.sem_estados[a as usize];self.cobrar(5,std::mem::size_of::<Significado>() as u64)?;let s=x.significado.completar(ctx)?;
            let mut igual=false;for i in 0..self.raices.len(){self.cobrar(6,std::mem::size_of::<Significado>() as u64)?;igual|=self.raices[i]==s;}
            if !igual{let nuevos=suma(self.r.estados,1)?;if nuevos>self.c.estados{return Err(Error::Estados)}self.r.poner(self.c,7,&mut self.raices,s,self.c.estados)?;self.r.estados=nuevos;}
            a=x.siguiente;
        }
        // Orden canónico explícito por campos/variantes tipadas; no depende de hashes ni permisos.
        for i in 1..self.raices.len(){let mut j=i;while j>0{self.cobrar(6,std::mem::size_of::<Significado>() as u64)?;if self.raices[j-1]<=self.raices[j]{break}self.raices.swap(j-1,j);j-=1;}}
        Ok(())
    }
}

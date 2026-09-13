import {readFileSync,writeFileSync,mkdirSync,existsSync} from 'node:fs';
import {resolve,dirname,basename,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
// ES: Observador externo; nunca obtiene el esperado del compilador.
// EN: External observer; expected results never come from the compiler.
const home=dirname(fileURLToPath(import.meta.url));
const [exeArg,outArg]=process.argv.slice(2);
if(!exeArg||!outArg)throw Error('uso: node verificar.mjs <sv-native> <directorio-nuevo>');
const exe=resolve(exeArg),out=resolve(outArg);
if(existsSync(out))throw Error('La evidencia previa no se sobrescribe / Previous evidence is not overwritten');
mkdirSync(out,{recursive:true});
const hash=b=>createHash('sha256').update(b).digest('hex');
const bankBytes=readFileSync(join(home,'BANCO_COMPROMETIDO.json'));
const bank=JSON.parse(bankBytes);
function matches(actual,expected){
 if(actual.error||actual.signal||actual.status!==expected.exit_code||actual.stderr!==expected.stderr)return false;
 if('stdout' in expected)return actual.stdout===expected.stdout;
 try{return isDeepStrictEqual(JSON.parse(actual.stdout),expected.json);}catch{return false;}
}
const results=[],observed=new Map();
for(const test of bank.casos){
 const p=join(home,test.fixture), bytes=readFileSync(p);
 if(hash(bytes)!==test.sha256)throw Error('Fixture alterado / Altered fixture: '+test.id);
 const a=spawnSync(exe,['--profile',test.profile,p],{encoding:'utf8',timeout:10000,maxBuffer:1024*1024});
 writeFileSync(join(out,test.id+'.stdout'),a.stdout??'');
 writeFileSync(join(out,test.id+'.stderr'),a.stderr??'');
 observed.set(test.id,a);
 results.push({id:test.id,escenario:test.escenario,conforme:matches(a,test.expected),exit_code:a.status,signal:a.signal,error:a.error?.message??null,stdout_sha256:hash(a.stdout??''),stderr_sha256:hash(a.stderr??'')});
}
// ES: Mutaciones del observable para comprobar sensibilidad, no pruebas de la DSL.
// EN: Observable mutations check sensitivity; these are not DSL tests.
const positive=bank.casos.find(c=>c.id==='b4-exacto'),a=observed.get(positive.id);
const sensitivities=[];
for(const kind of ['posiciones','dimension','identidad','rechazo_con_salida']){
 let changed={...a},expected=positive.expected;
 try{
 if(kind==='rechazo_con_salida'){
  const neg=bank.casos.find(c=>c.id==='b4-longitud-15');
  changed={...observed.get(neg.id),stdout:'{}\n'};expected=neg.expected;
 }else{
  const j=JSON.parse(a.stdout);
  if(kind==='posiciones'){const v=j.objects[3].fields.vector;[v[0],v[1]]=[v[1],v[0]];}
  if(kind==='dimension')j.objects[2].fields.n=9;
  if(kind==='identidad')j.objects[3].fields.spec='Otra';
  changed.stdout=JSON.stringify(j);
 }
 sensitivities.push({mutacion:kind,detectada:!matches(changed,expected)});
 }catch(e){sensitivities.push({mutacion:kind,detectada:false,error:String(e)});}
}
const report={version:'BIS-C01-RESULTADO/1',fecha_utc:new Date().toISOString(),corte_lenguaje:bank.corte_lenguaje,ejecutable:basename(exe),ejecutable_sha256:hash(readFileSync(exe)),banco_sha256:hash(bankBytes),node:process.version,plataforma:process.platform,arquitectura:process.arch,variantes:results.length,conformes:results.filter(r=>r.conforme).length,resultados:results,sensibilidad:sensitivities};
writeFileSync(join(out,'RESULTADO.json'),JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify({variantes:report.variantes,conformes:report.conformes,sensibilidad:sensitivities}));
if(report.conformes!==report.variantes||sensitivities.some(s=>!s.detectada))process.exitCode=1;

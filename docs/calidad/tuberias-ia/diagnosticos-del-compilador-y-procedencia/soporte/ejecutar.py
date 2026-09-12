from pathlib import Path
import subprocess,json,time,hashlib,datetime,sys,shutil
r=Path('tmp/diagnosticos-158');e=r/'evidencia';e.mkdir(exist_ok=True)
rustc=str(Path('tmp/presentacion-150/toolchain/installed/bin/rustc').resolve())
state=json.loads((e/'PROCESOS.json').read_text()) if (e/'PROCESOS.json').exists() else []
def run(label,args,timeout=120):
 start=time.monotonic_ns();utc=datetime.datetime.now(datetime.timezone.utc).isoformat()
 try:
  p=subprocess.run(args,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=timeout);rc=p.returncode;out=p.stdout;err=p.stderr
 except subprocess.TimeoutExpired as x:rc=-1;out=x.stdout or b'';err=x.stderr or b''
 (e/(label+'.stdout')).write_bytes(out);(e/(label+'.stderr')).write_bytes(err)
 row=dict(label=label,argv=args,utc=utc,elapsed_ns=time.monotonic_ns()-start,returncode=rc,stdout_sha256=hashlib.sha256(out).hexdigest(),stderr_sha256=hashlib.sha256(err).hexdigest());state.append(row);(e/'PROCESOS.json').write_text(json.dumps(state,indent=2)+'\n');print(label,rc,flush=True)
 if rc:print(err.decode()[-8000:],out.decode()[-1000:],flush=True);raise SystemExit(rc or 1)
 return out
if __name__=='__main__':
 attempt=sys.argv[1] if len(sys.argv)>1 else '01'
 snap=e/('fuentes-'+attempt);shutil.copytree(r/'candidata',snap,dirs_exist_ok=True)
 for mode,flags in [('debug',[]),('release',['-O'])]:
  for version,source in [('base',Path('tmp/continuacion-157/candidata/rust/sv_core/src/lib.rs')),('new',r/'candidata/rust/sv_core/src/lib.rs')]:
   lib=r/f'libsv_core_{version}_{mode}.rlib'
   run(f'{attempt}-{version}-{mode}-compile',[rustc,'--edition=2021','--crate-name','sv_core','--crate-type','rlib',*flags,str(source),'-o',str(lib)])
   run(f'{attempt}-{version}-{mode}-corpus-build',[rustc,'--edition=2021',*flags,str(r/'corpus.rs'),'--extern',f'sv_core={lib}','-o',str(r/f'corpus-{version}-{mode}')])
   items=json.loads((r/'CORPUS.json').read_text());(r/'corpus.tsv').write_text(''.join(x['expected']+'\t'+str(r/'corpus'/x['path'])+'\n' for x in items))
   run(f'{attempt}-{version}-{mode}-corpus',[str(r/f'corpus-{version}-{mode}'),str(r/'corpus.tsv')])
  assert (e/f'{attempt}-base-{mode}-corpus.stdout').read_bytes()==(e/f'{attempt}-new-{mode}-corpus.stdout').read_bytes(),'legacy drift'
  run(f'{attempt}-{mode}-unit-build',[rustc,'--edition=2021','--test',*flags,str(r/'candidata/rust/sv_core/src/lib.rs'),'-o',str(r/f'units-{mode}')])
  run(f'{attempt}-{mode}-units',[str(r/f'units-{mode}'),'--test-threads=1'])
  run(f'{attempt}-{mode}-focal-build',[rustc,'--edition=2021',*flags,str(r/'focal.rs'),'--extern',f'sv_core={r}/libsv_core_new_{mode}.rlib','-o',str(r/f'focal-{mode}')])
  for rep in range(1,4):run(f'{attempt}-{mode}-focal-{rep}',[str(r/f'focal-{mode}')])
 print('CUALIFICACIÓN COMPLETADA',flush=True)

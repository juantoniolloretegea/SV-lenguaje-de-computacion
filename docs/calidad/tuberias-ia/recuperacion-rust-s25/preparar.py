from pathlib import Path
import sys, json, hashlib, shutil, subprocess
sys.path.insert(0,'bis-c02')
from registro import R, B, L, rows, guardar, retp, now

base=Path('rust-env-s25')
D=R/'docs/calidad/tuberias-ia/recuperacion-rust-s25'
D.mkdir(parents=True,exist_ok=True)
for p in base.iterdir():
    if p.is_file() and p.suffix in ['.py','.sh','.json','.stdout','.stderr'] and not p.name.startswith('PUBLICACION'):
        shutil.copyfile(p,D/p.name)
shutil.copytree(base/'bis-c01-ejecucion',D/'bis-c01-ejecucion',dirs_exist_ok=True)
for name in ['Cargo.toml','src/main.rs','type_error.rs']:
    dest=D/'sonda'/name;dest.parent.mkdir(parents=True,exist_ok=True)
    shutil.copyfile(Path('rust-recovery-check')/name,dest)
shutil.copyfile('/opt/sv-rust-1.98.0/lib/rustlib/install.log',D/'instalacion.log')
shutil.copyfile('rust-1.98.0-x86_64-unknown-linux-gnu.tar.gz.sha256',D/'paquete.sha256')
fuentes=json.loads((B/'consolidacion-bis-02/FUENTES.json').read_text())['fuentes'][:3]
for f in fuentes:assert hashlib.sha256((R/f['ruta']).read_bytes()).hexdigest()==f['sha256']
(D/'FUENTES.json').write_text(json.dumps({'corte_lenguaje':'52fc506a836821b7285834bdc0adf93e9ddc5680','corte_laboratorio':'389ecbaf34a2274455d81da85bc46bbab675816d','rectores':fuentes,'alcance_lectura':'Lecturas completas rectoras previas reutilizadas con identidad cotejada, conforme al workflow §2. AGENTS, reglas de sucesos, condiciones de paso y contrato/observador C01 consultados para este incremento.'},ensure_ascii=False,indent=2)+'\n')
(D/'README.md').write_text('''# S25 · Recuperación de Rust 1.98.0 y activación del entorno

Juan Antonio Lloret Egea y Watson · 13 de septiembre de 2026 · RETP-2026-217

## Resultado y alcance

Instalación autónoma Linux x86_64 en `/opt/sv-rust-1.98.0`, con rustc y Cargo 1.98.0, biblioteca estándar nativa, rustfmt y Clippy. La descarga del paquete completo y su huella oficial pudieron completarse después de fallos de conexión del proxy. No se atribuye una causa resuelta permanentemente a esa intermitencia. El ejecutable Windows aportado antes era rustup-init y no se empleó para esta instalación Linux.

Paquete: https://static.rust-lang.org/dist/rust-1.98.0-x86_64-unknown-linux-gnu.tar.gz

SHA-256 cotejada con el fichero oficial: `aa30409afa67bd1ada244cefd82c7980e6a65bc113bb978e934b2413c75e3900`.

Comando de instalación ejecutado en el turno precedente, conservado aquí como registro posterior:

```sh
bash rust-1.98.0-x86_64-unknown-linux-gnu/install.sh --prefix=/opt/sv-rust-1.98.0 --components=rustc,cargo,rust-std-x86_64-unknown-linux-gnu,rustfmt-preview,clippy-preview --disable-ldconfig
```

El alta de S25 se realiza por petición humana después de iniciada la actividad. No representa un alta previa a la instalación. S23 conserva su cierre histórico. La fecha de inicio registrada corresponde a la configuración observada de este incremento; no se inventa una hora de inicio de la descarga o instalación anterior.

## Activación y comprobación

[env.sh](env.sh) declara `SV_RUST_PREFIX` y añade `bin` a `PATH` sin duplicarlo. [configurar.py](configurar.py) lo copia a la instalación y enlaza su carga desde los archivos de inicio de Bash, conservando su contenido anterior. [CONFIGURACION.json](CONFIGURACION.json) recoge huellas antes/después. No se fijan RUSTUP_HOME ni CARGO_HOME: esta instalación no utiliza rustup y conserva la configuración de Cargo existente.

Nuevas sesiones login y carga de bashrc verificadas. Para una consola que ya estuviera abierta o un ejecutor que no cargue archivos de inicio:

```sh
. /opt/sv-rust-1.98.0/env.sh
rustc --version
cargo --version
```

El directorio global `/usr/local/bin` rechazó la creación de enlaces por permisos en el turno anterior; se emplea la activación de usuario. Persistencia verificada en nuevas sesiones del entorno actual, no garantizada tras sustitución o limpieza del contenedor. El archivo activador y los registros quedan versionados para recuperación.

Sonda Rust edición 2024: compilación y ejecución offline con salida `SV_TOOLCHAIN_OK:100`, una prueba aprobada y rechazo de una cadena asignada a u32 mediante E0308. El programa sólo comprueba instalación; no representa un contrato SV.

## Retorno nativo a Bis

Se compiló el checkout `52fc506a836821b7285834bdc0adf93e9ddc5680`, usando un directorio target nuevo:

```sh
CARGO_TARGET_DIR=/workspace/scratch/cdd3907241dd/rust-env-s25/target-native cargo build --manifest-path rust/Cargo.toml --locked --offline -p sv_native --bin sv-native
```

La primera comprobación con el target del checkout reutilizó artefactos; por ello se realizó la compilación con target nuevo. Ambos registros se conservan. La construcción nueva concluyó con 25 advertencias del núcleo; no se corrigen ni ocultan en este trabajo instrumental. [Registro completo](build-clean.stderr).

Reejecución del banco comprometido C01: **13/13 variantes conformes y 4/4 sensibilidades detectadas**. Se utilizó sin modificaciones el observador Node del repositorio, que invoca al binario Rust y compara esperados ya comprometidos; Node no implementa la semántica. [Resultado](bis-c01-ejecucion/RESULTADO.json) y [montaje/órdenes/retornos](VERIFICACION.json). El campo corte_lenguaje del resultado del observador procede del banco histórico; el corte realmente compilado es el indicado arriba y en VERIFICACION.json. No se sustituye la evidencia C01 anterior ni se suman estas repeticiones como nuevas variantes.

C02–C12: **202 filas preparadas, ninguna ejecutada**. BIS-02 sigue abierto; BIS-03 y GUI conservan su estado pendiente. El siguiente paso sustantivo es comprometer el contrato de integración y banco común C02–C05 con guardas transversales. Esta recuperación permite continuar su trabajo nativo; no constituye tamaños de soporte, autoridad ni paridad visual y no cierra el catálogo.

No se han instalado ni probado nuevos destinos WASM/WASI en esta recuperación. Tampoco se acredita conectividad general de dependencias: esta compilación usó las dependencias disponibles offline.

## Evidencia y trazabilidad

[Fuentes y corte de lectura](FUENTES.json), [script de verificación](verificar.py), [script de preparación](preparar.py), [publicación](publicar.py) y [comprobación del espejo](verificar_publicacion.py). Los scripts administrativos se conservan con sus rutas de ejecución originales; no deben ejecutarse indiscriminadamente desde otra ubicación. Sus usos de Python son configuración, registro y comparación; las sondas y el núcleo son Rust.

La huella del paquete no equivale a una firma criptográfica comprobada. El manifiesto de este expediente identifica sus archivos; el repositorio conserva contenido literal de sondas, órdenes y resultados. El paquete binario de 365 MB no se incorpora al repositorio.
''')
t=now();f,rs=rows();assert rs[-1]['id']=='S24'
conf=json.loads((base/'CONFIGURACION.json').read_text())
start=conf['fecha_utc'].split('.')[0]+'Z'
r=dict.fromkeys(f,'');r.update(id='S25',estado='finalizado',fecha_alta_utc=t,fecha_inicio_utc=start,fecha_actualizacion_utc=t,fecha_fin_utc=t,unidad_responsable='Watson / W-S0',actividad='Recuperación Rust 1.98.0, entorno y comprobación nativa de continuidad',alcance='Instalación autónoma previa, activación idempotente en Bash y comprobación del entorno nativo; repetición C01 sobre el corte vigente.',repositorios_y_ramas='SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente',cortes_de_entrada='Lenguaje 52fc506a836821b7285834bdc0adf93e9ddc5680; laboratorio 389ecbaf34a2274455d81da85bc46bbab675816d',dependencias='S23 histórico; indisponibilidad observada en RETP-216; descarga oficial aportada por el autor.',resultado='Rust y Cargo 1.98.0 operativos; activación en sesiones nuevas; sonda positiva y E0308; sv-native construido desde target nuevo, C01 13/13 y sensibilidad 4/4.',verificacion='Huella oficial cotejada; stdout/stderr y scripts conservados. Compilación --locked --offline; 25 advertencias registradas. Espejo cotejado al publicar.',evidencias='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/recuperacion-rust-s25/README.md',referencia_calidad='RETP-2026-217',siguiente_accion='Continuar S22: contrato y banco integrado C02–C05; comprobar entorno en cada relevo.',observaciones='Alta posterior a actividad iniciada, por petición humana. Inicio corresponde a configuración de esta revisión. No nuevas variantes C01; C02–C12 sin ejecutar; no recuperación WASM/WASI acreditada ni persistencia garantizada tras reemplazo del contenedor.')
rs.append(r);guardar(f,rs,r)
s22=next(x for x in rs if x['id']=='S22');s22.update(fecha_actualizacion_utc=t,verificacion='RETP-217/S25: entorno recuperado y nueva construcción nativa; repetición C01 13/13, sensibilidad 4/4. C02–C12: 202 filas sin ejecutar.',siguiente_accion='Comprometer contrato de integración y banco común C02–C05, con guardas transversales; Rust nativo nuevamente disponible.')
guardar(f,rs,s22)
retp(217,'S25 · Recuperación de Rust y continuidad nativa de Bis',t,r,'tuberias-ia/recuperacion-rust-s25/README.md','RECUPERACION_INSTRUMENTAL','Corte 52fc506; reglas Sucesos; workflow V2 §2; rectores con identidad confirmada','S25 finalizado; S22 continúa; S24 pendiente')
p=B/'ESTADO_WORKFLOW.json';s=json.loads(p.read_text());s['siguiente_accion']=s22['siguiente_accion'];s['consolidacion_bis02']['toolchain_actual']='Recuperado en S25/RETP-217: Rust/Cargo 1.98.0; compilación nativa y C01 repetidos. Sin alterar la comprobación histórica RETP-216.'
s['recuperacion_entorno']={'suceso':'S25','registro':'RETP-2026-217','informe':'../../recuperacion-rust-s25/README.md','prefijo':'/opt/sv-rust-1.98.0','activador':'/opt/sv-rust-1.98.0/env.sh','version':'1.98.0','bis_c01_repetido':13,'nuevas_variantes':0,'c02_c12_ejecutadas':0}
# Relative path: study -> paridad -> tuberias-ia -> recovery folder.
p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
text=L.read_text();needle='## Entrada vigente y continuidad · revisión del 13 de septiembre de 2026\n';text=text.replace(needle,needle+'\n**Actualización instrumental S25 / RETP-2026-217:** [Rust recuperado y entorno activado](../recuperacion-rust-s25/README.md). Rust/Cargo 1.98.0 disponibles; compilación nativa nueva y repetición C01 13/13, sensibilidad 4/4. S23 conserva su evidencia histórica; C02–C12 y la integración de BIS-02 siguen pendientes.\n',1);L.write_text(text)
manifest={str(p.relative_to(D)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(D.rglob('*')) if p.is_file() and p.name!='MANIFIESTO.json'}
(D/'MANIFIESTO.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n')
print('Preparado S25/RETP-217:',len(manifest),'archivos identificados')

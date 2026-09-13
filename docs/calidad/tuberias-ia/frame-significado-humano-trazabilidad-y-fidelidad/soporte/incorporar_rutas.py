"""Añade texto y páginas sin reconstruir las páginas históricas. Ejecutar desde checkout.
Entradas exactas y texto íntegro en INCORPORACION_RUTAS_2026_09_13.json.
Reportlab compone; pypdf conserva las páginas previas. No ejecuta ejemplos Rust.
"""
from pathlib import Path
import json,hashlib,re,html,tempfile
from urllib.parse import urljoin
from pypdf import PdfReader,PdfWriter
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
S=Path(__file__).resolve().parent;R=Path.cwd()
for name,f in [('DV','DejaVuSans.ttf'),('DVB','DejaVuSans-Bold.ttf')]:pdfmetrics.registerFont(TTFont(name,'/usr/share/fonts/truetype/dejavu/'+f))
styles={'p':ParagraphStyle('p',fontName='DV',fontSize=9,leading=13,spaceAfter=8),
'h':ParagraphStyle('h',fontName='DVB',fontSize=14,leading=19,spaceBefore=10,spaceAfter=10,keepWithNext=True)}
report=[]
for item in json.loads((S/'INCORPORACION_RUTAS_2026_09_13.json').read_text()):
 root=R/item['directory'];name=item['name'];old={ext:(root/(name+ext)).read_bytes() for ext in ['.md','.pdf']}
 for ext,b in old.items():assert hashlib.sha256(b).hexdigest()==item['original'][ext]
 tmp=Path(tempfile.mkdtemp(prefix='sv-rutas-pdf-'));(tmp/'original.pdf').write_bytes(old['.pdf'])
 reader=PdfReader(tmp/'original.pdf');count=len(reader.pages)
 base='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/'+item['directory']+'/'
 def inline(t):
  t=html.escape(t)
  return re.sub(r'\[([^]]+)\]\(([^)]+)\)',lambda m:'<link color="#167C80" href="'+html.escape(urljoin(base,html.unescape(m[2])),quote=True)+'">'+m[1]+'</link>',t)
 story=[]
 for block in item['append'].split('\n\n'):
  block=block.strip()
  if not block:continue
  if block.startswith('#'):story.append(Paragraph(inline(block.lstrip('# ')),styles['h']))
  elif block.startswith('- '):
   for line in block.splitlines():story.append(Paragraph(inline(line),styles['p']))
  else:story.append(Paragraph(inline(block.replace('\n',' ')),styles['p']))
 def footer(c,d):
  c.setFont('DV',7);c.drawString(45,815,'SV / CALIDAD / INCORPORACIÓN DOCUMENTAL - 13/09/2026')
  c.drawString(45,26,'Juan Antonio Lloret Egea · Edición Watson · RETP-230')
  c.drawRightString(550,26,str(count+d.page))
 SimpleDocTemplate(str(tmp/'append.pdf'),pagesize=(595.28,841.89),leftMargin=45,rightMargin=45,topMargin=48,bottomMargin=48).build(story,onFirstPage=footer,onLaterPages=footer)
 writer=PdfWriter();writer.append(reader);writer.append(PdfReader(tmp/'append.pdf'));writer.add_metadata({'/Title':reader.metadata.title or name,'/Author':'Juan Antonio Lloret Egea; edición Watson'})
 with (root/(name+'.pdf')).open('wb') as f:writer.write(f)
 (root/(name+'.md')).write_bytes(old['.md']+b'\n\n'+item['append'].encode())
 new=PdfReader(root/(name+'.pdf'))
 assert all(reader.pages[i].extract_text()==new.pages[i].extract_text() for i in range(count))
 assert (root/(name+'.md')).read_bytes().startswith(old['.md'])
 report.append(dict(name=name,previous_pages=count,pages=len(new.pages),original_sha256=item['original'],updated_sha256={ext:hashlib.sha256((root/(name+ext)).read_bytes()).hexdigest() for ext in ['.md','.pdf']},old_pages_text_preserved=True,old_markdown_prefix_preserved=True,original_pdf_local=str(tmp/'original.pdf')))
(S/'VERIFICACION_RUTAS_2026_09_13.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print(json.dumps(report,ensure_ascii=False))

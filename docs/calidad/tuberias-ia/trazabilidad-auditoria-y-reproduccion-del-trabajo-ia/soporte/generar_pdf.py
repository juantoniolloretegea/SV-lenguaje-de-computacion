"""Genera el PDF desde el Markdown de referencia. Requiere reportlab y DejaVu."""
from pathlib import Path
import re, html
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, PageBreak, Preformatted, Table, TableStyle, Image, KeepTogether
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib.colors import HexColor, white
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.enums import TA_LEFT
ROOT=Path(__file__).resolve().parents[1]
NAME='TRAZABILIDAD_AUDITORIA_Y_REPRODUCCION_DEL_TRABAJO_IA_2026_09_12'
FONT=Path('/usr/share/fonts/truetype/dejavu')
for name,file in [('DV','DejaVuSans.ttf'),('DV-B','DejaVuSans-Bold.ttf'),('Mono','DejaVuSansMono.ttf')]:
 pdfmetrics.registerFont(TTFont(name,str(FONT/file)))
pdfmetrics.registerFontFamily('DV',normal='DV',bold='DV-B',italic='DV',boldItalic='DV-B')
navy=HexColor('#15394B');teal=HexColor('#167C80');gray=HexColor('#54646B')
styles={
 'body':ParagraphStyle('body',fontName='DV',fontSize=9.2,leading=13.8,spaceAfter=7,textColor=navy),
 'h1':ParagraphStyle('h1',fontName='DV-B',fontSize=23,leading=29,spaceAfter=15,textColor=navy),
 'h2':ParagraphStyle('h2',fontName='DV-B',fontSize=14,leading=19,spaceBefore=8,spaceAfter=10,textColor=navy,keepWithNext=True),
 'h3':ParagraphStyle('h3',fontName='DV-B',fontSize=10.5,leading=15,spaceBefore=7,spaceAfter=7,textColor=teal,keepWithNext=True),
 'code':ParagraphStyle('code',fontName='Mono',fontSize=8.1,leading=10.6,backColor=HexColor('#F0F4F5'),borderPadding=8,spaceAfter=10),
 'cell':ParagraphStyle('cell',fontName='DV',fontSize=8.3,leading=11.5,textColor=navy),
 'headcell':ParagraphStyle('headcell',fontName='DV-B',fontSize=8.3,leading=11.5,textColor=white),
}
baseurl='https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/'
def inline(s):
 s=html.escape(s)
 s=re.sub(r'\[([^\]]+)\]\(([^)]+)\)',lambda m:'<link href="'+html.escape(baseurl+m[2][3:] if m[2].startswith('../') else m[2],quote=True)+'" color="#167C80">'+m[1]+'</link>',s)
 s=re.sub(r'\*\*([^*]+)\*\*',r'<b>\1</b>',s)
 s=re.sub(r'`([^`]+)`',r'<font name="Mono" size="8">\1</font>',s)
 return s
lines=(ROOT/(NAME+'.md')).read_text().splitlines();story=[];i=0
while i<len(lines):
 line=lines[i].strip()
 if not line:i+=1;continue
 if line=='<!-- pagebreak -->':story.append(PageBreak());i+=1;continue
 if line.startswith('```'):
  code=[];i+=1
  while i<len(lines) and not lines[i].startswith('```'):code.append(lines[i]);i+=1
  story.append(Preformatted('\n'.join(code),styles['code']));i+=1;continue
 if line.startswith('|'):
  rows=[]
  while i<len(lines) and lines[i].strip().startswith('|'):
   cells=[x.strip() for x in lines[i].strip().strip('|').split('|')]
   if not all(re.match(r'^[-: ]+$',x) for x in cells):rows.append(cells)
   i+=1
  data=[[Paragraph(inline(c),styles['headcell' if n==0 else 'cell']) for c in row] for n,row in enumerate(rows)]
  t=Table(data,colWidths=[108,391],repeatRows=1,hAlign='LEFT')
  t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),navy),('VALIGN',(0,0),(-1,-1),'TOP'),('LEFTPADDING',(0,0),(-1,-1),8),('RIGHTPADDING',(0,0),(-1,-1),8),('TOPPADDING',(0,0),(-1,-1),6),('BOTTOMPADDING',(0,0),(-1,-1),6),('ROWBACKGROUNDS',(0,1),(-1,-1),[HexColor('#F1F5F6'),white]),('LINEBELOW',(0,0),(-1,-1),0.25,HexColor('#CEDBDE'))]))
  story.extend([t,Spacer(1,10)]);continue
 if line.startswith('!['):
  m=re.match(r'!\[([^]]*)\]\(([^)]+)\)',line);p=ROOT/m[2]
  from PIL import Image as PILImage
  w,h=PILImage.open(p).size
  maxw,maxh=499,220
  if '03-rust' in p.name:maxw,maxh=180,248
  if '01-procesos' in p.name:maxh=205
  if '02-pregunta' in p.name:maxh=250
  scale=min(maxw/w,maxh/h)
  story.extend([Image(str(p),w*scale,h*scale,hAlign='LEFT'),Spacer(1,8)]);i+=1;continue
 if line.startswith('#'):
  n=len(line)-len(line.lstrip('#'));story.append(Paragraph(inline(line[n:].strip()),styles['h'+str(min(n,3))]));i+=1;continue
 para=[line];i+=1
 while i<len(lines) and lines[i].strip() and not lines[i].startswith(('#','|','```','![','<!--')):
  if re.match(r'^(\d+\. |- )',lines[i]):break
  para.append(lines[i].strip());i+=1
 story.append(Paragraph(inline(' '.join(para)),styles['body']))
def footer(c,d):
 c.saveState();c.setStrokeColor(teal);c.setLineWidth(1);c.line(48,794,547,794)
 c.setFont('DV',7.4);c.setFillColor(gray);c.drawString(48,805,'SISTEMA VECTORIAL SV  /  CALIDAD  /  REFERENCIA DE CONTINUIDAD')
 c.line(48,42,547,42);c.drawString(48,29,'Juan Antonio Lloret Egea · Edición Watson · 12/09/2026');c.drawRightString(547,29,str(d.page));c.restoreState()
doc=SimpleDocTemplate(str(ROOT/(NAME+'.pdf')),pagesize=(595.28,841.89),rightMargin=48,leftMargin=48,topMargin=62,bottomMargin=58,title='Trazabilidad, auditoría y reproducción del trabajo de la IA',author='Juan Antonio Lloret Egea; preparación editorial Watson')
doc.build(story,onFirstPage=footer,onLaterPages=footer)
print(ROOT/(NAME+'.pdf'))

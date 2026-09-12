import sys,json
sys.path.insert(0,'/root/.codex/plugins/cache/openai-curated-remote/openai-library/0.1.55/skills/library/scripts')
from library_hosted_apps import HostedAppsClient
client=HostedAppsClient()
CONNECTOR='connector_76869538009648d5b282a4bb21c3d157'
def call(name,args):
 r=client.call_tool(CONNECTOR,name,args)
 s=r.get('structuredContent')
 if s is None:
  for t in r.get('content',[]):
   if t.get('type')=='text':
    try:s=json.loads(t['text']);break
    except ValueError:pass
 if not isinstance(s,dict):raise RuntimeError('Missing structured response: '+name)
 return s.get('result',s)
def fetch(url):
 r=call('fetch',{'url':url})
 return json.loads(r['content'])

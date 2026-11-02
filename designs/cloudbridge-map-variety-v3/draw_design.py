"""Design-only SVG authoring; no runtime files are edited."""
import json, html, itertools, math
from pathlib import Path
ROOT=Path(__file__).resolve().parent
maps=[
 dict(n=1,title='双岛断桥',nodes={'A':[0,0],'B':[6,0]},edges=[['A','B']],start='A',goal='B',wood=3,stone=0,winds=[],gem=None,required=False,tip='两座岛，一条直桥；先学会搭桥和亲自行走。'),
 dict(n=2,title='四岛分岔',nodes={'A':[0,0],'B':[0,-6],'C':[6,0],'D':[6,-6]},edges=[['A','B'],['B','D'],['A','C'],['C','D']],start='A',goal='D',wood=6,stone=0,winds=[],gem='B',required=False,tip='四座岛形成两条支路；选一条，不能全部铺满。'),
 dict(n=3,title='转角栈道',nodes={'A':[0,0],'B':[6,0],'C':[6,6],'D':[12,6]},edges=[['A','B'],['B','C'],['C','D']],start='A',goal='D',wood=8,stone=1,winds=[],gem=None,required=False,tip='折线穿过四座岛；学会转弯，混合使用建材。'),
 dict(n=4,title='风口岔路',nodes={'A':[0,0],'J':[6,0],'G':[6,-6],'C':[6,6]},edges=[['A','J'],['J','G'],['J','C']],start='A',goal='G',wood=7,stone=2,winds=[0,1],gem='C',required=False,tip='T形岔路有两处风口；南侧支路可额外取星。'),
 dict(n=5,title='环岛寻星',nodes={'A':[0,0],'B':[0,-6],'C':[6,-6],'D':[12,-6],'E':[12,0],'F':[6,0]},edges=[['A','B'],['B','C'],['C','D'],['D','E'],['E','F'],['F','A']],start='A',goal='D',wood=8,stone=1,winds=[1],gem='B',required=False,tip='六岛环线，两条完整路线；比较风口与取星路线。'),
 dict(n=6,title='中枢远航',nodes={'A':[-6,0],'J':[0,0],'B':[0,-6],'C':[6,0],'D':[0,6],'G':[6,6]},edges=[['A','J'],['J','B'],['J','C'],['J','D'],['C','G'],['D','G']],start='A',goal='G',wood=10,stone=2,winds=[0,4,5],gem='B',required=True,tip='先到北岛取星，再回中枢选路；需要折返与规划。'),
]

def cells(m,e):
 a,b=(m['nodes'][v] for v in e);dx=0 if a[0]==b[0] else (1 if b[0]>a[0] else -1);dz=0 if a[1]==b[1] else (1 if b[1]>a[1] else -1)
 assert dx==0 or dz==0
 dist=abs(b[0]-a[0])+abs(b[1]-a[1]);assert dist==6
 return [(a[0]+dx*i,a[1]+dz*i) for i in range(2,dist-1)]

for m in maps:
 solutions=[]
 for bits in itertools.product([0,1],repeat=len(m['edges'])):
  selected=[i for i,b in enumerate(bits) if b];adj={n:[] for n in m['nodes']}
  for i in selected:
   a,b=m['edges'][i];adj[a].append(b);adj[b].append(a)
  seen={m['start']};todo=list(seen)
  while todo:
   for v in adj[todo.pop()]:
    if v not in seen:seen.add(v);todo.append(v)
  gaps=sum(len(cells(m,m['edges'][i])) for i in selected);need_stone=sum(i in m['winds'] for i in selected)
  if m['goal'] in seen and (not m['required'] or m['gem'] in seen) and gaps<=m['wood']+m['stone'] and need_stone<=m['stone']:
   solutions.append({'edges':selected,'gaps':gaps,'minimum_stone':need_stone})
 assert solutions,m['title'];m['checked_build_solutions']=solutions
(ROOT/'maps.json').write_text(json.dumps(maps,ensure_ascii=False,indent=2))

D='''<defs><linearGradient id="sky" x2="0" y2="1"><stop stop-color="#d9f3ff"/><stop offset="1" stop-color="#f4fbff"/></linearGradient><linearGradient id="grass" x2="1" y2="1"><stop stop-color="#a9db5d"/><stop offset="1" stop-color="#57ad61"/></linearGradient><linearGradient id="rock" x2="1" y2="1"><stop stop-color="#c9ad81"/><stop offset="1" stop-color="#7e8b91"/></linearGradient></defs>'''

def text(x,y,t,size=22,color='#183e54',weight=500,anchor='start'):
 return f'<text x="{x}" y="{y}" font-size="{size}" fill="{color}" font-weight="{weight}" text-anchor="{anchor}">{html.escape(str(t))}</text>'
def polygon(pts,fill,stroke='none',width=1,extra=''):
 return f'<polygon points="{" ".join(f"{x:.2f},{y:.2f}" for x,y in pts)}" fill="{fill}" stroke="{stroke}" stroke-width="{width}" stroke-linejoin="round" {extra}/>'
def star(x,y,r=16):
 pts=[(x+math.sin(i*math.pi/5)*(r if i%2==0 else r*.46),y-math.cos(i*math.pi/5)*(r if i%2==0 else r*.46)) for i in range(10)]
 return polygon(pts,'#ffd657','#e8a624',2)

def drawmap(m,box,built=None,show_meta=True):
 x,y,w,h=box
 raw=lambda a,b: (a-b,(a+b)*.48)
 allp=[raw(a+dx,b+dz) for a,b in m['nodes'].values() for dx,dz in [(-1.5,-1.5),(-1.5,1.5),(1.5,-1.5),(1.5,1.5)]]
 mnx=min(p[0] for p in allp);mxx=max(p[0] for p in allp);mny=min(p[1] for p in allp);mxy=max(p[1] for p in allp)
 u=min((w-65)/(mxx-mnx),(h-55)/(mxy-mny+3.0));ox=x+(w-(mxx-mnx)*u)/2-mnx*u;oy=y+18-mny*u
 def project(a,b):
  px,py=raw(a,b);return ox+px*u,oy+py*u
 def tile(a,b):return [project(a-.5,b-.5),project(a+.5,b-.5),project(a+.5,b+.5),project(a-.5,b+.5)]
 out=[]
 # The grass and bridge surfaces share one projection and one height.
 for name,(a,b) in sorted(m['nodes'].items(),key=lambda q:sum(q[1])):
  cap=[project(a-1.5,b-1.5),project(a+1.5,b-1.5),project(a+1.5,b+1.5),project(a-1.5,b+1.5)]
  cx,cy=project(a,b);low=(cx+u*.12,cy+u*3.0)
  out.append(polygon([cap[3],cap[2],cap[1],(cap[1][0]-.7*u,cap[1][1]+1.8*u),low,(cap[3][0]+.7*u,cap[3][1]+1.8*u)],'url(#rock)','#9c987e',1.3))
  for k in [-.6,0,.6]:out.append(f'<path d="M {cx+k*u:.2f} {cy+u*.65:.2f} L {low[0]+k*u*.4:.2f} {low[1]-.1*u:.2f}" stroke="#ad9778" stroke-width="{u*.12:.2f}"/>')
 for ei,e in enumerate(m['edges']):
  for j,(a,b) in enumerate(cells(m,e)):
   pts=tile(a,b);filled=built is None or ei in built
   if filled:
    color='#99a7b1' if ei in m['winds'] and j==1 else '#d5a56d'
    side=[pts[3],pts[2],(pts[2][0],pts[2][1]+u*.22),(pts[3][0],pts[3][1]+u*.22)]
    out.append(polygon(side,'#a5784f','#8b6c4c',1));out.append(polygon(pts,color,'#8f6c43',1.2))
    # Plank joints stay on the exact same flat surface, never form a staircase.
    pa=project(a-.34,b-.5);pb=project(a-.34,b+.5);out.append(f'<path d="M {pa[0]} {pa[1]} L {pb[0]} {pb[1]}" stroke="#e9c496" stroke-width="1.4"/>')
   else:out.append(polygon(pts,'#eefbff','#5cbae4',1.7,'stroke-dasharray="5 4"'))
   if ei in m['winds'] and j==1:
    px,py=project(a,b);out.append(f'<path d="M {px-11} {py-5} Q {px+8} {py-17} {px+13} {py-4} M {px-8} {py+4} Q {px+10} {py-8} {px+16} {py+3}" fill="none" stroke="#078dbd" stroke-width="2.3" stroke-linecap="round"/>')
 for name,(a,b) in sorted(m['nodes'].items(),key=lambda q:sum(q[1])):
  cap=[project(a-1.5,b-1.5),project(a+1.5,b-1.5),project(a+1.5,b+1.5),project(a-1.5,b+1.5)]
  out.append(polygon(cap,'url(#grass)','#47945a',1.3))
  for k in [-.5,.5]:
   p0=project(a-1.5,b+k);p1=project(a+1.5,b+k);p2=project(a+k,b-1.5);p3=project(a+k,b+1.5)
   out.append(f'<path d="M {p0[0]} {p0[1]} L {p1[0]} {p1[1]} M {p2[0]} {p2[1]} L {p3[0]} {p3[1]}" stroke="#b8e19c" stroke-width="1" opacity=".7"/>')
  cx,cy=project(a,b)
  if name==m['start']:
   out.append(f'<rect x="{cx-10}" y="{cy-24}" width="20" height="22" rx="4" fill="#ffd951" stroke="#be9924" stroke-width="1.4"/><rect x="{cx-7}" y="{cy-18}" width="14" height="9" rx="2" fill="#184c65"/>')
   for dx in [-3,3]:out.append(f'<rect x="{cx+dx-1}" y="{cy-16}" width="2" height="5" fill="#65e4f6"/>')
  if name==m['goal']:out.append(star(cx+u*.2,cy-u*.5,max(13,u*.48)))
  if name==m['gem']:
   px=cx-u*.4;py=cy-u*.6
   out.append(polygon([(px,py-13),(px+9,py-2),(px+6,py+11),(px-6,py+11),(px-9,py-2)],'#58d6f8','#178ab2',1.4))
 return ''.join(out)

def svg(w,h,content):return f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><style>text{{font-family:"PingFang SC","Heiti SC",sans-serif}}</style>{D}{content}</svg>'
board=['<rect width="1960" height="1180" fill="#edf6fa"/>',text(40,53,'云桥工程师 · 六关独立地图布局稿',33,weight=700),text(40,88,'岛屿数量、连接关系与路线都变化；桥面由同一网格生成，所有落脚点在同一高度。',22)]
for i,m in enumerate(maps):
 x=30+(i%3)*645;y=115+(i//3)*500
 board.append(f'<rect x="{x}" y="{y}" width="625" height="478" rx="22" fill="url(#sky)" stroke="#cbdee7"/>')
 board.append(text(x+22,y+40,f'{m["n"]:02d}  {m["title"]}',27,weight=700))
 board.append(text(x+22,y+71,f'{len(m["nodes"])}座岛 · {len(m["edges"])}段路线 · 木桥{m["wood"]} / 石桥{m["stone"]}',20))
 board.append(drawmap(m,(x+12,y+85,601,310)))
 board.append(text(x+22,y+440,m['tip'][:26],20))
 board.append(text(x+22,y+467,m['tip'][26:],20))
board.extend([text(40,1150,'布局示意：棕色表示可搭桥路段，非开局全部铺好；金星=终点，机器人=起点，蓝晶=收集物，蓝风纹=石桥格。',21)])
(ROOT/'six-maps.svg').write_text(svg(1960,1180,''.join(board)))

m=maps[1]
ui=['<rect width="1280" height="720" fill="url(#sky)"/>']
ui.append(drawmap(m,(280,110,770,460),built={0,1}))
ui.append('<rect x="20" y="16" width="1240" height="65" rx="24" fill="#fffdf5"/>')
ui.extend([text(44,59,'←',33),text(110,58,'云桥工程师',32,weight=700),text(800,56,'第2关 · 四岛分岔',23),text(1145,56,'? 怎么玩',22)])
for i in range(6):
 cx=435+i*48;fill='#0daf69' if i==0 else '#f2aa27' if i==1 else '#adbdc7';ui.append(f'<circle cx="{cx}" cy="48" r="16" fill="{fill}"/>');ui.append(text(cx,55,str(i+1),19,'white',600,'middle'))
ui.append('<rect x="22" y="105" width="250" height="132" rx="23" fill="#fffdf5"/>');ui.extend([text(42,140,'本关目标',26,weight=700),text(42,177,'找到终点，可选取星晶',20),text(42,212,'当前：过桥模式',21,'#1682b4')])
ui.append('<rect x="1092" y="106" width="166" height="452" rx="25" fill="#fffdf5"/>');ui.append(text(1175,146,'剩余建材',23,weight=700,anchor='middle'))
for i,(name,num) in enumerate([('木桥',0),('石桥',0)]):
 yy=176+i*150;ui.append(f'<rect x="1108" y="{yy}" width="134" height="130" rx="18" fill="#f4f7f6" stroke="#b9d4e0"/>')
 ui.append(f'<rect x="1142" y="{yy+20}" width="67" height="50" rx="7" fill="{"#d8af78" if i==0 else "#a1adb5"}"/>');ui.append(text(1175,yy+106,f'{name} ×{num}',22,anchor='middle'))
ui.append('<rect x="20" y="591" width="1240" height="111" rx="26" fill="#fffdf5"/>');ui.extend([text(42,633,'亲自带机器人到终点',26,weight=700),text(42,669,'点方向移动，缺口和风口会拦住',18)])
for i,(dx,dz,label) in enumerate([(-1,0,'左上'),(0,-1,'右上'),(0,1,'左下'),(1,0,'右下')]):
 x=435+i*130;y=607;ui.append(f'<rect x="{x}" y="{y}" width="112" height="79" rx="18" fill="#e3f3ff" stroke="#8ec8ed"/>')
 sx=dx-dz;sy=dx+dz;cx=x+56;cy=y+28;ex=cx+sx*15;ey=cy+sy*13;bx=cx-sx*11;by=cy-sy*10
 ui.append(f'<path d="M {bx} {by} L {ex} {ey} M {ex-sx*12} {ey} L {ex} {ey} L {ex} {ey-sy*12}" fill="none" stroke="#087ee6" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/>');ui.append(text(cx,y+65,label,18,anchor='middle'))
ui.append('<rect x="989" y="607" width="248" height="79" rx="19" fill="#087ef5"/>');ui.append(text(1113,657,'返回搭桥',28,'white',600,'middle'))
(ROOT/'manual-crossing.svg').write_text(svg(1280,720,''.join(ui)))
print('Design-only files created; all six maps have resource-feasible routes.')

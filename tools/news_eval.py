"""신문 괄호 병기 쌍(金鍾泌(김종필))으로 변환기를 잰다.
입력: news_hanja_pairs.tsv (한자, 한글, 출현). 글자마다 한글 음절이 그 한자의 알려진 독음(대표·다른 독음·두음 꼴)이어야 쓴다.
"""
import sys, collections, unicodedata
sys.path.insert(0, 'tools')
import demo   # 기준 변환기 (cv, convert, prim ...)
from hanja_ref import nfc

SURNAMES = set('金李朴崔鄭姜趙尹張林韓吳徐申權黃安宋柳洪全高文孫梁裵白曺許劉南沈盧河丁成郭車兪具禹朱任羅辛閔陳池嚴蔡元千方孔玄咸卞楊廉邊呂秋魯都蘇愼石宣薛馬吉周延表明魏潘王琴玉陸印孟諸卓秦慶睦太奇殷片龍芮邢慶偰鞠陰甘')
alts = collections.defaultdict(set)
for line in open('/home/claude/hanja_hangul/data/alts_nikl.csv', encoding='utf-8'):
    if not line.startswith('#'):
        k, v = line.rstrip('\n').split(',', 1); alts[k].update(v.split())
undu = collections.defaultdict(set)
for k, v in demo.dueum.items(): undu[v].add(k)
def ok_syl(c, s):
    rs = {demo.prim.get(c)} | alts[c]
    rs |= {demo.dueum.get(r, r) for r in rs if r}
    return s in rs

pairs = collections.Counter()
skipped = collections.Counter()
for line in open(sys.argv[1], encoding='utf-8'):
    if line.startswith('#'): continue
    h, k, n = line.rstrip('\n').split('\t'); n = int(n); h = nfc(h)
    if len(h) != len(k) or not all(ok_syl(c, s) for c, s in zip(h, k)):
        skipped[(h, k)] += n; continue
    pairs[(h, k)] += n
# 같은 한자에 한글이 둘 이상이면 많은 쪽만 정답 (表記 오기 걸러내기)
best = {}
for (h, k), n in pairs.items():
    if h not in best or n > pairs[(h, best[h])]: best[h] = k
tot_h = collections.Counter()
for (h, k), n in pairs.items(): tot_h[h] += n
gold = {h: (k, tot_h[h]) for h, k in best.items()}

def kind(h):
    return '인명(성+2자)' if len(h) == 3 and h[0] in SURNAMES else ('2자' if len(h) == 2 else '기타')

def evaluate(conv, label, show=0):
    tot = collections.Counter(); ok = collections.Counter(); typ = collections.Counter(); oktyp = collections.Counter()
    errs = []
    for h, (k, n) in gold.items():
        out = conv(h)
        g = kind(h)
        tot[g] += n; typ[g] += 1
        if out == k: ok[g] += n; oktyp[g] += 1
        else: errs.append((n, h, out, k))
    T, O = sum(tot.values()), sum(ok.values())
    print(f'{label}: 출현 가중 {O/T:.2%} ({O:,}/{T:,}), 종류 {sum(oktyp.values())/sum(typ.values()):.2%} ({sum(oktyp.values()):,}/{sum(typ.values()):,})  |  ' +
          '  '.join(f'{g} {ok[g]/tot[g]:.1%}' for g in sorted(tot)))
    errs.sort(reverse=True)
    for e in errs[:show]: print('   ', *e)
    return errs

if __name__ == '__main__':
    print(f'쌍 {len(pairs):,}종 / 버림 {len(skipped):,}종 (외국어 독음·잘림: ' + ' '.join(f'{h}→{k}' for (h, k), _ in skipped.most_common(8)) + ')')
    print(f'정답 한자 {len(gold):,}종, 출현 {sum(n for _, n in gold.values()):,}; 종류별 ' + str(collections.Counter(kind(h) for h in gold)))
    evaluate(demo.convert, '0.2', show=40)

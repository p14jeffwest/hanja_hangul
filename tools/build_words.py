"""예외표 생성 + 평가 (hanja_hangul 0.2 1단계).

입력: out/nikl_hanja.tsv.gz (extract_nikl.py), refs/hanja/{hanja_char,dueum,hanja_word}.csv
출력: data/words_nikl.csv      예외표 (한자,한글,출처)          — 전체 표제어로 생성
      data/primary_changes.tsv 대표 독음을 바꾼 글자 (검토용)
      data/chars_ext.csv       글자표에 없던 글자 (사전 정렬로 유도)
      data/alts_nikl.csv       글자별 다른 독음 (사전 정렬로 확인된 것)
      data/surname_words.csv   金·沈·葉·契·單 로 시작하는 표제어 전부
      out/report.md            통계·평가
"""
import collections, gzip, hashlib, os, re, sys
from hanja_ref import Converter, load_chars, load_pairs, nfc, is_hanja, is_syllable

REFS = sys.argv[1]            # refs/hanja 디렉터리
NIKL = sys.argv[2]            # out/nikl_hanja.tsv.gz
DATA = sys.argv[3]            # 출력 data 디렉터리
OUT = sys.argv[4]             # 보고 디렉터리
os.makedirs(DATA, exist_ok=True)
os.makedirs(OUT, exist_ok=True)

SURNAME = {'金': '김', '沈': '심', '葉': '섭', '契': '글', '單': '선'}
SURNAME_PLAIN = {'金': '금', '沈': '침', '葉': '엽', '契': '계', '單': '단'}
FIXED = set(SURNAME) | {'不', '茶'}     # 자동으로 대표 독음을 바꾸지 않는 글자 (茶: 한 글자 '차' 가 흔함, 10.15 Jeff)
UNITS = {'단어', '어휘', '구'}

rep = []
def say(*a):
    s = ' '.join(str(x) for x in a)
    print(s)
    rep.append(s)

# ---------- 1. 표제어 읽기 ----------
prim0, alts0 = load_chars(os.path.join(REFS, 'hanja_char.csv'))
dueum = load_pairs(os.path.join(REFS, 'dueum.csv'))
hand = load_pairs(os.path.join(REFS, 'hanja_word.csv'))
undueum = collections.defaultdict(set)
for k, v in dueum.items():
    undueum[v].add(k)

readings = collections.defaultdict(lambda: collections.Counter())   # key → reading → count
srcs = collections.defaultdict(set)                                  # (key, reading) → {S,U}
stat = collections.Counter()
lenmis = collections.Counter()
marked = set()                                                       # (key, reading) — ▽ 표시 (음이 변한 한자)
EQU = re.compile(r'<(?:equ|EQU)>(?:&#x([0-9A-Fa-f]+);|([^<]*))</(?:equ|EQU)>')
def equ(hexv, lit):
    if hexv:
        o = int(hexv, 16)
        return '\x00' if 0xE000 <= o <= 0xF8FF else chr(o)   # 사용자 정의 영역은 버림
    return lit
with gzip.open(NIKL, 'rt', encoding='utf-8') as f:
    next(f)
    for line in f:
        src, code, word, hangul, hanja, langs, unit, types, pos = line.rstrip('\n').split('\t')
        stat[src + ' 전체'] += 1
        if unit not in UNITS:
            stat[src + ' 제외: 단위(' + unit + ')'] += 1
            continue
        if '일반어' not in types.split(','):
            stat[src + ' 제외: 일반어 아님'] += 1
            continue
        if set(langs.split('+')) - {'한자', '/(병기)'}:
            stat[src + ' 혼용(2차)'] += 1
            continue
        if '←' in hanja or '<hanja>' in hanja:
            stat[src + ' 제외: 줄임말·그림 한자'] += 1
            continue
        changed = '▽' in hanja
        h = EQU.sub(lambda m: equ(m.group(1), m.group(2)), hanja).replace('▽', '').replace(' ', '')
        alts_h = [nfc(x) for x in h.split('/') if x]
        alts_h = [x for x in alts_h if all(is_hanja(c) for c in x)]
        if not alts_h:
            stat[src + ' 제외: 한자 아닌 글자'] += 1
            continue
        if not hangul or not all(is_syllable(c) for c in hangul):
            stat[src + ' 제외: 표제어에 한글 음절 아닌 것'] += 1
            continue
        good = [x for x in alts_h if len(x) == len(hangul)]
        if not good:
            stat[src + ' 제외: 길이 다름'] += 1
            lenmis[(alts_h[0], hangul)] += 1
            continue
        if changed:
            stat[src + ' 그중 ▽(음이 변한 한자)'] += 1
        if len(alts_h) > 1:
            stat[src + ' 그중 / 병기'] += 1
        for key in good:
            readings[key][hangul] += 1
            srcs[(key, hangul)].add(src)
            if changed:
                marked.add((key, hangul))
        stat[src + ' 채택'] += 1

say('## 1. 표제어')
for k in sorted(stat):
    say(f'- {k}: {stat[k]:,}')
say(f'- 서로 다른 순한자 키: {len(readings):,} (표준에 있는 키 {sum(1 for k in readings if any("S" in srcs[(k, r)] for r in readings[k])):,})')
say(f'- 독음이 둘 이상인 키: {sum(1 for k in readings if len(readings[k]) > 1):,}')
say('- 길이 다름 보기: ' + ' '.join(f'{k}→{h}' for (k, h), _ in lenmis.most_common(12)))

# ---------- 2. 글자-독음 정렬 ----------
mid = collections.defaultdict(collections.Counter)    # 둘째 글자 이후 (두음 무관)
ini = collections.defaultdict(collections.Counter)    # 첫 글자 (두음 적용된 꼴)
for key, rs in readings.items():
    for r in rs:
        for i, (c, s) in enumerate(zip(key, r)):
            (ini if i == 0 else mid)[c][s] += 1

def char_readings(c):
    s = set()
    if c in prim0:
        s.add(prim0[c])
    s.update(alts0.get(c, []))
    s.update(r for r, n in mid[c].items() if n >= 2)
    for r, n in ini[c].items():
        if n >= 2:
            s.add(r)
            s.update(undueum.get(r, ()))
    return s

# ---------- 3. 대표 독음 손질 ----------
prim = dict(prim0)
changes = []
for c, cnt in mid.items():
    if c in FIXED or c not in prim:
        continue
    total = sum(cnt.values())
    top, ct = cnt.most_common(1)[0]
    cp = cnt.get(prim[c], 0)
    # 옛 대표에 두음을 적용한 꼴이 새 대표면 바꾸지 않는다: 어중 집계가 합성어 안 두음(○○聯合 의 연)에 오염된 것
    if dueum.get(prim[c]) == top:
        continue
    if total >= 5 and top != prim[c] and ct >= 2 * max(cp, 1) and ct >= 5:
        changes.append((c, prim[c], top, cp, ct, total))
        prim[c] = top
for c, r in SURNAME_PLAIN.items():
    if prim.get(c) != r:
        changes.append((c, prim.get(c), r, mid[c].get(prim.get(c), 0), mid[c].get(r, 0), sum(mid[c].values())))
        prim[c] = r
prim['不'] = '불'   # 不 규칙이 ㄷ·ㅈ 앞에서 부 로 바꾼다
changes.sort(key=lambda x: -x[5])
with open(os.path.join(DATA, 'primary_override.csv'), 'w', encoding='utf-8') as w:
    w.write('# 대표 독음 덮어쓰기 (hanja_char.csv 위에 얹는다; 10.15 Jeff 승인). 근거: 국립국어원 표준국어대사전·우리말샘(CC BY-SA 2.0 KR) '
            '순한자 표제어의 둘째 글자 이후 자리에서 읽힌 독음 집계.\n')
    w.write('# 한자,새 대표,옛 대표,새 대표 어중 표제어 수,옛 대표 어중 표제어 수,어중 표제어 수\n')
    for c, a, nb_, na, nb, t in changes:
        w.write(f'{c},{nb_},{a},{nb},{na},{t}\n')

# 글자표에 없던 글자
ext = {}
for c in set(mid) | set(ini):
    if c in prim:
        continue
    if mid[c]:
        r, n = mid[c].most_common(1)[0]
    else:
        r0, n = ini[c].most_common(1)[0]
        cand = [x for x in undueum.get(r0, ())]
        r = r0 if not cand else sorted(cand)[0]   # 첫머리 꼴만 있으면 두음 이전 꼴을 대표로
    ext[c] = (r, n)
for c, (r, n) in ext.items():
    prim[c] = r
PLAIN = Converter(prim, dueum)
with open(os.path.join(DATA, 'chars_ext.csv'), 'w', encoding='utf-8') as w:
    w.write('# 국립국어원 표준국어대사전·우리말샘(CC BY-SA 2.0 KR) 표제어 정렬로 유도한 독음. 한자,한글,근거 표제어 수\n')
    for c in sorted(ext, key=ord):
        w.write(f'{c},{ext[c][0]},{ext[c][1]}\n')

alts = {}
for c in set(mid) | set(ini) | set(alts0):
    rs = sorted(char_readings(c) - {prim.get(c)})
    if rs:
        alts[c] = rs
with open(os.path.join(DATA, 'alts_nikl.csv'), 'w', encoding='utf-8') as w:
    w.write('# 글자별 다른 독음 (대표 제외). 출처: hanja_char.csv 호환 한자 행 + 국립국어원 사전 표제어 정렬(2회 이상). 한자,독음들(공백)\n')
    for c in sorted(alts, key=ord):
        w.write(f'{c},{" ".join(alts[c])}\n')

say('\n## 2. 글자표')
say(f'- 대표 독음 바꿈: {len(changes)}자 (data/primary_override.csv). 어중 표제어 수 상위:')
say('  ' + ' '.join(f'{c}:{a}→{b}({na}→{nb}/{t})' for c, a, b, na, nb, t in changes[:30]))
say(f'- 글자표에 없던 글자 추가: {len(ext)}자 (data/chars_ext.csv): ' + ' '.join(f'{c}{r}' for c, (r, n) in sorted(ext.items(), key=lambda x: -x[1][1])[:40]))
say(f'- 다른 독음이 있는 글자: {len(alts):,} (예전 호환 한자 행 기준 {len(alts0)})')

# ---------- 4. 예외표 생성 (고정점) ----------
FREQ = {}
if len(sys.argv) > 5:
    for line in open(sys.argv[5], encoding='utf-8'):
        if not line.startswith('#'):
            f_, n_ = line.rstrip('\n').split('\t')
            FREQ[f_] = int(n_)

# PLAIN: 예외표 없는 글자별 변환기 (대표 독음 확정 직후에 만든다)

def accepted(key):
    """맞는 독음 집합.
    - 독음이 하나면 그것.
    - ▽(음이 변한 한자) 표시 독음이 있으면 그것. 단 글자별 독음도 사전에 있고 신문 빈도가 더 크면 글자별 독음
      (困難: 곤란▽ 31,801 > 곤난 20 → 곤란; 自家: 자갸▽ 1 < 자가 46,486 → 자가).
    - 아니면 전부 (行列 행렬/항렬). 신문 빈도는 동음이의어에 오염되므로(雲龍 운용 = 運用) 여기서는 안 쓴다."""
    rs = set(readings[key])
    if len(key) >= 2 and key[-1] == '茶':
        # 음료 이름의 茶 는 끝자리에서 '차' (綠茶 녹차·紅茶 홍차; 사전은 녹다·홍다). 앞자리는 사전대로 '다' (茶房 다방) — 10.15
        rs = {r[:-1] + '차' if r[-1] == '다' else r for r in rs}
    if len(rs) == 1:
        return rs
    m = {r for r in rs if (key, r) in marked}
    if m:
        pc = PLAIN.reading(key)
        if pc in rs and pc not in m and FREQ.get(pc, 0) > max(FREQ.get(r, 0) for r in m):
            return {pc}
        return m
    return rs

def choose(key):
    """예외표에 넣을 독음: 맞는 독음 집합에서 표준 → 사전 출현 수 → 신문 빈도 순."""
    acc = accepted(key)
    rs = readings[key]
    return sorted(acc, key=lambda r: ('S' not in srcs.get((key, r), ()), -rs.get(r, 0), -FREQ.get(r, 0), r))[0]

def stored_form(key, r):
    """첫 음절이 두음 적용 꼴이면 두음 이전 꼴로 저장 (연쇄 중간에서 맞게 읽히도록)."""
    c0 = key[0]
    cands = [x for x in undueum.get(r[0], ()) if x in char_readings(c0)]
    if cands:
        return sorted(cands)[0] + r[1:]
    return r

def dueum_words(keys):
    """첫머리 두음이 실제로 일어나는 2~4자 표제어 (理由 이유, 女性 여성, 勞動 노동 …)."""
    d = set()
    for k in keys:
        if not 2 <= len(k) <= DMAX:
            continue
        r = choose(k)
        p0 = prim.get(k[0])
        if p0 and p0 != r[0] and dueum.get(p0) == r[0]:
            d.add(k)
    return d

def pair_words(keys):
    """둘째 글자의 대표 독음이 두음 대상인 2자 표제어."""
    return {k for k in keys if len(k) == 2 and prim.get(k[1]) in dueum} if W2ON else set()

def build(keys, base_words, dw=None, w2=None):
    conv = Converter(prim, dueum, base_words, dwords=dw, dmin_prefix=DMINP, w2=w2)
    words = dict(base_words)
    for rnd in range(8):
        add = {}
        for k in keys:
            out = conv.reading(k)
            if out not in accepted(k):
                add[k] = stored_form(k, choose(k))
        add = {k: v for k, v in add.items() if k not in hand and len(k) > 1}   # 손 표가 이긴다; 한 글자는 글자표 몫
        if not add:
            break
        words.update(add)
        conv.set_words(words)
    return conv, words, rnd

def is_heldout(k):
    return hashlib.md5(k.encode('utf-8')).digest()[0] < 26     # ≈ 10%

keys = sorted(readings)
train = [k for k in keys if not is_heldout(k)]
held = [k for k in keys if is_heldout(k)]

def acc(conv, ks, only_s=False):
    tot = ok = 0
    bylen = collections.defaultdict(lambda: [0, 0])
    errs = collections.Counter()
    for k in ks:
        if only_s and not any('S' in srcs[(k, r)] for r in readings[k]):
            continue
        tot += 1
        out = conv.reading(k)
        L = min(len(k), 6)
        bylen[L][0] += 1
        if out in accepted(k):
            ok += 1
            bylen[L][1] += 1
        elif out:
            g = choose(k)
            for a, b, c in zip(k, out, g):
                if b != c:
                    errs[(a, b, c)] += 1
    return tot, ok, bylen, errs

hand_conv = Converter(prim0, dueum, hand, bu_rule=False, ryul_rule=False)
madi_conv = Converter(prim0, dueum, hand, bu_rule=False, ryul_rule=True)
rules_conv = Converter(prim, dueum, hand)
DMAX = int(os.environ.get('DMAX', '0'))
DMINP = int(os.environ.get('DMINP', '1'))
W2ON = os.environ.get('W2', '0') == '1'
dw_tr = dueum_words(train) if DMAX else set()
dw_all = dueum_words(keys) if DMAX else set()
say(f'- 합성어 안 두음 단어표: DMAX={DMAX}, 앞부분 최소 {DMINP}자, 생성용 {len(dw_tr):,} / 전체 {len(dw_all):,}')
w2_tr, w2_all = pair_words(train), pair_words(keys)
say(f'- 두음 막는 2자어: 생성용 {len(w2_tr):,} / 전체 {len(w2_all):,}')
conv_tr, words_tr, rounds = build(train, hand, dw_tr, w2_tr)

say('\n## 3. 평가 — 떼어 둔 10% (예외표는 나머지 90% 로 생성)')
say(f'- 떼어 둔 키 {len(held):,} / 생성용 {len(train):,}; 생성 예외표 {len(words_tr) - len(hand):,}행 + 손 표 {len(hand)}행, 고정점 {rounds + 1}회')
say('\n| 변환기 | 전체 키 | 표준국어대사전 키 |')
say('|---|---|---|')
rows = [('0.1.3 (글자별 + 두음 + 손 표 135)', hand_conv),
        ('madi hanja.rs (+ 렬/률)', madi_conv),
        ('새 글자표 + 不·렬/률 규칙 (예외표 없음)', rules_conv),
        ('새 변환기 (예외표 포함)', conv_tr)]
res_by = {}
for name, cv in rows:
    t, o, bl, er = acc(cv, held)
    ts, os_, _, _ = acc(cv, held, only_s=True)
    res_by[name] = (bl, er)
    say(f'| {name} | {o/t:.2%} ({o:,}/{t:,}) | {os_/ts:.2%} ({os_:,}/{ts:,}) |')
bl, er = res_by['새 변환기 (예외표 포함)']
say('\n길이별 (새 변환기, 떼어 둔 키): ' + ' · '.join(f'{L}{"+" if L == 6 else ""}자 {v[1]/v[0]:.1%}({v[0]:,})' for L, v in sorted(bl.items())))
say('남은 오류 상위 (한자, 낸 것, 사전): ' + ' '.join(f'{a}:{b}/{c}×{n}' for (a, b, c), n in er.most_common(30)))

with open(os.path.join(OUT, 'heldout_errors.tsv'), 'w', encoding='utf-8') as w:
    for k in held:
        o_ = conv_tr.reading(k)
        if o_ not in accepted(k):
            w.write(f'{k}\t{o_}\t{"/".join(sorted(accepted(k)))}\n')
t, o, _, _ = acc(conv_tr, train)
say(f'- 생성용 90% 위 정확도(자기 검증): {o/t:.3%}')

# ---------- 5. 최종 예외표 (전체로) ----------
conv_all, words_all, rounds = build(keys, hand, dw_all, w2_all)
gen = {k: v for k, v in words_all.items() if k not in hand}
t, o, _, _ = acc(conv_all, keys)
with open(os.path.join(DATA, 'words_nikl.csv'), 'w', encoding='utf-8') as w:
    w.write('# hanja_hangul 예외표 — 국립국어원 표준국어대사전·우리말샘(CC BY-SA 2.0 KR) 표제어에서, '
            '글자별 변환으로 맞지 않는 것만 뽑음. 한자,한글,출처(S=표준·U=우리말샘)\n')
    for k in sorted(gen):
        s = ''.join(sorted({x for r in readings[k] for x in srcs[(k, r)]}))
        w.write(f'{k},{gen[k]},{s}\n')
size = os.path.getsize(os.path.join(DATA, 'words_nikl.csv'))
say('\n## 4. 최종 예외표 (전체 키로 생성)')
say(f'- {len(gen):,}행, {size/1024:.0f} KB, 고정점 {rounds + 1}회, 전체 키 위 정확도 {o/t:.3%}')
lens = collections.Counter(len(k) for k in gen)
say('- 길이 분포: ' + ' '.join(f'{L}자 {n:,}' for L, n in sorted(lens.items())))

# 손 표와 사전이 다른 것
conf = [(k, v, '/'.join(accepted(k))) for k, v in hand.items() if k in readings and v not in accepted(k)]
say(f'- 손 표 135행 중 사전에 없는 키 {sum(1 for k in hand if k not in readings)}, 사전과 독음이 다른 키 {len(conf)}: ' + ' '.join(f'{k}:{v}≠{d}' for k, v, d in conf))

# ---------- 6. 합성어 두음 표·두음 막는 2자어·성씨 2자 접두 ----------
HDR = '# 국립국어원 표준국어대사전·우리말샘(CC BY-SA 2.0 KR) 순한자 표제어에서 유도. '
with open(os.path.join(DATA, 'dueum_words.txt'), 'w', encoding='utf-8') as w:
    w.write(HDR + '첫머리 두음이 일어나는 2~4자어: 연쇄 중간에서 이 단어가 시작하면 두음을 적용한다 (新女性 신여성).\n')
    w.write('\n'.join(sorted(dw_all)) + '\n')
with open(os.path.join(DATA, 'dueum_block.txt'), 'w', encoding='utf-8') as w:
    w.write(HDR + '둘째 글자가 두음 대상인 2자어: 앞 글자와 이 짝을 이루면 연쇄 중간 두음을 막는다 (評論家 평론가).\n')
    w.write('\n'.join(sorted(w2_all)) + '\n')
sp = sorted({k for k in keys if k[0] in SURNAME and 2 <= len(k) <= 4 and choose(k)[0] != SURNAME[k[0]]})   # 인명(金正喜 김정희)은 빼고
with open(os.path.join(DATA, 'surname_block.txt'), 'w', encoding='utf-8') as w:
    w.write(HDR + '金·沈·葉·契·單 으로 시작하고 성씨 아닌 독음(금·침·엽·계·단)으로 읽는 2~4자 표제어. '
            '연쇄가 2~3자면 연쇄 전체가, 4자 이상이면 앞 2~4자가 여기 있을 때 성씨로 읽지 않는다.\n')
    w.write('\n'.join(sp) + '\n')
sizes = {f: os.path.getsize(os.path.join(DATA, f)) for f in ('words_nikl.csv', 'dueum_words.txt', 'dueum_block.txt', 'surname_block.txt', 'chars_ext.csv', 'alts_nikl.csv')}
say('\n## 5. 크레이트에 싣는 생성 자료')
for f, s in sizes.items():
    say(f'- data/{f}: {s/1024:.0f} KB')
say(f'- 합계 {sum(sizes.values())/1024:.0f} KB (손 자료 hanja_char.csv 245 KB 별도)')
say('- 성씨 막는 표제어: ' + ' '.join(f'{c}:{sum(1 for k in sp if k[0] == c)}' for c in SURNAME))

# ---------- 7. 신문 후보표 대조 ----------
cand = os.path.join(REFS, 'hanja_word_candidates.tsv')
if os.path.exists(cand):
    say('\n## 6. hanja_word_candidates.tsv (신문 SH 빈도 순) 대조')
    rows = []
    for line in open(cand, encoding='utf-8'):
        if line.startswith('#'):
            continue
        p = line.rstrip('\n').split('\t')
        if len(p) < 6:
            continue
        rows.append((nfc(p[0]), p[1], p[3], int(p[5] or 0)))
    agree_rep = sum(1 for k, s, r, f in rows if conv_all.reading(k) == r)
    agree_sug = sum(1 for k, s, r, f in rows if conv_all.reading(k) == s)
    say(f'- {len(rows)}행: 새 변환기 = 대표 독음 {agree_rep}, = 제안 독음(빈도 높은 다른 말) {agree_sug}, 그 밖 {len(rows) - agree_rep - agree_sug}')
    say('- 신문 빈도 상위 30: ' + ' '.join(f'{k}→{conv_all.reading(k)}' for k, s, r, f in sorted(rows, key=lambda x: -x[3])[:30]))

open(os.path.join(OUT, 'report.md'), 'w', encoding='utf-8').write('\n'.join(rep) + '\n')

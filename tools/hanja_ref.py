"""hanja_hangul 0.2 변환 알고리즘의 파이썬 기준 구현 (예외표 생성·평가용).

Rust 크레이트와 같은 규칙을 따른다 (claude/madi-hanja-upgrade-plan.md §2):
  한자 연쇄마다 왼쪽부터
    예외표 최장 일치 → 그 독음 (연쇄 첫머리면 첫 음절에 두음)
    아니면 글자표 대표 독음 (연쇄 첫머리면 두음)
  글자별로 낸 음절에만 렬/률 규칙과 不 규칙을 후처리한다 (예외표 독음은 그대로 둔다).
"""
import unicodedata

def nfc(s):
    return unicodedata.normalize('NFC', s)

def is_hanja(c):
    o = ord(c)
    return (0x3400 <= o <= 0x4DBF or 0x4E00 <= o <= 0x9FFF or 0xF900 <= o <= 0xFAFF
            or 0x20000 <= o <= 0x3134F or 0x2F800 <= o <= 0x2FA1F)

def is_syllable(c):
    return 0xAC00 <= ord(c) <= 0xD7A3

def coda(c):
    return (ord(c) - 0xAC00) % 28 if is_syllable(c) else None

def onset(c):
    return (ord(c) - 0xAC00) // 588 if is_syllable(c) else None

ONSET_D, ONSET_J = 3, 12   # ㄷ, ㅈ


def load_chars(path):
    """통합 한자 행 = 대표 독음, 호환 한자 행 = 다른 독음 (madi hanja.rs 와 같은 규칙)."""
    prim, alts, compat = {}, {}, []
    for line in open(path, encoding='utf-8-sig'):
        if ',' not in line:
            continue
        k, v = line.strip().split(',', 1)
        k, v = k.strip(), v.strip()
        if not k or not v:
            continue
        is_compat = 0xF900 <= ord(k[0]) <= 0xFAFF or 0x2F800 <= ord(k[0]) <= 0x2FA1F
        kk = nfc(k)[0]
        if is_compat:
            compat.append((kk, v[0]))
        else:
            prim[kk] = v[0]
    for k, v in compat:
        if k in prim:
            if prim[k] != v:
                alts.setdefault(k, [])
                if v not in alts[k]:
                    alts[k].append(v)
        else:
            prim[k] = v
    return prim, alts


def load_pairs(path):
    d = {}
    for line in open(path, encoding='utf-8-sig'):
        if ',' in line:
            k, v = line.strip().split(',', 1)
            if k.strip() and v.strip():
                d[nfc(k.strip())] = v.strip()
    return d


class Converter:
    def __init__(self, prim, dueum, words=None, bu_rule=True, ryul_rule=True, dwords=None, dmin_prefix=1, w2=None):
        self.w2 = w2 or set()             # 둘째 글자가 두음 대상인 2자어 (評論·地理): 이 짝이면 합성어 안 두음을 막는다
        self.dwords = dwords or set()     # 첫머리에서 두음법칙이 적용되는 단어 (합성어 안 두음)
        self.dmax = max((len(k) for k in self.dwords), default=0)
        self.dmin_prefix = dmin_prefix
        self.prim = prim
        self.dueum = dueum
        self.words = dict(words or {})
        self.max_word = max((len(k) for k in self.words), default=0)
        self.bu_rule = bu_rule
        self.ryul_rule = ryul_rule

    def set_words(self, words):
        self.words = dict(words)
        self.max_word = max((len(k) for k in self.words), default=0)

    def du(self, s):
        return self.dueum.get(s, s)

    def reading(self, span, word_start=True):
        """한자열 하나(공백 없는 연쇄)의 독음. 글자표에 없는 글자는 None."""
        out = []        # (음절, 글자별 여부, 원 한자)
        i, n = 0, len(span)
        while i < n:
            hit = None
            for L in range(min(self.max_word, n - i), 1, -1):
                v = self.words.get(span[i:i + L])
                if v is not None:
                    hit = (L, v)
                    break
            if hit is None and self.max_word >= 1:
                v = self.words.get(span[i])
                if v is not None:
                    hit = (1, v)
            if hit:
                L, v = hit
                for j, s in enumerate(v):
                    if i == 0 and j == 0 and word_start:
                        s = self.du(s)
                    out.append((s, False, None))
                i += L
                continue
            c = span[i]
            r = self.prim.get(c)
            if r is None:
                return None
            if i == 0 and word_start:
                r = self.du(r)
            elif i >= self.dmin_prefix and self.dwords and span[i - 1:i + 1] not in self.w2:
                for L in range(min(self.dmax, n - i), 1, -1):
                    if span[i:i + L] in self.dwords:
                        r = self.du(r)
                        break
            out.append((r, True, c))
            i += 1
        # 후처리: 글자별로 낸 음절만
        res = [s for s, _, _ in out]
        for k, (s, per_char, c) in enumerate(out):
            if not per_char:
                continue
            if self.bu_rule and c == '不':
                nxt = res[k + 1] if k + 1 < len(res) else None
                res[k] = '부' if nxt is not None and onset(nxt) in (ONSET_D, ONSET_J) else '불'
                if k == 0 and word_start:
                    res[k] = self.du(res[k])
            if self.ryul_rule and k > 0 and s in ('렬', '률'):
                if coda(res[k - 1]) in (0, 4):
                    res[k] = '열' if s == '렬' else '율'
        return ''.join(res)

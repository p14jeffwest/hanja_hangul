import sys
sys.path.insert(0, 'tools')
from hanja_ref import Converter, load_chars, load_pairs, nfc, is_hanja
R = '/mnt/user-data/uploads/madi/refs/hanja/'
prim, _ = load_chars(R + 'hanja_char.csv')
for line in open('data/primary_override.csv', encoding='utf-8'):
    if not line.startswith('#'):
        c, b = line.split(',')[:2]; prim[c] = b
prim['不'] = '불'
for line in open('data/chars_ext.csv', encoding='utf-8'):
    if not line.startswith('#'):
        c, r, _ = line.strip().split(','); prim[c] = r
dueum = load_pairs(R + 'dueum.csv')
words = load_pairs(R + 'hanja_word.csv')
for line in open('data/words_nikl.csv', encoding='utf-8'):
    if not line.startswith('#'):
        k, v, s = line.strip().split(','); words.setdefault(k, v)
rd = lambda f: {l.strip() for l in open(f, encoding='utf-8') if l.strip() and not l.startswith('#')}
cv = Converter(prim, dueum, words, dwords=rd('data/dueum_words.txt'), w2=rd('data/dueum_block.txt'))
SB = rd('data/surname_block.txt'); SUR = {'金': '김', '沈': '심', '葉': '섭', '契': '글', '單': '선'}
def convert(text):
    out, i, n = [], 0, len(text)
    while i < n:
        if not is_hanja(text[i]):
            out.append(text[i]); i += 1; continue
        j = i
        while j < n and is_hanja(text[j]): j += 1
        span = nfc(text[i:j])
        ws = not (i > 0 and (text[i - 1].isdigit() or 0xAC00 <= ord(text[i - 1]) <= 0xD7A3))   # 숫자·한글 바로 뒤는 첫머리 아님
        r = cv.reading(span, ws)
        if r is None:
            r = span
        elif span[0] in SUR and (len(span) > 1 or span[0] in '金沈') and not (
                span in SB if len(span) <= 3 else any(span[:L] in SB for L in (2, 3, 4))):
            rest = cv.reading(span[1:], False) if len(span) > 1 else ''
            r = SUR[span[0]] + (rest or '')
        out.append(r); i = j
    return ''.join(out)
for line in sys.stdin:
    print(convert(line.rstrip('\n')))

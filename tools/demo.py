import sys
sys.path.insert(0, 'tools')
from hanja_ref import Converter, load_chars, load_pairs, nfc, is_hanja
import os
R = os.environ.get('HH_REFS', '/mnt/user-data/uploads/badang/hanja_hangul/')
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
NB = rd('data/name_block.txt')
NAME_SUR = set('金李朴崔鄭姜趙尹張林韓吳徐申權黃安宋柳洪全高文孫梁裵白曺許劉南沈盧河丁成郭車兪具禹朱任羅辛閔陳池嚴蔡元千方孔玄咸卞楊廉邊呂秋魯都蘇愼石宣薛馬吉周延表明魏潘王琴玉陸印孟諸卓秦慶睦太奇殷片龍芮邢偰鞠陰甘')
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
        else:
            sur = False
            if span not in cv.words and span[0] in SUR and (len(span) > 1 or span[0] in '金沈') and not (
                    span in SB if len(span) <= 3 else any(span[:L] in SB for L in (2, 3, 4))):
                # 성씨 (연쇄 전체가 예외표에 있으면 예외표가 우선: 金龍煥 김용환)
                rest = cv.reading(span[1:], False) if len(span) > 1 else ''
                r = SUR[span[0]] + (rest or '')
                sur = True
            # 성씨 규칙이 이미 인명이라고 봤으면 2자어 막기는 건너뛴다 (金龍洙: 金龍 금룡 은 낱말이지만 인명)
            if (len(span) == 3 and span[0] in NAME_SUR and span not in cv.words and span not in NB
                    and (sur or span[:2] not in cv.w2) and len(r) == 3):
                # 이름 첫 음절 두음 (李龍俊 이용준, 金令培 김영배)
                r = r[0] + cv.du(r[1]) + r[2]
        out.append(r); i = j
    return ''.join(out)
if __name__ == "__main__":
  for line in sys.stdin:
    print(convert(line.rstrip("\n")))

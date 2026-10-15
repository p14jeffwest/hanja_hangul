"""국립국어원 사전(표준국어대사전·우리말샘) XML → 한자 원어가 있는 표제어 목록.

입력: korean-dict-nikl-master.zip (풀지 않고 읽는다)
출력(TSV, gzip): src  code  word  hangul  hanja  langs  unit  types  pos
  src   S = 표준국어대사전, U = 우리말샘
  word  표제어 원문 (-, ^, 동형어 번호 포함)
  hangul 표제어에서 기호·번호를 뺀 것
  hanja original_language 조각을 이은 것 (조각마다 language_type 은 langs 에 '+' 로)
  types 뜻 갈래(일반어·방언·북한어·옛말) 모음, pos 품사 모음
"""
import gzip, re, sys, zipfile
import xml.etree.ElementTree as ET

def clean_word(w):
    w = re.sub(r'\d+$', '', w)          # 동형어 번호 (자음-자001)
    return w.replace('-', '').replace('^', '').replace(' ', '')

def items(z, prefix):
    for n in sorted(z.namelist()):
        if n.startswith(prefix) and n.endswith('.xml'):
            with z.open(n) as f:
                for ev, el in ET.iterparse(f, events=('end',)):
                    if el.tag == 'item':
                        yield el
                        el.clear()

def run(zpath, out):
    z = zipfile.ZipFile(zpath)
    n = {'S': 0, 'U': 0}
    with gzip.open(out, 'wt', encoding='utf-8') as w:
        w.write('src\tcode\tword\thangul\thanja\tlangs\tunit\ttypes\tpos\n')
        for src, prefix, wi_tag in (('S', 'korean-dict-nikl-master/stdict/', 'word_info'),
                                    ('U', 'korean-dict-nikl-master/opendict/', 'wordInfo')):
            for it in items(z, prefix):
                wi = it.find(wi_tag)
                if wi is None:
                    continue
                ol = wi.findall('original_language_info')
                if not ol:
                    continue
                pieces = [(o.findtext('original_language') or '').strip() for o in ol]
                langs = [(o.findtext('language_type') or '').strip() for o in ol]
                if '한자' not in langs:
                    continue
                word = (wi.findtext('word') or '').strip()
                unit = (wi.findtext('word_unit') or '').strip()
                types = sorted({(t.text or '').strip() for t in it.iter('type') if t.text and t.text.strip() in ('일반어', '방언', '북한어', '옛말')})
                pos = sorted({(p.text or '').strip() for p in it.iter('pos') if p.text})
                code = (it.findtext('target_code') or '').strip()
                w.write('\t'.join([src, code, word, clean_word(word), ''.join(pieces), '+'.join(langs),
                                   unit, ','.join(types), ','.join(pos)]) + '\n')
                n[src] += 1
    print(n, file=sys.stderr)

if __name__ == '__main__':
    run(sys.argv[1], sys.argv[2])

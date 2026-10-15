import sys, os, io, contextlib
# 쓰는 법: python3 tools/dump_keys.py <refs 디렉터리(hanja_char.csv 등)> <nikl_hanja.tsv.gz> <임시 data> <임시 out> <news_noun_freq.tsv>
#   → out/all_keys.txt (표제어 키), out/heldout_keys.tsv (떼어 둔 키와 정답)
sys.argv = ['build_words.py'] + sys.argv[1:6]
os.environ.update(DMAX='4', DMINP='1', W2='1')
buf = io.StringIO()
with contextlib.redirect_stdout(buf):
    exec(open('tools/build_words.py', encoding='utf-8').read())
with open('out/all_keys.txt', 'w', encoding='utf-8') as w:
    for k in keys:
        w.write(k + '\n')
with open('out/heldout_keys.tsv', 'w', encoding='utf-8') as w:
    for k in held:
        w.write(k + '\t' + '/'.join(sorted(accepted(k))) + '\n')
print(len(keys), len(held))

#!/usr/bin/env python3

def usage(name):
    print(f'''Usage: {name} [options] [(cipher) text]\n
    -k\tkey [default: 7]
    -d\tdecryption mode
    -h\thelp
''')


def index(alph, ch):
    if ch in alph:
        return alph.index(ch)
    return ch

def caesar(inp, key, rev=False, alph='abcdefghijklmnopqrstuvwxyz', keep_foreign=True):
    if rev:
        key *= -1
    al = len(alph)
    out = ''
    for ch in inp:
        upper = ch.isupper()
        idx = index(alph.upper() if upper else alph, ch)
        if isinstance(idx, str) and keep_foreign:
            out += idx
            continue
        idx += key
        while idx < 0 or idx > al - 1:
            if idx < 0:
                idx += al
            elif idx > al - 1:
                idx -= al
        nc = alph[idx]
        out += nc.upper() if upper else nc
    return out


if __name__ == '__main__':
    from sys import argv
    if len(argv) > 1:
        from getopt import getopt, GetoptError
        rev = False
        key = 7

        try:
            opts, args = getopt(argv[1:], 'hdk:', ['help', 'decrypt', 'key='])
            for opt, arg in opts:
                opt = opt.strip('-')[0]

                if opt == 'h':
                    usage(argv[0])

                elif opt == 'd':
                    rev = True
                elif opt == 'k':
                    key = int(arg)

            print(caesar(' '.join(args), key, rev))

        except GetoptError:
            usage(argv[0])
    else:
        usage(argv[0])

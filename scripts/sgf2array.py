#!/usr/bin/env python3

""" Pretty-print the provided SGF file into a format that is easier to store
inside of Rust source code. For example:

```
./scripts/sgf2array.py < data/3ch0-gokifu-20221114-Yang_Dingxin-Shibano_Toramaru.sgf
```
"""

import pprint
import re
import sys

def parse_line(line):
    """ Yields all `(color, coord)` tuples that exist in the given SGF file. """

    for matches in re.findall(r';([BW])\[([a-z]*)\]', line):
        coord = ['abcdefghijklmnopqrstuvwyz'.index(ch) for ch in matches[1]]

        if len(coord) != 2:
            coord = [19, 19]

        yield (matches[0], tuple(coord))

for line in sys.stdin:
    pprint.pprint([m for m in parse_line(line)], indent=4, width=120, compact=True)

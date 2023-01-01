#!/usr/bin/env python3

""" Pretty-print an array of 2048 random `u32`'s """

import pprint
import random

class Hex(int):
    def __repr__(self):
        return f'0x{self:08x}'

pprint.pprint(
    [Hex(random.randint(0, 4294967295)) for _ in range(2048)],
    indent=4,
    width=120,
    compact=True,
)

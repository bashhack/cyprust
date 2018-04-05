import random
import re
import string
from itertools import tee

import rustlib


# Python ZIP
def count_doubles(val):
    total = 0
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total


# Python REGEX
double_re = re.compile(r'(?=(.)\1)')


def count_doubles_regex(val):
    return len(double_re.findall(val))


def count_doubles_functional(val):
    c1s, c2s = tee(val)
    next(c2s, None)
    total = 0
    for c1, c2 in zip(c1s, c2s):
        if c1 == c2:
            total += 1
    return total


# Benchmarking...
val = ''.join(random.choice(string.ascii_letters) for i in range(1000000))


def test_pure_python(benchmark):
    benchmark(count_doubles, val)


def test_functional_python(benchmark):
    benchmark(count_doubles_functional, val)


def test_regex(benchmark):
    benchmark(count_doubles_regex, val)


def test_rust(benchmark):
    benchmark(rustlib.count_doubles, val)


def test_rust_fast_af(benchmark):
    benchmark(rustlib.count_doubles_fast_af, val)

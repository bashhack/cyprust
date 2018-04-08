# cython: infer_types=True
import re
from itertools import tee


# Python Zip
def count_doubles(val):
    total = 0
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total


# cpdef unsigned long long typed_count_doubles(str val):
#     cdef unsigned long long total = 0
#     for c1, c2 in zip(val, val[1:]):
#         if c1 == c2:
#             total += 1
#     return total


# Python REGEX
double_re = re.compile(r'(?=(.)\1)')


def count_doubles_regex(val):
    return len(double_re.findall(val))


# Python Itertools
def count_doubles_functional(val):
    c1s, c2s = tee(val)
    next(c2s, None)
    total = 0
    for c1, c2 in zip(c1s, c2s):
        if c1 == c2:
            total += 1
    return total


# cpdef typed_count_doubles_functional(str val):
#     cdef unsigned long long total = 0
#     c1s, c2s = tee(val)
#     for c1, c2 in zip(c1s, c2s):
#         if c1 == c2:
#             total += 1
#     return total

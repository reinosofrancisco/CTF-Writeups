# Tadpole

## Introduction

This is a implementation of a LCG algorithm. The algorithm is used to generate a random number.

## Implementation

```python
from Crypto.Util.number import bytes_to_long, isPrime
from secrets import randbelow

p = bytes_to_long(open("flag.txt", "rb").read())
assert isPrime(p)

a = randbelow(p)
b = randbelow(p)

def f(s):
    return (a * s + b) % p

print("a = ", a)
print("b = ", b)
print("f(31337) = ", f(31337))
print("f(f(31337)) = ", f(f(31337)))
```

## Given Data

We are given the Values of a, b, f(s) and f(f(s)). <br>
s being hardcoded to 31337.


## Resolution

We will need to calculate de gcd of 
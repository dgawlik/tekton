
# Tekton block cipher

I wrote this experimental 128/256 block cipher while learning cryptography. The rationale was it to be:

* understandable algorithm
* performance competing with AES
* secure


### Design

The encryption as in other ciphers comes in rounds, but with smaller number of them. So for the encryption it is:

```
state = diffusion(state)
state = permute(state)
state = substitute(state)
state = xor(state, key[i])
```

and decryption is:

```
state = xor(state, key[i])
state = inverseSubstitute(state)
state = inversePermute(state)
state = diffusion(state)
```

The **diffusion layer** takes the numbers and applies transformation so that each resulting bit is:

bit<sub>i</sub> = bit<sub>i</sub>  ^ bit<sub>i+1</sub>  ^ bit<sub>i+2</sub>  ^bit<sub>i+4</sub>  ^ bit<sub>i+8</sub>  ^ bit<sub>i+16</sub>  ^ ...

In other words like this

```go
p1 := (x & uint64(0b_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101)) << 1
p2 := (x & uint64(0b_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011)) << 2
p3 := (x & uint64(0b_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111)) << 4
p4 := (x & uint64(0b_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111)) << 8
p5 := (x & uint64(0b_00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111)) << 16
p6 := (x & uint64(0b_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111)) << 32

return x ^ p1 ^ p2 ^ p3 ^ p4 ^ p5 ^ p6
```

For xor'ing to be reversible it must be one-sided with mask (can't do criss-cross or rotation of bits).

The **permutation layer** shuffles 16 bytes of the vector. It is crucial as it distributes evenly confusion arising from
xors among byte positions.

**Substitution layer** replaces each byte with value from 256-byte lookup table (which is random). It breaks the pattern 
propagation. So it counters differential analysis, I would be surprised if it didn't suffice, it seems solid.

**xor with key** layer binds the cipher to the key.

I think combination of diffusion, substitution, and permutation in multiple rounds guarantees that even for input with only single bit set, we 
get a number with random number of 1's and 0's (roughly same). So the high confusion and diffusion Shannon metrics are 
satisfied. 

I believe that you can't do differential analysis or interpolation attacks on the cipher but I might be wrong. Overall design looks
pretty solid.

### Performance

Let's be honest AES has been around for 20 years and it's even on processors. So you can't beat that, but I'm satisfied 
with the perfomance which is 3-4 time slower.

||Tekton 128bit|Tekton 256bit|AES 128bit|AES 256bit|
|---|---|---|---|----|
|2M Nonces|262 ms  | 526 ms | 57 ms | 63 ms|


### Help needed

If you have some spare time you can help me out with couple of things:

* if you are expert in crypto, can you please assess security of the cipher
* if you are fluent in go, can you please instruct me in writing .s files as I would like to try out AVX2
* if you are low level coder, maybe you could show how vectorize diffusion, permute and substitute parts?
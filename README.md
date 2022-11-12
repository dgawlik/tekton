# Lightweight Block Cipher

This is experimental block cipher written in Rust taking advantage of features of modern processors. 

Usage:

```rustup toolchain install nightly```

```rustup default nightly```

```./benchmark.sh```


### Design

Encrypting is done in rounds, each round consists of these steps:

* key XOR
* expansion
* sbox
* rotation

### Performance

|Metric|AES|LBC|
|------|---|---|
|Uniformness|0.998|1.002|
|Diffusion|63.171|63.288|
|1M nonces|18.33 ms|14.03 ms|


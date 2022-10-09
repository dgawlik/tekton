package main

import (
	"bytes"
	"encoding/hex"
	"flag"
	"fmt"
	"math/rand"
	"time"
	"unsafe"
)

var generate = flag.Bool("generate", false, "Generates random 128-bit hex string")
var key = flag.String("key", "", "128-bit key")
var encrypt = flag.String("encrypt", "", "Encrypts 128-bit hash string")
var decrypt = flag.String("decrypt", "", "Decrypts 128-bit hash string")

type U128 [16]byte

type State struct {
	Keys []U128
	P    [16]int
	invP [16]int
	S    [256]byte
	invS [256]byte
}

func hexToVector(x string) U128 {
	return *(*U128)(decode(x))
}

func (key *U128) expand(P [16]int, S [256]byte, noRounds int) []U128 {
	var keys []U128

	for i := 1; i <= noRounds; i++ {
		newKey := *key

		hi, lo := newKey.longView()
		*hi <<= i
		*lo <<= i

		newKey.permuteSubstitute(&P, &S)
		keys = append(keys, newKey)
	}

	return keys
}

func (key *U128) bootstrap() State {
	Ks := (*[2]uint64)(unsafe.Pointer(&key[0]))

	K := Ks[0] ^ Ks[1]

	source := rand.NewSource(int64(K))
	r := rand.New(source)

	var P [16]int
	var invP [16]int
	var S [256]byte
	var invS [256]byte

	for i := 0; i < 16; i++ {
		P[i] = i
	}

	for i := 0; i < 256; i++ {
		S[i] = byte(i)
	}

	for i := 0; i < 65_000; i++ {
		a := r.Intn(16)
		b := r.Intn(16)

		c := r.Intn(256)
		d := r.Intn(256)

		P[a], P[b] = P[b], P[a]
		S[c], S[d] = S[d], S[c]
	}

	for i := 0; i < 16; i++ {
		invP[P[i]] = i
	}

	for i := 0; i < 256; i++ {
		invS[S[i]] = byte(i)
	}

	return State{key.expand(P, S, 2), P, invP, S, invS}

}

func (a *U128) permuteSubstitute(P *[16]int, S *[256]byte) {
	var result U128

	for i := 0; i < 16; i++ {
		result[P[i]] = S[a[i]]
	}

	*a = result
}

func (a *U128) invPermuteSubstitute(invP *[16]int, invS *[256]byte) {
	var result U128

	for i := 0; i < 16; i++ {
		result[invP[i]] = invS[a[i]]
	}

	*a = result
}

var SOURCE = rand.NewSource(time.Now().UnixNano())
var RNG = rand.New(SOURCE)

func randomStringU128() string {

	var buf bytes.Buffer

	hi := RNG.Uint64()
	lo := RNG.Uint64()

	buf.WriteString(fmt.Sprintf("%016x", hi))
	buf.WriteString(fmt.Sprintf("%016x", lo))

	return buf.String()
}

func randomStringU256() string {
	return randomStringU128() + randomStringU128()
}

func decode(text string) []byte {
	v, _ := hex.DecodeString(text)
	return v
}

func encode(payload []byte) string {

	var buf bytes.Buffer

	for _, v := range payload {
		buf.WriteString(fmt.Sprintf("%02x", v))
	}

	return buf.String()
}

func (x *U128) longView() (*uint64, *uint64) {
	hi := (*uint64)(unsafe.Pointer(x))
	lo := (*uint64)(unsafe.Pointer(uintptr(unsafe.Pointer(&x[0])) + 8*unsafe.Sizeof(x[0])))

	return hi, lo
}

func diffusionUint64(x uint64) uint64 {

	p1 := (x & uint64(0b_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101)) << 1
	p2 := (x & uint64(0b_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011)) << 2
	p3 := (x & uint64(0b_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111)) << 4
	p4 := (x & uint64(0b_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111)) << 8
	p5 := (x & uint64(0b_00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111)) << 16
	p6 := (x & uint64(0b_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111)) << 32

	p7 := (x & uint64(0b_10101010_10101010_10101010_10101010_10101010_10101010_10101010_10101010)) >> 1
	p8 := (x & uint64(0b_11001100_11001100_11001100_11001100_11001100_11001100_11001100_11001100)) >> 2
	p9 := (x & uint64(0b_11110000_11110000_11110000_11110000_11110000_11110000_11110000_11110000)) >> 4
	p10 := (x & uint64(0b_11111111_00000000_11111111_00000000_11111111_00000000_11111111_00000000)) >> 8
	p11 := (x & uint64(0b_11111111_11111111_00000000_00000000_11111111_11111111_00000000_00000000)) >> 16
	p12 := (x & uint64(0b_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000)) >> 32

	return x ^ p1 ^ p2 ^ p3 ^ p4 ^ p5 ^ p6 ^ p7 ^ p8 ^ p9 ^ p10 ^ p11 ^ p12
}

func (x *U128) diffusion128() {
	cX, cY := x.longView()
	*cX = diffusionUint64(*cX)
	*cY = diffusionUint64(*cY)
	c0 := *cY ^ uint64(0)
	c1 := *cX ^ uint64(0)
	*cX = c0
	*cY = c1
}

func (x *U128) xor(y *U128) {

	for i := 0; i < 16; i++ {
		x[i] ^= y[i]
	}
}

func (st *State) doEncrypt(x U128) U128 {

	state := x

	state.diffusion128()
	state.permuteSubstitute(&st.P, &st.S)
	state.xor(&st.Keys[0])

	state.diffusion128()
	state.permuteSubstitute(&st.P, &st.S)
	state.xor(&st.Keys[1])

	return state

}

func (st *State) doDecrypt(x U128) U128 {
	state := x

	state.xor(&st.Keys[1])
	state.invPermuteSubstitute(&st.invP, &st.invS)
	state.diffusion128()

	state.xor(&st.Keys[0])
	state.invPermuteSubstitute(&st.invP, &st.invS)
	state.diffusion128()

	return state
}

func main() {

	flag.Parse()

	if *generate {
		fmt.Println(randomStringU128())
	} else if *encrypt != "" && *key != "" {
		k := hexToVector(*key)
		t := hexToVector(*encrypt)

		st := k.bootstrap()

		c := st.doEncrypt(t)
		fmt.Println(encode(c[:]))
	} else if *decrypt != "" && *key != "" {

	} else {
		flag.Usage()
	}
}

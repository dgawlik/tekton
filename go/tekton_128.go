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

type StateU128 struct {
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

func (key *U128) bootstrap() StateU128 {
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
		c := r.Intn(256)
		d := r.Intn(256)

		S[c], S[d] = S[d], S[c]
	}

	P[0] = 9
	invP[9] = 0
	P[1] = 13
	invP[13] = 1
	P[2] = 10
	invP[10] = 2
	P[3] = 14
	invP[14] = 3
	P[4] = 8
	invP[8] = 4
	P[5] = 11
	invP[11] = 5
	P[6] = 15
	invP[15] = 6
	P[7] = 12
	invP[12] = 7

	for i := 0; i < 256; i++ {
		invS[S[i]] = byte(i)
	}

	return StateU128{key.expand(P, S, 2), P, invP, S, invS}

}

func (a *U128) permuteSubstitute(P *[16]int, S *[256]byte) {

	for i := 0; i < 8; i++ {
		t := a[i]
		a[i] = a[P[i]]
		a[P[i]] = t
	}

	for i := 0; i < 16; i++ {
		a[i] = S[a[i]]
	}
}

func (a *U128) invPermuteSubstitute(invP *[16]int, invS *[256]byte) {

	for i := 8; i < 16; i++ {
		t := a[i]
		a[i] = a[invP[i]]
		a[invP[i]] = t
	}

	for i := 0; i < 16; i++ {
		a[i] = invS[a[i]]
	}
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

func (x *U128) diffusion() {
	hi, lo := x.longView()

	*hi = diffusionUint64(*hi)
	*lo = diffusionUint64(*lo)
}

func (x *U128) xor(y *U128) {

	for i := 0; i < 16; i++ {
		x[i] ^= y[i]
	}
}

func (st *StateU128) doEncrypt(x U128) U128 {

	state := x

	state.diffusion()
	state.permuteSubstitute(&st.P, &st.S)
	state.xor(&st.Keys[0])

	state.diffusion()
	state.permuteSubstitute(&st.P, &st.S)
	state.xor(&st.Keys[1])

	return state

}

func (st *StateU128) doDecrypt(x U128) U128 {
	state := x

	state.xor(&st.Keys[1])
	state.invPermuteSubstitute(&st.invP, &st.invS)
	state.diffusion()

	state.xor(&st.Keys[0])
	state.invPermuteSubstitute(&st.invP, &st.invS)
	state.diffusion()

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

package main

import (
	"bytes"
	"encoding/hex"
	"flag"
	"fmt"
	"math/bits"
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

func bootstrap(key U128) State {
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

	keys := make([]U128, 1)
	keys = append(keys, key)

	shift := 1
	for i := 0; i < 6; i++ {
		k := key
		cK := (*[2]uint64)(unsafe.Pointer(&k[0]))
		cK[0], cK[1] = bits.RotateLeft64(cK[0], -i), bits.RotateLeft64(cK[1], -i)
		keys = append(keys, k)
		shift *= 2
	}

	return State{keys, P, invP, S, invS}

}

func (st *State) permuteSubstitute(a *U128) {
	var result U128

	for i := 0; i < 16; i++ {
		result[st.P[i]] = st.S[a[i]]
	}

	*a = result
}

func (st *State) invPermuteSubstitute(a *U128) {
	var result U128

	for i := 0; i < 16; i++ {
		result[st.invP[i]] = st.invS[a[i]]
	}

	*a = result
}

var SOURCE = rand.NewSource(time.Now().UnixNano())
var RNG = rand.New(SOURCE)

func randomString() string {

	var buf bytes.Buffer

	for i := 0; i < 16; i++ {
		buf.WriteString(fmt.Sprintf("%02x", byte(RNG.Intn(256))))
	}

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

func (x *U128) getLongs() [2]uint64 {
	var result [2]uint64
	result[0] = *((*uint64)(unsafe.Pointer(x)))
	result[1] = *((*uint64)(unsafe.Pointer(uintptr(unsafe.Pointer(&x[0])) + 8*unsafe.Sizeof(x[0]))))

	return result
}

func (result *U128) saveLongs(x *[2]uint64) {

	hi := (*[8]byte)(unsafe.Pointer(x))
	lo := (*[8]byte)(unsafe.Pointer(uintptr(unsafe.Pointer(&x[0])) + 1*unsafe.Sizeof(x[0])))

	copy(result[:], hi[:])
	copy(result[len(hi):], lo[:])
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

func diffusion128(x *U128) {
	convX := x.getLongs()
	convX[0] = diffusionUint64(convX[0])
	convX[1] = diffusionUint64(convX[1])
	c0 := convX[1] ^ uint64(0)
	c1 := convX[0] ^ uint64(0)
	convX[0] = c0
	convX[1] = c1
	x.saveLongs(&convX)
}

func (x *U128) xor(y *U128) {

	for i := 0; i < 16; i++ {
		x[i] ^= y[i]
	}
}

func (st *State) doEncrypt(x U128) U128 {

	state := x

	diffusion128(&state)
	st.permuteSubstitute(&state)

	state.xor(&st.Keys[1])
	state.xor(&st.Keys[2])
	state.xor(&st.Keys[3])

	return state

}

func (st *State) doDecrypt(x U128) U128 {
	state := x

	state.xor(&st.Keys[3])
	state.xor(&st.Keys[2])
	state.xor(&st.Keys[1])

	st.invPermuteSubstitute(&state)
	diffusion128(&state)

	return state
}

func main() {

	flag.Parse()

	if *generate {
		fmt.Println(randomString())
	} else if *encrypt != "" && *key != "" {
		k := hexToVector(*key)
		t := hexToVector(*encrypt)

		st := bootstrap(k)

		c := st.doEncrypt(t)
		fmt.Println(encode(c[:]))
	} else if *decrypt != "" && *key != "" {

	} else {
		flag.Usage()
	}
}

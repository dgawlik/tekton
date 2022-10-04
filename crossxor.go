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

type BitVector [16]byte
type BitIndex [128]int

type State struct {
	Keys []BitVector
	P    [16]int
	invP [16]int
	S    [256]byte
	invS [256]byte
}

func hexToVector(x string) BitVector {
	return *(*BitVector)(decode(x))
}

func bootstrap(key BitVector) State {
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

	keys := make([]BitVector, 1)
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

func (st *State) permuteSubstitute(a *BitVector) {
	var result BitVector

	for i := 0; i < 16; i++ {
		result[st.P[i]] = st.S[a[i]]
	}

	*a = result
}

func (st *State) substituteLong(a uint64) uint64 {
	A := (*[8]byte)(unsafe.Pointer(&a))
	var result [8]byte

	for i := 0; i < 8; i++ {
		result[i] = st.S[A[i]]
	}

	return *(*uint64)(unsafe.Pointer(&result))
}

func (st *State) invSubstituteLong(a uint64) uint64 {
	A := (*[8]byte)(unsafe.Pointer(&a))
	var result [8]byte

	for i := 0; i < 8; i++ {
		result[i] = st.invS[A[i]]
	}

	return *(*uint64)(unsafe.Pointer(&result))
}

func (st *State) invPermuteSubstitute(a *BitVector) {
	var result BitVector

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

func (st *State) getKey(i, j int) uint64 {
	return (*[2]uint64)(unsafe.Pointer(&st.Keys[i]))[j]
}

func diffusion(x uint64) uint64 {

	p1 := (x ^ bits.RotateLeft64(x&uint64(0b_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101), 1)) & uint64(0b_10101010_10101010_10101010_10101010_10101010_10101010_10101010_10101010)
	p2 := (x ^ bits.RotateLeft64(x&uint64(0b_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011), 2)) & uint64(0b_11001100_11001100_11001100_11001100_11001100_11001100_11001100_11001100)
	p3 := (x ^ bits.RotateLeft64(x&uint64(0b_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111), 4)) & uint64(0b_11110000_11110000_11110000_11110000_11110000_11110000_11110000_11110000)
	p4 := (x ^ bits.RotateLeft64(x&uint64(0b_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111), 8)) & uint64(0b_11111111_00000000_11111111_00000000_11111111_00000000_11111111_00000000)
	p5 := (x ^ bits.RotateLeft64(x&uint64(0b_00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111), 16)) & uint64(0b_11111111_11111111_00000000_00000000_11111111_11111111_00000000_00000000)
	p6 := (x ^ bits.RotateLeft64(x&uint64(0b_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111), 32)) & uint64(0b_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000)

	p7 := (x ^ bits.RotateLeft64(x&uint64(0b_10101010_10101010_10101010_10101010_10101010_10101010_10101010_10101010), -1)) & uint64(0b_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101)
	p8 := (x ^ bits.RotateLeft64(x&uint64(0b_11001100_11001100_11001100_11001100_11001100_11001100_11001100_11001100), -2)) & uint64(0b_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011)
	p9 := (x ^ bits.RotateLeft64(x&uint64(0b_11110000_11110000_11110000_11110000_11110000_11110000_11110000_11110000), -4)) & uint64(0b_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111)
	p10 := (x ^ bits.RotateLeft64(x&uint64(0b_11111111_00000000_11111111_00000000_11111111_00000000_11111111_00000000), -8)) & uint64(0b_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111)
	p11 := (x ^ bits.RotateLeft64(x&uint64(0b_11111111_11111111_00000000_00000000_11111111_11111111_00000000_00000000), -16)) & uint64(0b_00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111)
	p12 := (x ^ bits.RotateLeft64(x&uint64(0b_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000), -32)) & uint64(0b_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111)

	return x ^ p1 ^ p2 ^ p3 ^ p4 ^ p5 ^ p6 ^ p7 ^ p8 ^ p9 ^ p10 ^ p11 ^ p12
}

func (st *State) doEncrypt(x BitVector) BitVector {

	state := x
	st.permuteSubstitute(&state)

	cState := *(*[2]uint64)(unsafe.Pointer(&state[0]))
	var cResult [2]uint64

	s1 := cState[0]

	s1 = diffusion(s1)
	s1 ^= st.getKey(1, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(2, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(3, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(4, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(5, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(6, 0)

	s2 := cState[1]

	s2 = diffusion(s2)
	s2 ^= st.getKey(1, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(2, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(3, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(4, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(5, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(6, 1)

	cResult[0] = st.substituteLong(s1) ^ st.getKey(1, 0)
	cResult[1] = st.substituteLong(s2) ^ st.getKey(0, 0)

	return *(*BitVector)(unsafe.Pointer(&cResult))
}

func (st *State) doDecrypt(x BitVector) BitVector {
	state := x

	cState := *(*[2]uint64)(unsafe.Pointer(&state[0]))
	var cResult [2]uint64

	s1 := cState[0]
	s2 := cState[1]

	s1 = st.invSubstituteLong(s1 ^ st.getKey(1, 0))
	s2 = st.invSubstituteLong(s2 ^ st.getKey(0, 0))

	s1 ^= st.getKey(6, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(5, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(4, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(3, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(2, 0)
	s1 = diffusion(s1)
	s1 ^= st.getKey(1, 0)
	s1 = diffusion(s1)

	s2 ^= st.getKey(6, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(5, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(4, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(3, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(2, 1)
	s2 = diffusion(s2)
	s2 ^= st.getKey(1, 1)
	s2 = diffusion(s2)

	cResult[0], cResult[1] = s1, s2

	result := *(*BitVector)(unsafe.Pointer(&cResult))

	st.invPermuteSubstitute(&result)

	return result
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

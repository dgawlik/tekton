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
	Key  BitVector
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

	return State{key, P, invP, S, invS}

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

func (st *State) encryptLong(x, key uint64) uint64 {
	state := x

	state ^= bits.RotateLeft64(key, 1)
	state = bits.RotateLeft64(state, 1)

	state ^= bits.RotateLeft64(key, 2)
	state = bits.RotateLeft64(state, 2)

	state ^= bits.RotateLeft64(key, 4)
	state = bits.RotateLeft64(state, 4)

	state ^= bits.RotateLeft64(key, 8)
	state = bits.RotateLeft64(state, 8)

	state ^= bits.RotateLeft64(key, 16)
	state = bits.RotateLeft64(state, 16)

	state ^= bits.RotateLeft64(key, 32)
	state = bits.RotateLeft64(state, 32)

	return st.substituteLong(state)
}

func (st *State) decryptLong(x, key uint64) uint64 {
	state := st.invSubstituteLong(x)

	state = bits.RotateLeft64(state, -32)
	state ^= bits.RotateLeft64(key, 32)

	state = bits.RotateLeft64(state, -16)
	state ^= bits.RotateLeft64(key, 16)

	state = bits.RotateLeft64(state, -8)
	state ^= bits.RotateLeft64(key, 8)

	state = bits.RotateLeft64(state, -4)
	state ^= bits.RotateLeft64(key, 4)

	state = bits.RotateLeft64(state, -2)
	state ^= bits.RotateLeft64(key, 2)

	state = bits.RotateLeft64(state, -1)
	state ^= bits.RotateLeft64(key, 1)

	return state
}

// not homomorphic
func (st *State) doEncrypt(x BitVector) BitVector {

	state := x
	st.permuteSubstitute(&state)

	var result BitVector

	cA := (*[2]uint64)(unsafe.Pointer(&state[0]))
	cB := (*[2]uint64)(unsafe.Pointer(&st.Key[0]))
	cResult := (*[2]uint64)(unsafe.Pointer(&result[0]))

	cResult[0], cResult[1] = st.encryptLong(cA[0], cB[0]), st.encryptLong(cA[1], cB[1])
	cA[0], cA[1] = cResult[0], cResult[1]

	cResult[0] = cA[0] ^ cB[1]
	cResult[1] = cA[1] ^ cB[0]

	return result
}

// not homomorphic
func (st *State) doDecrypt(x BitVector) BitVector {
	state := x
	var result BitVector

	cA := (*[2]uint64)(unsafe.Pointer(&state[0]))
	cB := (*[2]uint64)(unsafe.Pointer(&st.Key[0]))
	cResult := (*[2]uint64)(unsafe.Pointer(&result[0]))

	cResult[0] = cA[0] ^ cB[1]
	cResult[1] = cA[1] ^ cB[0]
	cA[0], cA[1] = cResult[0], cResult[1]
	cResult[0], cResult[1] = uint64(0), uint64(0)

	cResult[0], cResult[1] = st.decryptLong(cA[0], cB[0]), st.decryptLong(cA[1], cB[1])

	state = result
	st.invPermuteSubstitute(&state)

	return state
}

func main() {

	flag.Parse()

	if *generate {
		fmt.Println(randomString())
	} else if *encrypt != "" && *key != "" {

	} else if *decrypt != "" && *key != "" {

	} else {
		flag.Usage()
	}
}

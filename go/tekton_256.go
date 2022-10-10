package main

import (
	"math/rand"
	"unsafe"
)

type U256 [32]byte

type StateU256 struct {
	Keys   []U256
	P32    [32]int
	invP32 [32]int
	S      [256]byte
	invS   [256]byte
}

func hexToVector256(x string) U256 {
	return *(*U256)(decode(x))
}

func (key *U256) expand(P32 [32]int, S [256]byte, noRounds int) []U256 {
	var keys []U256

	for i := 1; i <= noRounds; i++ {
		newKey := *key

		hi, m2, m1, lo := newKey.longView()
		*hi <<= i
		*lo <<= i
		*m2 <<= i
		*m1 <<= i

		newKey.permute(&P32)
		newKey.substitute(&S)
		keys = append(keys, newKey)
	}

	return keys
}

func (key *U256) bootstrap() StateU256 {
	Ks := (*[4]uint64)(unsafe.Pointer(&key[0]))

	K := Ks[0] ^ Ks[1] ^ Ks[2] ^ Ks[3]

	source := rand.NewSource(int64(K))
	r := rand.New(source)

	var P32 [32]int
	var invP32 [32]int
	var S [256]byte
	var invS [256]byte

	for i := 0; i < 32; i++ {
		P32[i] = i
	}

	for i := 0; i < 256; i++ {
		S[i] = byte(i)
	}

	for i := 0; i < 65_000; i++ {
		a := r.Intn(32)
		b := r.Intn(32)
		c := r.Intn(256)
		d := r.Intn(256)

		S[c], S[d] = S[d], S[c]
		P32[a], P32[b] = P32[b], P32[a]
	}

	for i := 0; i < 32; i++ {
		invP32[P32[i]] = i
	}

	for i := 0; i < 256; i++ {
		invS[S[i]] = byte(i)
	}

	return StateU256{key.expand(P32, S, 4), P32, invP32, S, invS}

}

func (a *U256) permute(P32 *[32]int) {
	var result U256

	for i := 0; i < 32; i++ {
		result[i] = a[P32[i]]
	}

	*a = result
}

func (a *U256) substitute(S *[256]byte) {
	for i := 0; i < 16; i++ {
		a[i] = S[a[i]]
	}

}

func (a *U256) invPermute(invP32 *[32]int) {
	var result U256

	for i := 0; i < 32; i++ {
		result[i] = a[invP32[i]]
	}

	*a = result
}

func (a *U256) invSubstitute(invS *[256]byte) {
	for i := 0; i < 16; i++ {
		a[i] = invS[a[i]]
	}

}

func randomStringU256() string {
	return randomStringU128() + randomStringU128()

}

func (x *U256) longView() (*uint64, *uint64, *uint64, *uint64) {
	hi := (*uint64)(unsafe.Pointer(x))
	m2 := (*uint64)(unsafe.Pointer(uintptr(unsafe.Pointer(&x[0])) + 8*unsafe.Sizeof(x[0])))
	m1 := (*uint64)(unsafe.Pointer(uintptr(unsafe.Pointer(&x[0])) + 16*unsafe.Sizeof(x[0])))
	lo := (*uint64)(unsafe.Pointer(uintptr(unsafe.Pointer(&x[0])) + 24*unsafe.Sizeof(x[0])))

	return hi, m2, m1, lo
}

func (x *U256) xor(y *U256) {

	for i := 0; i < 32; i++ {
		x[i] ^= y[i]
	}
}

func (x *U256) diffusion() {
	hi, m2, m1, lo := x.longView()

	c2 := *m2
	c3 := *m1
	c4 := *lo

	*hi = diffusionUint64(*hi)
	*m2 = diffusionUint64(*m2)
	*m1 = diffusionUint64(*m1)
	*lo = diffusionUint64(*lo)

	*hi ^= c2
	*m1 ^= c4

	*hi ^= c3
	*m2 ^= c4
}

func (st *StateU256) doEncrypt(x U256) U256 {

	state := x

	state.diffusion()
	state.permute(&st.P32)
	state.substitute(&st.S)
	state.xor(&st.Keys[0])

	state.diffusion()
	state.permute(&st.P32)
	state.substitute(&st.S)
	state.xor(&st.Keys[1])

	state.diffusion()
	state.permute(&st.P32)
	state.substitute(&st.S)
	state.xor(&st.Keys[2])

	state.diffusion()
	state.permute(&st.P32)
	state.substitute(&st.S)
	state.xor(&st.Keys[3])

	return state

}

func (st *StateU256) doDecrypt(x U256) U256 {
	state := x

	state.xor(&st.Keys[3])
	state.invSubstitute(&st.invS)
	state.invPermute(&st.invP32)
	state.diffusion()

	state.xor(&st.Keys[2])
	state.invSubstitute(&st.invS)
	state.invPermute(&st.invP32)
	state.diffusion()

	state.xor(&st.Keys[1])
	state.invSubstitute(&st.invS)
	state.invPermute(&st.invP32)
	state.diffusion()

	state.xor(&st.Keys[0])
	state.invSubstitute(&st.invS)
	state.invPermute(&st.invP32)
	state.diffusion()

	return state
}

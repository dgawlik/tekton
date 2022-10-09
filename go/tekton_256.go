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

		newKey.permuteSubstitute(&P32, &S)
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
		a := r.Intn(16)
		b := r.Intn(16)

		c := r.Intn(256)
		d := r.Intn(256)

		P32[a], P32[b] = P32[b], P32[a]
		S[c], S[d] = S[d], S[c]
	}

	for i := 0; i < 32; i++ {
		invP32[P32[i]] = i
	}

	for i := 0; i < 256; i++ {
		invS[S[i]] = byte(i)
	}

	return StateU256{key.expand(P32, S, 3), P32, invP32, S, invS}

}

func (a *U256) permuteSubstitute(P32 *[32]int, S *[256]byte) {
	var result U256

	for i := 0; i < 32; i++ {
		result[P32[i]] = S[a[i]]
	}

	*a = result
}

func (a *U256) invPermuteSubstitute(invP32 *[32]int, invS *[256]byte) {
	var result U256

	for i := 0; i < 32; i++ {
		result[invP32[i]] = invS[a[i]]
	}

	*a = result
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

	*hi = diffusionUint64(*hi)
	*m2 = diffusionUint64(*m2)
	*m1 = diffusionUint64(*m1)
	*lo = diffusionUint64(*lo)
}

func (st *StateU256) doEncrypt(x U256) U256 {

	state := x

	state.diffusion()
	state.permuteSubstitute(&st.P32, &st.S)
	state.xor(&st.Keys[0])

	state.diffusion()
	state.permuteSubstitute(&st.P32, &st.S)
	state.xor(&st.Keys[1])

	state.diffusion()
	state.permuteSubstitute(&st.P32, &st.S)
	state.xor(&st.Keys[2])

	return state

}

func (st *StateU256) doDecrypt(x U256) U256 {
	state := x

	state.xor(&st.Keys[2])
	state.invPermuteSubstitute(&st.invP32, &st.invS)
	state.diffusion()

	state.xor(&st.Keys[1])
	state.invPermuteSubstitute(&st.invP32, &st.invS)
	state.diffusion()

	state.xor(&st.Keys[0])
	state.invPermuteSubstitute(&st.invP32, &st.invS)
	state.diffusion()

	return state
}

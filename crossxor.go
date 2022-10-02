package main

import (
	"bytes"
	"encoding/hex"
	"flag"
	"fmt"
	"math/big"
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
	Key BitVector
}

func hexToVector(x string) BitVector {
	return *(*BitVector)(decode(x))
}

func (a BitVector) xor(b BitVector) BitVector {
	var c BitVector

	for i := 0; i < 16; i++ {
		c[i] = a[i] ^ b[i]
	}

	return c
}

func (a BitVector) and(b BitVector) BitVector {
	var c BitVector

	for i := 0; i < 16; i++ {
		c[i] = a[i] & b[i]
	}

	return c
}

func (a BitVector) or(b BitVector) BitVector {
	var c BitVector

	for i := 0; i < 16; i++ {
		c[i] = a[i] | b[i]
	}

	return c
}

func (a BitVector) sum(b BitVector) BitVector {
	var c BitVector

	carry := 0
	for i := 0; i < 16; i++ {
		s := int(a[i]) + int(b[i]) + carry
		carry = s / 256
		c[i] = byte(s)
	}

	return c
}

func (a BitVector) sub(b BitVector) BitVector {
	var c BitVector

	var bigA big.Int
	bigA.SetBytes(a[:])

	var bigB big.Int
	bigB.SetBytes(b[:])

	var bigC big.Int

	bigC.Sub(&bigB, &bigA)
	bigC.FillBytes(c[:])

	return c
}

// func (a BitVector) crossProd(b BitVector, i int) BitVector {

// 	// if i >= 16 {
// 	// 	return a.crossProdBig(b, i)
// 	// } else if i == 2 {
// 	// 	return a.crossProd2(b, i)
// 	// } else if i == 4 {
// 	// 	return a.crossProd4(b, i)
// 	// } else {
// 	// 	return a.crossProd8(b, i)
// 	// }

// 	// var result BitVector

// 	// blocks := 128 / i
// 	// blockSize := i

// 	// for j := 0; j < blocks; j++ {
// 	// 	for k := 0; k < blockSize/2; k++ {
// 	// 		iSrc := j*blockSize + k
// 	// 		iDest := j*blockSize + blockSize/2 + k

// 	// 		if (!HasBit(a[iSrc/8], iSrc%8) && HasBit(b[iDest/8], iDest%8)) ||
// 	// 			(HasBit(a[iSrc/8], iSrc%8) && !HasBit(b[iDest/8], iDest%8)) {
// 	// 			result[iSrc/8] = SetBit(result[iSrc/8], iSrc%8)
// 	// 		}

// 	// 		if (!HasBit(b[iSrc/8], iSrc%8) && HasBit(a[iDest/8], iDest%8)) ||
// 	// 			(HasBit(b[iSrc/8], iSrc%8) && !HasBit(a[iDest/8], iDest%8)) {
// 	// 			result[iDest/8] = SetBit(result[iDest/8], iDest%8)
// 	// 		}
// 	// 	}
// 	// }

// 	// return result
// }

// func (a BitVector) crossProdBig(b BitVector, i int) BitVector {
// 	var result BitVector

// 	blocks := 128 / i
// 	blockSize := i / 8

// 	for j := 0; j < blocks; j++ {
// 		for k := 0; k < blockSize/2; k++ {
// 			iSrc := j*blockSize + k
// 			iDest := j*blockSize + blockSize/2 + k

// 			result[iSrc] = a[iSrc] ^ b[iDest]
// 			result[iDest] = a[iDest] ^ b[iSrc]
// 		}
// 	}

// 	return result

// }

func (a *BitVector) crossProd16(b *BitVector) BitVector {
	var result BitVector

	cA := (*[2]uint64)(unsafe.Pointer(&a[0]))
	cB := (*[2]uint64)(unsafe.Pointer(&b[0]))
	cResult := (*[2]uint64)(unsafe.Pointer(&result[0]))

	maskOdd := uint64(0b11111111_00000000_11111111_00000000_11111111_00000000_11111111_00000000)
	maskEven := uint64(0b00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111)

	for j := 0; j < 2; j++ {
		cResult[j] |= (cA[j] ^ (cB[j] << 8)) & maskOdd
		cResult[j] |= (cA[j] ^ (cB[j] >> 8)) & maskEven
	}

	return result
}

func (a *BitVector) crossProd32(b *BitVector) BitVector {
	var result BitVector

	cA := (*[2]uint64)(unsafe.Pointer(&a[0]))
	cB := (*[2]uint64)(unsafe.Pointer(&b[0]))
	cResult := (*[2]uint64)(unsafe.Pointer(&result[0]))

	maskOdd := uint64(0b11111111_11111111_00000000_00000000_11111111_11111111_00000000_00000000)
	maskEven := uint64(0b00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111)

	for j := 0; j < 2; j++ {
		cResult[j] |= (cA[j] ^ (cB[j] << 16)) & maskOdd
		cResult[j] |= (cA[j] ^ (cB[j] >> 16)) & maskEven
	}

	return result
}

func (a *BitVector) crossProd64(b *BitVector) BitVector {
	var result BitVector

	cA := (*[2]uint64)(unsafe.Pointer(&a[0]))
	cB := (*[2]uint64)(unsafe.Pointer(&b[0]))
	cResult := (*[2]uint64)(unsafe.Pointer(&result[0]))

	maskOdd := uint64(0b11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000)
	maskEven := uint64(0b00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111)

	for j := 0; j < 2; j++ {
		cResult[j] |= (cA[j] ^ (cB[j] << 32)) & maskOdd
		cResult[j] |= (cA[j] ^ (cB[j] >> 32)) & maskEven
	}

	return result
}

func (a *BitVector) crossProd128(b *BitVector) BitVector {
	var result BitVector

	cA := (*[2]uint64)(unsafe.Pointer(&a[0]))
	cB := (*[2]uint64)(unsafe.Pointer(&b[0]))
	cResult := (*[2]uint64)(unsafe.Pointer(&result[0]))

	cResult[0] = cA[0] ^ cB[1]
	cResult[1] = cA[1] ^ cB[0]

	return result
}

func (a *BitVector) crossProd2(b *BitVector) BitVector {
	var result BitVector

	cA := (*[2]uint64)(unsafe.Pointer(&a[0]))
	cB := (*[2]uint64)(unsafe.Pointer(&b[0]))
	cResult := (*[2]uint64)(unsafe.Pointer(&result[0]))

	maskOdd := uint64(0b10101010_10101010_10101010_10101010_10101010_10101010_10101010_10101010)
	maskEven := uint64(0b01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101)

	for j := 0; j < 2; j++ {
		cResult[j] |= (cA[j] ^ (cB[j] << 1)) & maskOdd
		cResult[j] |= (cA[j] ^ (cB[j] >> 1)) & maskEven
	}

	return result
}

func (a *BitVector) crossProd4(b *BitVector) BitVector {
	var result BitVector

	cA := (*[2]uint64)(unsafe.Pointer(&a[0]))
	cB := (*[2]uint64)(unsafe.Pointer(&b[0]))
	cResult := (*[2]uint64)(unsafe.Pointer(&result[0]))

	maskOdd := uint64(0b11001100_11001100_11001100_11001100_11001100_11001100_11001100_11001100)
	maskEven := uint64(0b00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011)

	for j := 0; j < 2; j++ {
		cResult[j] |= (cA[j] ^ (cB[j] << 2)) & maskOdd
		cResult[j] |= (cA[j] ^ (cB[j] >> 2)) & maskEven
	}

	return result
}

func (a *BitVector) crossProd8(b *BitVector) BitVector {
	var result BitVector

	cA := (*[2]uint64)(unsafe.Pointer(&a[0]))
	cB := (*[2]uint64)(unsafe.Pointer(&b[0]))
	cResult := (*[2]uint64)(unsafe.Pointer(&result[0]))

	maskOdd := uint64(0b11110000_11110000_11110000_11110000_11110000_11110000_11110000_11110000)
	maskEven := uint64(0b00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111)

	for j := 0; j < 2; j++ {
		cResult[j] |= (cA[j] ^ (cB[j] << 4)) & maskOdd
		cResult[j] |= (cA[j] ^ (cB[j] >> 4)) & maskEven
	}

	return result
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

// not homomorphic
func (st *State) doEncrypt(x BitVector) BitVector {

	state := x
	state = state.crossProd2(&st.Key)
	state = state.crossProd4(&st.Key)
	state = state.crossProd8(&st.Key)
	state = state.crossProd16(&st.Key)
	state = state.crossProd32(&st.Key)
	state = state.crossProd64(&st.Key)
	state = state.crossProd128(&st.Key)

	return state
}

// not homomorphic
func (st *State) doDecrypt(x BitVector) BitVector {

	state := x
	state = state.crossProd128(&st.Key)
	state = state.crossProd64(&st.Key)
	state = state.crossProd32(&st.Key)
	state = state.crossProd16(&st.Key)
	state = state.crossProd8(&st.Key)
	state = state.crossProd4(&st.Key)
	state = state.crossProd2(&st.Key)

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

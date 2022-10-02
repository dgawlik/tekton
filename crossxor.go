package main

import (
	"bytes"
	"encoding/hex"
	"flag"
	"fmt"
	"math/big"
	"math/rand"
	"time"
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

	for i := 0; i < 16; i += 2 {
		result[i] = a[i] ^ b[i+1]
		result[i+1] = a[i+1] ^ b[i]
	}

	return result
}

func (a *BitVector) crossProd32(b *BitVector) BitVector {
	var result BitVector

	for i := 0; i < 16; i += 4 {
		result[i] = a[i] ^ b[i+2]
		result[i+2] = a[i+2] ^ b[i]

		result[i+1] = a[i+1] ^ b[i+3]
		result[i+3] = a[i+3] ^ b[i+1]
	}

	return result
}

func (a *BitVector) crossProd64(b *BitVector) BitVector {
	var result BitVector

	for i := 0; i < 16; i += 8 {
		result[i] = a[i] ^ b[i+4]
		result[i+4] = a[i+4] ^ b[i]

		result[i+1] = a[i+1] ^ b[i+5]
		result[i+5] = a[i+5] ^ b[i+1]

		result[i+2] = a[i+2] ^ b[i+6]
		result[i+6] = a[i+6] ^ b[i+2]

		result[i+3] = a[i+3] ^ b[i+7]
		result[i+7] = a[i+7] ^ b[i+3]

	}

	return result
}

func (a *BitVector) crossProd128(b *BitVector) BitVector {
	var result BitVector

	result[0] = a[0] ^ b[8]
	result[8] = a[8] ^ b[0]

	result[1] = a[1] ^ b[9]
	result[9] = a[9] ^ b[1]

	result[2] = a[2] ^ b[10]
	result[10] = a[10] ^ b[2]

	result[3] = a[3] ^ b[11]
	result[11] = a[11] ^ b[3]

	result[4] = a[4] ^ b[12]
	result[12] = a[12] ^ b[4]

	result[5] = a[5] ^ b[13]
	result[13] = a[13] ^ b[5]

	result[6] = a[6] ^ b[14]
	result[14] = a[14] ^ b[6]

	result[7] = a[7] ^ b[15]
	result[15] = a[15] ^ b[7]

	return result
}

func (a *BitVector) crossProd2(b *BitVector) BitVector {
	var result BitVector

	maskOdd := byte(0b10101010)
	maskEven := byte(0b01010101)

	for j := 0; j < 16; j++ {
		result[j] |= (a[j] ^ (b[j] << 1)) & maskOdd
		result[j] |= (a[j] ^ (b[j] >> 1)) & maskEven
	}

	return result
}

func (a *BitVector) crossProd4(b *BitVector) BitVector {
	var result BitVector

	maskOdd := byte(0b11001100)
	maskEven := byte(0b00110011)

	for j := 0; j < 16; j++ {
		result[j] |= (a[j] ^ (b[j] << 2)) & maskOdd
		result[j] |= (a[j] ^ (b[j] >> 2)) & maskEven
	}

	return result
}

func (a *BitVector) crossProd8(b *BitVector) BitVector {
	var result BitVector

	maskOdd := byte(0b11110000)
	maskEven := byte(0b00001111)

	for j := 0; j < 16; j++ {
		result[j] |= (a[j] ^ (b[j] << 4)) & maskOdd
		result[j] |= (a[j] ^ (b[j] >> 4)) & maskEven
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

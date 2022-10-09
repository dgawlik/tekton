package main

import (
	"crypto/aes"
	"encoding/hex"
	"fmt"
	"reflect"
	"testing"
	"time"
)

func TestEncryptDecypt(t *testing.T) {
	key := hexToVector("a291a728727ac647a53193be9583c504")

	st := key.bootstrap()

	for i := 0; i < 10_000_00; i++ {
		A := hexToVector(randomStringU128())
		cipherA := st.doEncrypt(A)
		A2 := st.doDecrypt(cipherA)

		if !reflect.DeepEqual(A, A2) {
			t.Fail()
		}
	}
}

func TestTimeOfEncryption(t *testing.T) {
	key := hexToVector("a291a728727ac647a53193be9583c504")

	st := key.bootstrap()

	var totalTime time.Duration
	for i := 0; i < 10_000_000; i++ {
		s := randomStringU128()

		v := hexToVector(s)
		start := time.Now()
		_ = st.doEncrypt(v)
		totalTime += time.Since(start)
	}

	fmt.Printf("CrossXor encryption 10M nonces %s\n", totalTime)
}

func TestTimeOfDecryption(t *testing.T) {
	key := hexToVector("a291a728727ac647a53193be9583c504")

	st := key.bootstrap()

	var totalTime time.Duration
	for i := 0; i < 10_000_000; i++ {
		s := randomStringU128()

		v := hexToVector(s)
		start := time.Now()
		_ = st.doDecrypt(v)
		totalTime += time.Since(start)
	}

	fmt.Printf("CrossXor decryption 10M nonces %s\n", totalTime)
}

func TestTimeOfEncryptionAes(t *testing.T) {
	key := "a291a728727ac647a53193be9583c504"
	hexKey, _ := hex.DecodeString(key)

	aes, _ := aes.NewCipher(hexKey)

	var totalTime time.Duration
	for i := 0; i < 10_000_000; i++ {
		s := randomStringU128()
		hexS, err := hex.DecodeString(s)
		if err != nil {
			panic(err)
		}

		cipher := make([]byte, 16)
		start := time.Now()
		aes.Encrypt(cipher, hexS)
		totalTime += time.Since(start)
	}

	fmt.Printf("AES encryption 10M nonces %s\n", totalTime)
}

func TestTimeOfDecryptionAes(t *testing.T) {
	key := "a291a728727ac647a53193be9583c504"
	hexKey, _ := hex.DecodeString(key)

	aes, _ := aes.NewCipher(hexKey)

	var totalTime time.Duration
	for i := 0; i < 10_000_000; i++ {
		s := randomStringU128()
		hexS, err := hex.DecodeString(s)
		if err != nil {
			panic(err)
		}

		cipher := make([]byte, 16)
		start := time.Now()
		aes.Decrypt(cipher, hexS)
		totalTime += time.Since(start)
	}

	fmt.Printf("AES decryption 10M nonces %s\n", totalTime)
}

package main

import (
	"crypto/aes"
	"encoding/hex"
	"fmt"
	"reflect"
	"testing"
	"time"
)

func TestEncryptDecypt128(t *testing.T) {
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

func TestEncryptDecypt256(t *testing.T) {
	key := hexToVector256("a291a728727ac647a53193be9583c504a291a728727ac647a53193be9583c504")

	st := key.bootstrap()

	for i := 0; i < 10_000_00; i++ {
		A := hexToVector256(randomStringU256())
		cipherA := st.doEncrypt(A)
		A2 := st.doDecrypt(cipherA)

		if !reflect.DeepEqual(A, A2) {
			t.Fail()
		}
	}
}

func TestTimeOfEncryption(t *testing.T) {
	key := hexToVector("a291a728727ac647a53193be9583c504")
	key2 := hexToVector256("a291a728727ac647a53193be9583c504a291a728727ac647a53193be9583c504")

	st := key.bootstrap()
	st2 := key2.bootstrap()

	var totalTime time.Duration
	var totalTime2 time.Duration
	for i := 0; i < 2_000_000; i++ {
		s := randomStringU128()
		s2 := randomStringU256()

		v := hexToVector(s)
		start := time.Now()
		_ = st.doEncrypt(v)
		totalTime += time.Since(start)

		v2 := hexToVector256(s2)
		start2 := time.Now()
		_ = st2.doEncrypt(v2)
		totalTime2 += time.Since(start2)
	}

	fmt.Printf("Tekon encryption 2M nonces (128-bit) %s\n", totalTime)
	fmt.Printf("Tekon encryption 2M nonces (256-bit) %s\n", totalTime2)
}

func TestTimeOfDecryption(t *testing.T) {
	key := hexToVector("a291a728727ac647a53193be9583c504")
	key2 := hexToVector256("a291a728727ac647a53193be9583c504a291a728727ac647a53193be9583c504")

	st := key.bootstrap()
	st2 := key2.bootstrap()

	var totalTime time.Duration
	var totalTime2 time.Duration
	for i := 0; i < 2_000_000; i++ {
		s := randomStringU128()
		s2 := randomStringU256()

		v := hexToVector(s)
		start := time.Now()
		_ = st.doDecrypt(v)
		totalTime += time.Since(start)

		v2 := hexToVector256(s2)
		start2 := time.Now()
		_ = st2.doDecrypt(v2)
		totalTime2 += time.Since(start2)
	}

	fmt.Printf("Tekon decryption 2M nonces (128-bit) %s\n", totalTime)
	fmt.Printf("Tekon decryption 2M nonces (256-bit) %s\n", totalTime2)
}
func TestTimeOfEncryptionAes(t *testing.T) {
	key := "a291a728727ac647a53193be9583c504"
	hexKey, _ := hex.DecodeString(key)

	aes, _ := aes.NewCipher(hexKey)

	var totalTime time.Duration
	for i := 0; i < 2_000_000; i++ {
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

	fmt.Printf("AES encryption 2M nonces %s\n", totalTime)
}

func TestTimeOfDecryptionAes(t *testing.T) {
	key := "a291a728727ac647a53193be9583c504"
	hexKey, _ := hex.DecodeString(key)

	aes, _ := aes.NewCipher(hexKey)

	var totalTime time.Duration
	for i := 0; i < 2_000_000; i++ {
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

	fmt.Printf("AES decryption 2M nonces %s\n", totalTime)
}

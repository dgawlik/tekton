package main

import (
	"fmt"
	"reflect"
	"testing"
	"time"
)

func TestEncryptDecypt(t *testing.T) {
	key := hexToVector("a291a728727ac647a53193be9583c504")

	st := State{key}

	for i := 0; i < 1_000_00; i++ {
		A := hexToVector(randomString())
		cipherA := st.doEncrypt(A)
		A2 := st.doDecrypt(cipherA)

		if !reflect.DeepEqual(A, A2) {
			t.Fail()
		}
	}
}

func TestTimeOfEncryption(t *testing.T) {
	key := "a291a728727ac647a53193be9583c504"

	st := State{hexToVector(key)}

	var totalTime time.Duration
	for i := 0; i < 1_000_000; i++ {
		s := randomString()

		v := hexToVector(s)
		start := time.Now()
		_ = st.doEncrypt(v)
		totalTime += time.Since(start)
	}

	fmt.Printf("CrossXor encryption 1M nonces %s\n", totalTime)
}

func TestTimeOfDecryption(t *testing.T) {
	key := "a291a728727ac647a53193be9583c504"

	st := State{hexToVector(key)}

	var totalTime time.Duration
	for i := 0; i < 1_000_000; i++ {
		s := randomString()

		v := hexToVector(s)
		start := time.Now()
		_ = st.doDecrypt(v)
		totalTime += time.Since(start)
	}

	fmt.Printf("CrossXor decryption 1M nonces %s\n", totalTime)
}

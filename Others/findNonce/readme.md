


```go
package main

import (
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"strings"
)

func findNonce(input []byte) uint64 {
	var nonce uint64
	hash := sha256.New()
	buf := make([]byte, 8) // buffer for nonce bytes

	for nonce = 0; nonce <= 1e6; nonce++ {
		hash.Reset()
		hash.Write(input)
		binary.LittleEndian.PutUint64(buf, nonce)
		hash.Write(buf)
		sum := hash.Sum(nil)
		hexStr := hex.EncodeToString(sum)
		if len(hexStr) >= 2 && hexStr[0] == '0' && hexStr[1] == '0' {
			return nonce
		}
	}
	return 0
}

func hexStringToBytes(hexStr string) []byte {
	hexStr = strings.ReplaceAll(hexStr, " ", "")
	bytes, err := hex.DecodeString(hexStr)
	if err != nil {
		panic(err)
	}
	return bytes
}

func main() {
	// Example 1: From the problem statement
	input1 := "12 DE FF"
	bytes1 := hexStringToBytes(input1)
	nonce1 := findNonce(bytes1)
	fmt.Printf("Input: %s\nNonce: %X\n\n", input1, nonce1)

	// Example 2: From the problem statement
	input2 := "AB AB"
	bytes2 := hexStringToBytes(input2)
	nonce2 := findNonce(bytes2)
	fmt.Printf("Input: %s\nNonce: %X\n", input2, nonce2)
}
```
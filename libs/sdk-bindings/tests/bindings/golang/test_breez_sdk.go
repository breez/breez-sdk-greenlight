package main

import (
	"log"
	"example.org/golang/breezsdk"
)

func main() {
    if seed, err := breezsdk.MnemonicToSeed("cruise clever syrup coil cute execute laundry general cover prevent law sheriff"); err != nil {
		log.Fatalf("MnemonicToSeed failed; %#v", err)
	}
}
package main

import (
	"example.org/golang/breez_sdk"
	"log"
)

type BreezListener struct{}

func (BreezListener) Log(l breez_sdk.LogEntry) {
	log.Print(l.Line)
}

func (BreezListener) OnEvent(e breez_sdk.BreezEvent) {
	log.Printf("%#v", e)
}

func main() {
	breezListener := BreezListener{}

	breez_sdk.SetLogStream(breezListener)

	seed, err := breez_sdk.MnemonicToSeed("cruise clever syrup coil cute execute laundry general cover prevent law sheriff")

	if err != nil {
		log.Fatalf("MnemonicToSeed failed: %#v", err)
	}

	credentials, err := breez_sdk.RecoverNode(breez_sdk.NetworkBitcoin, seed)

	if err != nil {
		log.Fatalf("RecoverNode failed: %#v", err)
	}

	sdkServices, err := breez_sdk.InitServices(breez_sdk.DefaultConfig(breez_sdk.EnvironmentTypeStaging), seed, credentials, breezListener)

	if err != nil {
		log.Fatalf("InitServices failed: %#v", err)
	}

	if err := sdkServices.Start(); err != nil {
		log.Fatalf("Start failed: %#v", err)
	}

	nodeInfo, err := sdkServices.NodeInfo()

	if err != nil {
		log.Fatalf("NodeInfo failed: %#v", err)
	}

	log.Print(nodeInfo.Id)
}

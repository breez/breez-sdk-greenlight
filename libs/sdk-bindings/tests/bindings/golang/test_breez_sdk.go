package main

import (
	"log"

	"example.org/golang/breez_sdk"
)

type BreezListener struct{}

func (BreezListener) Log(l breez_sdk.LogEntry) {
	log.Print(l.Line)
}

func (BreezListener) OnEvent(e breez_sdk.BreezEvent) {
	//log.Printf("%#v", e)
}

func main() {
	breezListener := BreezListener{}

	breez_sdk.SetLogStream(breezListener)

	seed, err := breez_sdk.MnemonicToSeed("cruise clever syrup coil cute execute laundry general cover prevent law sheriff")

	if err != nil {
		log.Fatalf("MnemonicToSeed failed: %#v", err)
	}

	inviteCode := "code"
	config := breez_sdk.DefaultConfig(breez_sdk.EnvironmentTypeStaging, "", breez_sdk.NodeConfigGreenlight{Config: breez_sdk.GreenlightNodeConfig{PartnerCredentials: nil, InviteCode: &inviteCode}})
	sdkServices, err := breez_sdk.Connect(config, seed, breezListener)

	if err != nil {
		log.Fatalf("Connect failed: %#v", err)
	}

	nodeInfo, err := sdkServices.NodeInfo()

	if err != nil {
		log.Fatalf("NodeInfo failed: %#v", err)
	}

	log.Print(nodeInfo.Id)
}

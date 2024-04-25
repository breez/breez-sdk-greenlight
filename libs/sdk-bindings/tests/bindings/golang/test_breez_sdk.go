package main

import (
	"fmt"
	"log"

	"example.org/golang/breez_sdk"
)

type BreezListener struct{}

func (BreezListener) Log(l breez_sdk.LogEntry) {
	if l.Level != "TRACE" {
		fmt.Printf("%v\n", l.Line)
	}
}

func (BreezListener) OnEvent(e breez_sdk.BreezEvent) {
	fmt.Printf("%#v", e)
}

func main() {
	breezListener := BreezListener{}

	breez_sdk.SetLogStream(breezListener)

	seed, err := breez_sdk.MnemonicToSeed("repeat hawk combine screen network rhythm ritual social neither casual volcano powder")

	if err != nil {
		log.Fatalf("MnemonicToSeed failed: %#v", err)
	}

	config := breez_sdk.DefaultConfig(breez_sdk.EnvironmentTypeProduction, "code", breez_sdk.NodeConfigGreenlight{
		Config: breez_sdk.GreenlightNodeConfig{
			PartnerCredentials: nil,
			InviteCode:         nil,
		},
	})
	connectRequest := breez_sdk.ConnectRequest{Config: config, Seed: seed}
	sdkServices, err := breez_sdk.Connect(connectRequest, breezListener)

	if err != nil {
		log.Fatalf("Connect failed: %#v", err)
	}

	nodeInfo, err := sdkServices.NodeInfo()

	if err != nil {
		log.Fatalf("NodeInfo failed: %#v", err)
	}

	log.Print(nodeInfo.Id)
}

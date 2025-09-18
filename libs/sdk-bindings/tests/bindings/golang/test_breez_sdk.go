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

	filterLevel := breez_sdk.LevelFilterTrace

	breez_sdk.SetLogStream(breezListener, &filterLevel)

	seed, sdkErr := breez_sdk.MnemonicToSeed("repeat hawk combine screen network rhythm ritual social neither casual volcano powder")

	if sdkErr != nil {
		log.Fatalf("MnemonicToSeed failed: %#v", sdkErr)
	}

	config := breez_sdk.DefaultConfig(breez_sdk.EnvironmentTypeProduction, "code", breez_sdk.NodeConfigGreenlight{
		Config: breez_sdk.GreenlightNodeConfig{
			PartnerCredentials: nil,
			InviteCode:         nil,
		},
	})
	connectRequest := breez_sdk.ConnectRequest{Config: config, Seed: seed}
	sdkServices, connectErr := breez_sdk.Connect(connectRequest, breezListener)

	if connectErr != nil {
		log.Fatalf("Connect failed: %#v", connectErr)
	}

	nodeInfo, sdkErr := sdkServices.NodeInfo()

	if sdkErr != nil {
		log.Fatalf("NodeInfo failed: %#v", sdkErr)
	}

	log.Print(nodeInfo.Id)
}

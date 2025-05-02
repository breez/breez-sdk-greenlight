import breez_sdk

class SDKListener(breez_sdk.EventListener):
   def on_event(self, event):
      print(event)


seed = breez_sdk.mnemonic_to_seed("repeat hawk combine screen network rhythm ritual social neither casual volcano powder");
config = breez_sdk.default_config(breez_sdk.EnvironmentType.PRODUCTION, "code", breez_sdk.NodeConfig.GREENLIGHT(breez_sdk.GreenlightNodeConfig(partner_credentials=None, invite_code=None)))
connect_request = breez_sdk.ConnectRequest(config=config, seed=seed)
sdk_services = breez_sdk.connect(connect_request, SDKListener())     
node_info = sdk_services.node_info()    
assert node_info.id == "027e2b899f9f75b92a1ad210da21d74e7314e3499375213a71c6bf3e1b4b4394a1"

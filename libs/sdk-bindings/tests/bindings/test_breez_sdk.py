import unittest
import breez_sdk

class SDKListener(breez_sdk.EventListener):
   def on_event(self, event):
      print(event)


def test():        
     seed = breez_sdk.mnemonic_to_seed("repeat hawk combine screen network rhythm ritual social neither casual volcano powder");
     config = breez_sdk.default_config(breez_sdk.EnvironmentType.PRODUCTION, "code", breez_sdk.NodeConfig.GREENLIGHT(breez_sdk.GreenlightNodeConfig(None, None)))
     connect_request = breez_sdk.ConnectRequest(config, seed)
     sdk_services = breez_sdk.connect(connect_request, SDKListener())     
     node_info = sdk_services.node_info()    
     print(node_info)      

test()  
import unittest
import breez_sdk

class SDKListener(breez_sdk.EventListener):
   def on_event(self, event):
      print(event)


def test():        
     seed = breez_sdk.mnemonic_to_seed("cruise clever syrup coil cute execute laundry general cover prevent law sheriff");
     config = breez_sdk.default_config(breez_sdk.EnvironmentType.STAGING, "code",  breez_sdk.NodeConfig.GREENLIGHT(breez_sdk.GreenlightNodeConfig(None, "")))
     sdk_services = breez_sdk.connect(config, seed, SDKListener())     
     node_info = sdk_services.node_info()    
     print(node_info)      

test()  
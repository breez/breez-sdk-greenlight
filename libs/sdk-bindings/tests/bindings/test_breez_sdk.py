import unittest
import breez_sdk

class SDKListener(breez_sdk.EventListener):
   def on_event(self, event):
      print(event)


def test():        
     seed = breez_sdk.mnemonic_to_seed("cruise clever syrup coil cute execute laundry general cover prevent law sheriff");
     credentials = breez_sdk.recover_node(breez_sdk.Network.BITCOIN, seed)
     sdk_services = breez_sdk.init_services(breez_sdk.default_config(breez_sdk.EnvironmentType.STAGING), seed, credentials, SDKListener())
     sdk_services.start()
     node_info = sdk_services.node_info()    
     print(node_info)   
      

test()  
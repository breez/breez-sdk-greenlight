
using BreezSDK;

try
{
 var seed = BreezSdkMethods.MnemonicToSeed("cruise clever syrup coil cute execute laundry general cover prevent law sheriff");
 BreezSdkMethods.SetLogStream(new LogStreamListener());
 var credentials = BreezSdkMethods.RecoverNode(Network.BITCOIN, seed);
 BlockingBreezServices sdkServices = BreezSdkMethods.InitServices(BreezSdkMethods.DefaultConfig(EnvironmentType.STAGING), seed, credentials, new SDKListener());
 sdkServices.Start();
 NodeState? nodeInfo = sdkServices.NodeInfo();
 Console.WriteLine(nodeInfo.id);
}
catch (Exception e)
{
 Console.WriteLine(e.Message);
}

class SDKListener : EventListener
{
 public void OnEvent(BreezEvent e)
 {
  Console.WriteLine("received event " + e);
 }
}

class LogStreamListener : LogStream
{
 public void Log(LogEntry l)
 {
  Console.WriteLine(l.line);
 }
}

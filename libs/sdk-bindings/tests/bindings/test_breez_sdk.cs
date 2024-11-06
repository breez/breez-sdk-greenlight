
using breez.breez_sdk;

try
{
 var seed = BreezSdkMethods.MnemonicToSeed("repeat hawk combine screen network rhythm ritual social neither casual volcano powder");
 BreezSdkMethods.SetLogStream(new LogStreamListener(), LevelFilter.TRACE);
 var config = BreezSdkMethods.DefaultConfig(EnvironmentType.PRODUCTION, "code", new NodeConfig.Greenlight(new GreenlightNodeConfig(null, null)));
 var connectRequest = new ConnectRequest(config, seed);
 BlockingBreezServices sdkServices = BreezSdkMethods.Connect(connectRequest, new SDKListener());
 NodeState? nodeInfo = sdkServices.NodeInfo();
 Console.WriteLine(nodeInfo!.id);
}
catch (Exception e)
{
 Console.WriteLine(e.Message);
 throw;
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
  if (l.level != "TRACE") {
   Console.WriteLine(l.line);
  }
 }
}

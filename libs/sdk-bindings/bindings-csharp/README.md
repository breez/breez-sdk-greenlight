# SDK bindings for C#.Net

## Usage
```
dotnet add package Breez.Sdk
```

## Create a package
Run the GitHub workflow 'Publish C# Bindings' when creating a new release of Breez SDK.
It will create an artifact containing a zip file with the nuget package in it.

## Publish package

- Manually upload the package to the breeztech nuget organization, or
- `dotnet nuget push ./bin/Debug/Breez.Sdk.{version}.nupkg --api-key PUT-API-KEY-HERE --source https://api.nuget.org/v3/index.json`
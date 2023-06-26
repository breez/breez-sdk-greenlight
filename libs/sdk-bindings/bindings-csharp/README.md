# SDK bindings for C#.Net

## Publish a package
### Prerequisites
- Access to the [breeztech](https://www.nuget.org/profiles/breeztech) nuget organization.
- The signing certificate for nuget packages from breez.
- Timestamper server url.

### Process

#### Build dependencies

Make sure sdk-core is freshly built for all platforms.

#### Bump version

The `<Version>` element in `Breez.Sdk.csproj`

#### Build the project

from the `bindings-csharp` directory run the following command.

```
dotnet pack
```

This creates a file `./bin/Debug/Breez.Sdk.{version}.nupkg`

#### Sign package

```
dotnet nuget sign ./bin/Debug/Breez.Sdk.{version}.nupkg --certificate-path /path/to/signing/certificate.pfx --certificate-password "the password" --timestamper http://timestamping-url
```

#### Publish package

- Manually upload the package to the breeztech nuget organization, or
- `dotnet nuget push ./bin/Debug/Breez.Sdk.{version}.nupkg --api-key PUT-API-KEY-HERE --source https://api.nuget.org/v3/index.json`
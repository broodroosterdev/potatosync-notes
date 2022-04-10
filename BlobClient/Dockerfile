FROM mcr.microsoft.com/dotnet/runtime:6.0 AS base
WORKDIR /app

FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build
WORKDIR /src
COPY ["BlobClient/BlobClient.csproj", "BlobClient/"]
RUN dotnet restore "BlobClient/BlobClient.csproj"
COPY . .
WORKDIR "/src/BlobClient"
RUN dotnet build "BlobClient.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "BlobClient.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "BlobClient.dll"]

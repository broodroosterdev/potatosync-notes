// See https://aka.ms/new-console-template for more information

using System.Diagnostics;
using Google.Protobuf;
using Google.Protobuf.WellKnownTypes;
using Grpc.Core;
using Grpc.Net.Client;
using GrpcBlobClient;

using var channel = GrpcChannel.ForAddress("https://localhost:7028");
var client = new Blob.BlobClient(channel);
var headers = new Metadata();
var token =
    "";
headers.Add("Authorization", $"Bearer {token}");

for (int i = 0; i < 100; i++)
{
    Stopwatch timer = new Stopwatch();
    timer.Start();
    var reply = await client.SyncAsync(new Data()
    {
        Id = Guid.NewGuid().ToString(),
        BlobType = "note",
        Content = ByteString.FromBase64("YnJ1aA=="),
        LastChanged = DateTime.UtcNow.ToTimestamp().Seconds
    }, headers: headers);
    timer.Stop();
    TimeSpan timeTaken = timer.Elapsed;
    Console.WriteLine(timeTaken.Milliseconds); 
}


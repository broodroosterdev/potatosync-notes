using System.Security.Claims;
using Google.Protobuf;
using Google.Protobuf.WellKnownTypes;
using Grpc.Core;
using LeafletAPI.Models;
using LeafletAPI.Repositories;
using Microsoft.AspNetCore.Authorization;

namespace LeafletAPI.Services;

[Authorize]
public class BlobService : Blob.BlobBase
{
    private readonly ILogger<BlobService> _logger;
    private readonly BlobRepository _repo;

    public BlobService(ILogger<BlobService> logger, BlobDbContext context)
    {
        _logger = logger;
        _repo = new BlobRepository(context);
    }

    private static Guid GetAccountId(ServerCallContext context)
    {
        var user = context.GetHttpContext().User;
        var accountId = user.FindFirst(ClaimTypes.NameIdentifier)?.Value;

        if (!Guid.TryParse(accountId, out var id))
            throw new RpcException(new Status(StatusCode.Unauthenticated, "Could not get accountId from token"));

        return id;
    }

    private static Guid ParseId(string id)
    {
        if (!Guid.TryParse(id, out var parsedId))
            throw new RpcException(new Status(StatusCode.InvalidArgument, "Invalid id given"));
        return parsedId;
    }


    public override async Task<Empty> Sync(Data request, ServerCallContext context)
    {
        var accountId = GetAccountId(context);

        await _repo.UpsertBlob(request, accountId);

        return new Empty();
    }

    public override async Task<Empty> Delete(DeleteRequest request, ServerCallContext context)
    {
        var accountId = GetAccountId(context);
        var blobId = ParseId(request.Id);

        await _repo.DeleteBlob(blobId, accountId);

        return new Empty();
    }

    public override async Task<Empty> DeleteAll(Empty request, ServerCallContext context)
    {
        var accountId = GetAccountId(context);

        await _repo.DeleteAllBlobs(accountId);

        return new Empty();
    }

    public override async Task<GetDeletedResponse> GetDeleted(GetDeletedRequest request, ServerCallContext context)
    {
        var accountId = GetAccountId(context);

        var ids = await _repo.GetBlobIds(request.BlobType, accountId);

        var givenIds = request.List.Select(ParseId);
        var removedIds = givenIds.Except(ids)
            .Select(id => id.ToString());

        var response = new GetDeletedResponse();
        response.List.AddRange(removedIds);

        return response;
    }

    public override async Task<GetUpdatedResponse> GetUpdated(GetUpdatedRequest request, ServerCallContext context)
    {
        var accountId = GetAccountId(context);

        var updatedBlobs = await _repo.GetUpdatedBlobs(request.BlobType, request.LastUpdated.ToDateTime(), accountId);

        var response = new GetUpdatedResponse();
        response.Items.AddRange(
            updatedBlobs.Select(b => new Data()
            {
                Id = b.Id.ToString(),
                BlobType = b.BlobType,
                Content = ByteString.CopyFrom(b.Content),
                LastChanged = Timestamp.FromDateTime(b.LastChanged)
            })
        );

        return response;
    }
}
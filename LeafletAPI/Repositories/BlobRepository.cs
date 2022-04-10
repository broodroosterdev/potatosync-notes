using Grpc.Core;
using LeafletAPI.Helpers;
using LeafletAPI.Models;
using Microsoft.EntityFrameworkCore;

namespace LeafletAPI.Repositories;

public class BlobRepository
{
    private readonly BlobDbContext _context;
    public BlobRepository(BlobDbContext context)
    {
        _context = context;
    }

    public async Task UpsertBlob(Data data, Guid accountId)
    {
        var blob = await _context.Blobs.FindAsync(IdHelper.ParseId(data.Id), accountId);
        if (blob == null)
        {
            blob = new BlobEntity()
            {
                Id = Guid.Parse(data.Id),
                AccountId = accountId,
                BlobType = data.BlobType,
                Content = data.Content.ToByteArray(),
                LastChanged = data.LastChanged.ToDateTime()
            };
            await _context.AddAsync(blob);
            await _context.SaveChangesAsync();
            return;
        }

        if (blob.BlobType != data.BlobType)
            throw new RpcException(new Status(StatusCode.InvalidArgument, $"The data given has a different blob_type, expected: {blob.BlobType}"));

        blob.Content = data.Content.ToByteArray();
        blob.LastChanged = data.LastChanged.ToDateTime();
        
        await _context.SaveChangesAsync();
    }

    public async Task DeleteBlob(Guid id, Guid accountId)
    {
        var blob = await _context.Blobs.FindAsync(id, accountId);
        if (blob == null)
            return;

        _context.Blobs.Remove(blob);
        await _context.SaveChangesAsync();
    }

    public async Task DeleteAllBlobs(Guid accountId)
    {
        var blobs = await _context.Blobs
            .Where(b => b.AccountId.Equals(accountId))
            .ToArrayAsync();
        
        _context.Blobs.RemoveRange(blobs);
        await _context.SaveChangesAsync();
    }

    public async Task<IEnumerable<Guid>> GetBlobIds(string blobType, Guid accountId)
    {
        var ids = await _context.Blobs
            .Where(b => b.AccountId.Equals(accountId))
            .Where(b => b.BlobType.Equals(blobType))
            .Select(b => b.Id)
            .ToArrayAsync();
        return ids;
    }

    public async Task<IEnumerable<BlobEntity>> GetUpdatedBlobs(string blobType, DateTime lastUpdated, Guid accountId)
    {
        var blobs = await _context.Blobs
            .Where(b => b.AccountId.Equals(accountId))
            .Where(b => b.BlobType.Equals(blobType))
            .Where(b => b.LastChanged > lastUpdated)
            .ToArrayAsync();

        return blobs;
    }
}
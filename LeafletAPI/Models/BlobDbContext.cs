using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.ChangeTracking;

namespace LeafletAPI.Models;

public class BlobDbContext : DbContext
{
    public BlobDbContext(DbContextOptions<BlobDbContext> options) : base(options)
    {
    }

    public DbSet<BlobEntity> Blobs { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<BlobEntity>(builder =>
        {
            // Use composite key of blob_id and account_id
            builder.HasKey(o => new { o.Id, o.AccountId });
            
            // Use reference equality instead of comparing raw byte arrays
            builder.Property(b => b.Content)
                .Metadata
                .SetValueComparer(new ValueComparer<byte[]>(
                    (obj, otherObj) => ReferenceEquals(obj, otherObj),
                    obj => obj.GetHashCode(),
                    obj => obj));
        });
    }
}
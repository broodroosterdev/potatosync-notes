using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace LeafletAPI.Models;

public class BlobEntity
{
   public Guid Id { get; init; }
   public Guid AccountId { get; init; }
   public string BlobType { get; init; }
   public byte[] Content { get; set; } 
   public DateTime LastChanged { get; set; }
}
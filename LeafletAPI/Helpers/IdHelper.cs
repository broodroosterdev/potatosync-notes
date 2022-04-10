using Grpc.Core;

namespace LeafletAPI.Helpers;

public class IdHelper
{
    public static Guid ParseId(string id)
    {
        if (!Guid.TryParse(id, out var parsedId))
            throw new RpcException(new Status(StatusCode.InvalidArgument, "Invalid id given"));
        return parsedId;
    }
}
using System.Security.Claims;
using LeafletAPI.Models;
using LeafletAPI.Services;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.EntityFrameworkCore;
using Microsoft.IdentityModel.Tokens;

var builder = WebApplication.CreateBuilder(args);

// Additional configuration is required to successfully run gRPC on macOS.
// For instructions on how to configure Kestrel and gRPC clients on macOS, visit https://go.microsoft.com/fwlink/?linkid=2099682

// Add services to the container.
builder.Services.AddGrpc();
builder.Services.AddDbContext<BlobDbContext>(options =>
{
    options.UseNpgsql(builder.Configuration.GetConnectionString("postgres"));
});

// Add authentication to the container
builder.Services.AddAuthentication(x =>
    {
        x.DefaultAuthenticateScheme = JwtBearerDefaults.AuthenticationScheme;
        x.DefaultChallengeScheme = JwtBearerDefaults.AuthenticationScheme;
    })
    .AddJwtBearer(options =>
    {
        options.Authority = "https://login.broodrooster.dev/realms/Sync-test/";
        options.Audience = "account";
    });
builder.Services.AddAuthorization();

var app = builder.Build();
app.UseRouting();

// Add authentication and authorization to app
app.UseAuthentication();
app.UseAuthorization();

// Configure the HTTP request pipeline.
app.UseEndpoints(builder =>
{
    builder.MapGrpcService<BlobService>();
    builder.MapGet("/",
        () =>
            "Communication with gRPC endpoints must be made through a gRPC client. To learn how to create a client, visit: https://go.microsoft.com/fwlink/?linkid=2086909");
});
app.Run();
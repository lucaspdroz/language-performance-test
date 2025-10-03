using CSharpApi.Data;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

// Kestrel e Porta (conf via appsettings.json também)
builder.WebHost.UseUrls("http://0.0.0.0:8080");

// Configuração EF / Npgsql
var conn = builder.Configuration.GetConnectionString("DefaultConnection")
// optionally fallback to environment var
    ?? Environment.GetEnvironmentVariable("ConnectionStrings__DefaultConnection") 
    ?? "Host=localhost;Port=5432;Database=api_dotnet;Username=postgres;Password=postgres";

builder.Services.AddDbContext<AppDbContext>(options =>
    options.UseNpgsql(conn));

// Swagger
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

builder.Services.AddControllers();

var app = builder.Build();

app.UseSwagger();
app.UseSwaggerUI(c =>
{
    c.SwaggerEndpoint("/swagger/v1/swagger.json", "Tokens API v1");
    c.RoutePrefix = "swagger"; // acessa em /swagger
});

app.MapControllers();

app.Run();

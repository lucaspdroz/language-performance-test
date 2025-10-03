using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using CSharpApi.Models;

namespace CSharpApi.Data
{
    public class AppDbContext : DbContext
    {
        public DbSet<Token> tokens { get; set; } = null!;

        public AppDbContext(DbContextOptions<AppDbContext> options) : base(options) { }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            modelBuilder.Entity<Token>().ToTable("tokens");
        }
    }
}

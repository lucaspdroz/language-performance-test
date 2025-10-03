using System;
using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace CSharpApi.Models
{
    [Table("tokens")] // usa a tabela já existente em minúsculas
    public class Token
    {
        [Key]
        [Column("id")]
        public string Id { get; set; } = Guid.NewGuid().ToString();

        // mapear explicitamente para DATE no Postgres
        [Column("date", TypeName = "date")]
        public DateTime Date { get; set; }

        // mapear para TIME no Postgres
        [Column("time", TypeName = "time")]
        public TimeSpan Time { get; set; }

        [Column("batch")]
        public string Batch { get; set; } = string.Empty;

        // created_at (TIMESTAMPTZ), DB default now() -> não setamos manualmente
        [Column("created_at")]
        public DateTimeOffset? CreatedAt { get; set; }
    }
}

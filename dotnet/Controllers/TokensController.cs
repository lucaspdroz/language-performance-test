using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using CSharpApi.Data;
using CSharpApi.Models;

namespace CSharpApi.Controllers
{
    [ApiController]
    [Route("/")]
    public class TokensController : ControllerBase
    {
        private readonly AppDbContext _db;

        public TokensController(AppDbContext db)
        {
            _db = db;
        }

        [HttpPost("encode")]
        public async Task<IActionResult> Encode([FromBody] EncodeRequest req)
        {
            // parsear data/hora com validação simples
            if (!DateTime.TryParse(req.Date, out var date))
                return BadRequest(new { error = "Invalid date format. Use YYYY-MM-DD" });

            if (!TimeSpan.TryParse(req.Time, out var time))
                return BadRequest(new { error = "Invalid time format. Use HH:MM:SS" });

            var token = new Token
            {
                Id = Guid.NewGuid().ToString(),
                Date = date.Date, // só a parte da data
                Time = time,
                Batch = req.Batch
                // CreatedAt: deixamos que o DB setará via default now()
            };

            _db.tokens.Add(token);
            await _db.SaveChangesAsync();

            var resp = new EncodeResponse
            {
                Id = token.Id,
                Date = token.Date.ToString("yyyy-MM-dd"),
                Time = token.Time.ToString(@"hh\:mm\:ss"),
                Batch = token.Batch
            };

            return Ok(resp);
        }

        [HttpGet("decode/{id}")]
        public async Task<IActionResult> Decode([FromRoute] string id)
        {
            var token = await _db.tokens.AsNoTracking().FirstOrDefaultAsync(t => t.Id == id);
            if (token == null) return NotFound();

            var resp = new EncodeResponse
            {
                Id = token.Id,
                Date = token.Date.ToString("yyyy-MM-dd"),
                Time = token.Time.ToString(@"hh\:mm\:ss"),
                Batch = token.Batch
            };

            return Ok(resp);
        }
    }
}

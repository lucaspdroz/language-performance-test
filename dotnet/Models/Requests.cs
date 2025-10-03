namespace CSharpApi.Models
{
    public class EncodeRequest
    {
        public string Date { get; set; } = ""; // "YYYY-MM-DD"
        public string Time { get; set; } = ""; // "HH:mm:ss"
        public string Batch { get; set; } = "";
    }

    public class EncodeResponse
    {
        public string Id { get; set; } = "";
        public string Date { get; set; } = "";
        public string Time { get; set; } = "";
        public string Batch { get; set; } = "";
    }
}

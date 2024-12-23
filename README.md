# Firecrawl API Server

A Rust web service that provides a REST API for web scraping and crawling using the Firecrawl API.

## Features

- `/scrape` endpoint for scraping single web pages
- `/crawl` endpoint for crawling multiple pages
- Support for multiple output formats (Markdown, HTML)
- Schema-based content extraction
- Environment-based configuration
- Comprehensive test suite

## Prerequisites

- Rust (latest stable version)
- Firecrawl API key (get one at [Firecrawl](https://firecrawl.co))

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd firecrawl-scrape-rust
```

2. Create a `.env` file in the project root:
```env
FIRECRAWL_API_KEY=your-api-key-here
```

3. Build the project:
```bash
cargo build
```

## Usage

### Starting the Server

Run the server:
```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

### API Endpoints

#### Scrape Endpoint

```http
POST /scrape
Content-Type: application/json

{
    "url": "https://example.com",
    "formats": ["markdown", "html"],
    "schema": {
        "type": "object",
        "properties": {
            "title": {"type": "string"},
            "content": {"type": "string"}
        },
        "required": ["title", "content"]
    }
}
```

- `url`: The webpage to scrape
- `formats`: Array of desired output formats ("markdown", "html", "extract")
- `schema`: Optional JSON schema for content extraction

#### Crawl Endpoint

```http
POST /crawl
Content-Type: application/json

{
    "url": "https://example.com",
    "limit": 10,
    "formats": ["markdown"]
}
```

- `url`: The starting URL for crawling
- `limit`: Maximum number of pages to crawl
- `formats`: Array of desired output formats ("markdown", "html")

## Development

### Running Tests

Create a `.env.test` file with your test API key, then run:

```bash
cargo test
```

### Environment Variables

- `FIRECRAWL_API_KEY`: Your Firecrawl API key (required)

## License

[MIT License](LICENSE)

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Support

For support, please contact [Firecrawl Support](https://firecrawl.co/support) or open an issue in this repository.

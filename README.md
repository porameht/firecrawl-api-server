# Firecrawl API Server

A Rust web service that provides a REST API for web scraping and crawling using the Firecrawl API.

## Features

- Single page scraping with the `/scrape` endpoint
- Multi-page crawling with the `/crawl` endpoint
- Multiple output formats (Markdown, HTML, Extract)
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

## API Reference

### Scrape Endpoint

The `/scrape` endpoint allows you to extract content from single web pages with optional structured data extraction.

#### Basic Request Format

```http
POST /scrape
Content-Type: application/json

{
    "url": "https://example.com",
    "formats": ["markdown", "html", "extract"],
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

Parameters:
- `url`: The webpage to scrape
- `formats`: Array of desired output formats ("markdown", "html", "extract")
- `schema`: Optional JSON schema for content extraction

#### Schema-based Content Extraction

The extract feature allows you to define JSON schemas for structured content extraction. Here are common use cases:

1. News Article Extraction:
```http
POST /scrape
Content-Type: application/json

{
    "url": "https://example.com/article",
    "formats": ["markdown", "extract"],
    "schema": {
        "type": "object",
        "properties": {
            "headline": {"type": "string"},
            "author": {"type": "string"},
            "publishDate": {"type": "string"},
            "content": {"type": "string"},
            "tags": {
                "type": "array",
                "items": {"type": "string"}
            }
        },
        "required": ["headline", "content"]
    }
}
```

2. Product Page Extraction:
```http
POST /scrape
Content-Type: application/json

{
    "url": "https://example.com/product",
    "formats": ["extract"],
    "schema": {
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "price": {"type": "number"},
            "description": {"type": "string"},
            "specifications": {
                "type": "object",
                "properties": {
                    "dimensions": {"type": "string"},
                    "weight": {"type": "string"},
                    "color": {"type": "string"}
                }
            }
        },
        "required": ["name", "price"]
    }
}
```

3. List Page Extraction:
```http
POST /scrape
Content-Type: application/json

{
    "url": "https://news.ycombinator.com",
    "formats": ["extract"],
    "schema": {
        "type": "object",
        "properties": {
            "items": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "title": {"type": "string"},
                        "points": {"type": "number"},
                        "author": {"type": "string"},
                        "url": {"type": "string"}
                    },
                    "required": ["title", "url"]
                }
            }
        },
        "required": ["items"]
    }
}
```

#### Extract Response Format

```json
{
    "extract": {
        // Extracted data matching your schema
    },
    "creditsUsed": 1
}
```

### Crawl Endpoint

The `/crawl` endpoint enables multi-page crawling starting from a seed URL.

```http
POST /crawl
Content-Type: application/json

{
    "url": "https://example.com",
    "limit": 10,
    "formats": ["markdown"]
}
```

Parameters:
- `url`: The starting URL for crawling
- `limit`: Maximum number of pages to crawl
- `formats`: Array of desired output formats ("markdown", "html", "extract")

## Best Practices

### Schema Design
1. Start with required fields only
2. Use descriptive property names
3. Add optional fields as needed
4. Use appropriate field types:
   - `string`: For text content
   - `number`: For numeric values
   - `array`: For lists
   - `object`: For nested structures

### Format Selection
1. Use `["markdown", "extract"]` to get both formatted content and structured data
2. Use `["extract"]` alone for faster responses when only structured data is needed
3. Use `["html"]` when you need to preserve the original HTML structure

### Error Handling
1. Check response status codes
2. Validate extracted data against your schema
3. Handle missing optional fields gracefully
4. Implement proper timeout handling

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

# Rust Tax Engine

A robust tax calculation engine that fetches and processes US tax brackets.

## Features

- Automated tax bracket fetching from IRS sources
- Support for US Federal tax calculations
- Type-safe decimal calculations
- Async support with proper error handling

## Usage

```rust
use tax_engine::{USFederalScraper, Jurisdiction, Country, TaxEntityType};

#[tokio::main]
async fn main() {
    let scraper = USFederalScraper::new();
    let rates = scraper
        .fetch_rates(
            &Jurisdiction::Federal(Country::USA),
            &TaxEntityType::Individual,
            2024,
        )
        .await;

    println!("Tax brackets: {:?}", rates);
}
```

## Installation

Add to your Cargo.toml:

```toml
[dependencies]
tax_engine = "0.1.0"
```

## Development

Requirements:

- Rust 1.75 or higher
- Cargo

## Testing

```bash
cargo test
```

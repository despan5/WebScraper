use reqwest;
use select::document::Document;
use select::predicate::Name;
use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Intro
    println!("Welcome to the Rust web scraper!");
    println!("You can search for words or phrases across multiple websites.");

    // Get search term
    let query = get_user_input("Enter a word or phrase to search for: ")?;

    // Get number of websites to scrape
    let num_websites = get_user_input("How many websites do you want to scrape? ")?
        .parse::<usize>()
        .unwrap_or(1); // Default to 1 if input is invalid

    // Get the list of URLs from user
    let mut urls: Vec<String> = Vec::new();
    for i in 1..=num_websites {
        let url_prompt = format!("Enter URL #{}: ", i);
        let url = get_user_input(&url_prompt)?;
        urls.push(url);
    }

    // Initialize HashMap to store how many times word is found
    let mut term_count: HashMap<String, usize> = HashMap::new();

    // Scrape each website and look for term
    for url in urls {
        println!("\nScraping: {}", url);

        match scrape_website(&url, &query) {
            Ok(count) => {
                println!("Found '{}' {} times on {}.", query, count, url);
                *term_count.entry(url).or_insert(0) += count;
            }
            Err(e) => println!("Error scraping {}: {}", url, e),
        }
    }

    // Display
    println!("\nSummary of results:");
    for (url, count) in term_count {
        println!("URL: {}, '{}' found {} times", url, query, count);
    }

    Ok(())
}

// Function to get user input and trim whitespace
fn get_user_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// Function to scrape a website and return the number times found
fn scrape_website(url: &str, query: &str) -> Result<usize, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .timeout(Duration::from_secs(10))
        .send()?
        .text()?;

    let document = Document::from(response.as_str());

    let mut count = 0;
    for node in document.find(Name("body")).next() {
        let text = node.text();
        count += text.matches(query).count();
    }

    Ok(count)
}

// Function to display an error if the URL is not valid
fn validate_url(url: &str) -> bool {
    reqwest::Url::parse(url).is_ok()
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_input() {
        let result = get_user_input("Enter a test input: ");
        assert!(result.is_ok(), "Failed to get user input.");
    }

    #[test]
    fn test_scrape_website() {
        let result = scrape_website("https://example.com", "example");
        assert!(result.is_ok(), "Failed to scrape website.");
    }

    #[test]
    fn test_validate_url() {
        assert!(validate_url("https://example.com"));
        assert!(!validate_url("invalid-url"));
    }
}

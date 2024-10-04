use std::sync::{Arc, Mutex};
use tokio::time::Duration;
use reqwest; // Ensure this is included in your Cargo.toml
use serde::Deserialize;

// Define a struct for the stock price
#[derive(Deserialize, Clone)] // Added Clone trait
struct StockPrice {
    price: f64,
}

// Struct for the Trading Bot
#[derive(Default)]
struct TradingBot {
    position: f64,      // Current position in the stock (positive for long, negative for short)
    balance: f64,       // Current balance
    total_profit: f64,  // Total profit/loss
    last_price: f64,    // Last stock price for comparison
}

// Implement trading logic
impl TradingBot {
    fn new() -> Self {
        Self::default()
    }

    fn evaluate_trade(&mut self, current_price: f64) {
        let price_change = (current_price - self.last_price) / self.last_price * 100.0;

        // Buy if price drops by 2%
        if price_change <= -2.0 && self.position <= 0.0 {
            self.position += 1.0; // Buy one unit
            self.balance -= current_price; // Deduct the price from balance
            println!("Bought at: {}", current_price);
        }
        // Sell if price rises by 3%
        else if price_change >= 3.0 && self.position >= 1.0 {
            self.position -= 1.0; // Sell one unit
            self.balance += current_price; // Add the price to balance
            println!("Sold at: {}", current_price);
        }

        // Update the last price and calculate total profit/loss
        self.last_price = current_price;
        self.total_profit = self.position * current_price + self.balance; // Calculate total profit/loss

        // Print summary of current state
        println!("Current Position: {}, Balance: {}, Total Profit/Loss: {}", 
            self.position, self.balance, self.total_profit);
    }
}

#[tokio::main]
async fn main() {
    // Initialize the state for storing stock prices
    let stock_prices = Arc::new(Mutex::new(Vec::new()));

    // Create a trading bot instance
    let mut trading_bot = TradingBot::new();

    // Start generating mock stock prices (from mock API)
    let stock_prices_clone = Arc::clone(&stock_prices);
    tokio::spawn(async move {
        loop {
            // Call the mock API to get current stock prices
            match reqwest::get("http://localhost:8080/api/stock_prices").await {
                Ok(response) => {
                    // Parse the JSON response
                    if let Ok(stock_prices_vec) = response.json::<Vec<StockPrice>>().await {
                        let mut prices = stock_prices_clone.lock().unwrap();
                        for stock_price in stock_prices_vec {
                            prices.push(stock_price.clone()); // Clone here to avoid move
                            println!("Fetched stock price: {}", stock_price.price); // Access original stock_price
                        }
                    } else {
                        eprintln!("Failed to parse stock price.");
                    }
                }
                Err(e) => eprintln!("Error fetching stock prices: {}", e),
            }

            // Wait before the next API call
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    // Start monitoring stock prices and making trades
    loop {
        // Get the latest stock prices
        let prices = stock_prices.lock().unwrap();
        if let Some(latest_price) = prices.last() {
            // Evaluate trades based on the latest price
            trading_bot.evaluate_trade(latest_price.price);
        }

        // Wait for a while before checking again
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

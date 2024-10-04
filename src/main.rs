use reqwest::Client;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio::time::{self, Duration};

#[derive(Debug, Clone)]
struct Stock {
    symbol: String,
    price: f64,
}

struct TradingBot {
    stocks: Vec<Stock>,
    positions: Vec<Stock>,
    balance: f64,
    profit_loss: f64,
}

impl TradingBot {
    fn new() -> Self {
        Self {
            stocks: vec![],
            positions: vec![],
            balance: 10000.0,
            profit_loss: 0.0,
        }
    }

    fn update_stocks(&mut self, fetched_stocks: Vec<Stock>) {
        self.stocks = fetched_stocks;
    }

    fn trade(&mut self) {
        for stock in &self.stocks {
            let current_price = stock.price;
            if let Some(position) = self.positions.iter_mut().find(|p| p.symbol == stock.symbol) {
                // Check for selling condition
                if current_price >= position.price * 1.03 {
                    self.balance += current_price; // Assuming selling 1 unit for simplicity
                    self.profit_loss += (current_price - position.price);
                    println!("Sold 1 share of {} at ${:.2}.", stock.symbol, current_price);
                    self.positions.retain(|p| p.symbol != stock.symbol); // Remove the position after selling
                }
            } else {
                // Check for buying condition
                if current_price <= stock.price * 0.98 {
                    let quantity_to_buy = 1; // Define quantity to buy
                    self.balance -= current_price;
                    self.positions.push(Stock {
                        symbol: stock.symbol.clone(),
                        price: current_price,
                    });
                    println!("Bought 1 share of {} at ${:.2}.", stock.symbol, current_price);
                }
            }
        }
    }

    fn summary(&self) {
        println!("Current Balance: ${:.2}", self.balance);
        println!("Total Profit/Loss: ${:.2}", self.profit_loss);
        println!("Open Positions: {:?}", self.positions);
    }
}

async fn start_trading_bot() {
    let client = Client::new();
    let bot = Arc::new(Mutex::new(TradingBot::new()));

    loop {
        // Fetch data from the mock API
        let response = client
            .get("http://127.0.0.1:8080/api/stock_prices")
            .send()
            .await
            .expect("Failed to fetch stock prices");

        let stocks: Vec<Value> = response.json().await.expect("Failed to parse JSON");

        let fetched_stocks: Vec<Stock> = stocks.into_iter().map(|s| {
            Stock {
                symbol: s["symbol"].as_str().unwrap().to_string(),
                price: s["price"].as_f64().unwrap(),
            }
        }).collect();

        let mut bot = bot.lock().unwrap();
        bot.update_stocks(fetched_stocks);
        bot.trade(); // Execute trading logic
        bot.summary(); // Print summary

        // Sleep for a specified duration (e.g., 5 seconds)
        time::sleep(Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    // Start the trading bot
    start_trading_bot().await;
}

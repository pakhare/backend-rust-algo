# Trading Backend
This is a simple trading bot application built with Rust that monitors stock price changes using a mock API. Implements basic trading strategies to buy or sell stocks based on price movements. 

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine. You can install them from [rustup.rs](https://rustup.rs/).

### Clone the Repository

```bash
git clone https://github.com/pakhare/backend-rust-algo.git
cd backend-rust-algo
```


### Start the mock API server:

```bash
cargo run --bin mock_api
```

This will start the mock API server at http://localhost:8080/api/stock_prices

### Build and Run the Trading Bot
In a new terminal window, navigate to the root of the project and run:

```bash
cargo run --bin backend_assignment
```
The trading bot will start monitoring stock prices and executing trades based on the defined strategy.
___

### API Usage
The mock API provides an endpoint to retrieve stock prices:

Endpoint: GET /api/stock_prices   
Response:
```json
[
    {
        "id": 1,
        "symbol": "AAPL",
        "price": 150.0,
        "quantity": 10
    },
    {
        "id": 2,
        "symbol": "GOOGL",
        "price": 2800.0,
        "quantity": 5
    }
]
```

### Trading Logic
The trading bot evaluates trades based on the following criteria:

- Buy: When the stock price drops by 2% compared to the last price, and there are no open positions.
- Sell: When the stock price rises by 3% compared to the last price, and there is an open position.
- Profit/Loss Tracking
    - Current Position: Positive for long positions, negative for short positions.
    - Current Balance: The cash balance available for trading.
    - Total Profit: The overall profit/loss based on current positions and balance.

### Technologies Used and Dependancies
- Rust
- Actix Web (for the mock API)
- Reqwest (for making HTTP requests)
- Serde (for JSON serialization/deserialization)
- Tokio (for asynchronous programming)



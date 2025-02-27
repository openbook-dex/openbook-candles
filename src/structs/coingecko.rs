use serde::Serialize;
use sqlx::types::Decimal;

use super::{markets::MarketInfo, openbook::token_factor};

#[derive(Debug, Clone, Serialize)]
pub struct CoinGeckoOrderBook {
    pub ticker_id: String,
    pub timestamp: String, //as milliseconds
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoinGeckoPair {
    pub ticker_id: String,
    pub base: String,
    pub target: String,
    pub pool_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoinGeckoTicker {
    pub ticker_id: String,
    pub base_currency: String,
    pub target_currency: String,
    pub last_price: String,
    pub base_volume: String,
    pub target_volume: String,
    // pub bid: String,
    // pub ask: String,
    pub high: String,
    pub low: String,
}

pub struct PgCoinGecko24HourVolume {
    pub address: String,
    pub raw_base_size: Decimal,
    pub raw_quote_size: Decimal,
}
impl PgCoinGecko24HourVolume {
    pub fn convert_to_readable(&self, markets: &Vec<MarketInfo>) -> CoinGecko24HourVolume {
        let market = markets.iter().find(|m| m.address == self.address).unwrap();
        let base_volume = self.raw_base_size / token_factor(market.base_decimals);
        let target_volume = self.raw_quote_size / token_factor(market.quote_decimals);
        CoinGecko24HourVolume {
            market_name: market.name.clone(),
            base_volume,
            target_volume,
        }
    }
}

#[derive(Debug, Default)]
pub struct CoinGecko24HourVolume {
    pub market_name: String,
    pub base_volume: Decimal,
    pub target_volume: Decimal,
}

#[derive(Debug, Default)]
pub struct PgCoinGecko24HighLow {
    pub market_name: String,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
}

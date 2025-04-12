
# Bitcoin Fair Value Rust Script

This Rust script connects to the Binance API to fetch the Bitcoin price for the last 24 hours and calculates a fair value based on moving averages.


## Usage

1. Clone the repository:
   ```sh
   git clone https://github.com/jamesbachini/Bitcoin-Fair-Value.git
   ```
2. Navigate to the project directory:
   ```sh
   cd Bitcoin-Fair-Value/
   ```
3. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

4. Build the project:
   ```sh
   cargo build
   ```
5. Run the script using:
   ```sh
   cargo run
   ```

The script will fetch the Bitcoin prices from Coinbase for the last 24 hours and calculate the fair value based on a simple moving average (SMA).

## Dependencies

This project uses the following dependencies:
- `reqwest` for making HTTP requests.
- `tokio` for asynchronous runtime.
- `serde` for JSON deserialization.

## Example Output

```
Current price of Bitcoin: $67488.78
24-hour moving average: $67131.63
7-day moving average: $69611.82
Fair value of Bitcoin: $68077.41
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- [Website](https://jamesbachini.com)
- [YouTube](https://www.youtube.com/c/JamesBachini?sub_confirmation=1)
- [Substack](https://bachini.substack.com)
- [Podcast](https://podcasters.spotify.com/pod/show/jamesbachini)
- [Spotify](https://open.spotify.com/show/2N0D9nvdxoe9rY3jxE4nOZ)
- [Twitter](https://twitter.com/james_bachini)
- [LinkedIn](https://www.linkedin.com/in/james-bachini/)
- [GitHub](https://github.com/jamesbachini)

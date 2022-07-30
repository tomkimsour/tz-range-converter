# tz-range-converter
Convert a time range from a timezone to local time

## Usage
Using the executable file :
```
./tz-range-converter <startTime> <endTime?> <timezone>
```

You can also clone the repository, compile it and execute it from source:
```
cargo run <startTime> <endTime?> <timezone>
```

## Tests
Theses tests are hardcoded meaning that they are being compared to the local UTC+9:30
```
cargo test
```
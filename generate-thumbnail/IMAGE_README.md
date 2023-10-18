# About

The dataset consists of [100 different image files](https://github.com/spolanyev/go-vs-js/blob/main/generate-thumbnail/image-statistics.txt).

For each image, I created thumbnails using three different programming languages - Go, JavaScript, and Rust with their respective image resizing libraries.

I repeated the process 10 times with an interval of 1 hour to ensure statistical significance, resulting in a total of [3,000 data points](https://github.com/spolanyev/go-vs-js/blob/main/generate-thumbnail/image-results.csv), and took the average.

This is done in a [Wildix](https://www.wildix.com/) R&D playground.


## Summary

### Go
Billing seconds 75.05

Billing MB 6400

### JavaScript
Billing seconds 95.62

Billing MB 6400

### Rust
Billing seconds 77.94

Billing MB 6400

## Technical Data
### Go
go1.21.1

provided.al2

x86_64

512 MB

https://pkg.go.dev/github.com/davidbyttow/govips/v2/vips

### JavaScript

v18.17.1

nodejs18.x

x86_64

512 MB

https://www.npmjs.com/package/sharp

### Rust
1.72.1

provided.al2

x86_64

512 MB

https://crates.io/crates/image

# About

A dataset consists of [100 different image files](https://github.com/spolanyev/go-vs-js/blob/main/generate-thumbnail/image-statistics.txt).

For each image, I created thumbnails using three different programming languages - JavaScript, Go, and Rust with their respective image resizing libraries.

I repeated the process 10 times with an interval of 1 hour to ensure statistical significance, resulting in a total of [3,000 observations](https://github.com/spolanyev/go-vs-js/blob/main/generate-thumbnail/results.csv), and took the average.

This is done in a [Wildix](https://www.wildix.com/) R&D playground.


## Summary

### js
Billing seconds 96.87

Billing MB 6400

### go
Billing seconds 119.33

Billing MB 6400

### rust
Billing seconds 81.57

Billing MB 6400

## Technical Data
### js

v18.17.1

nodejs18.x

x86_64

512 MB

https://www.npmjs.com/package/sharp

### go
go1.21.0

provided.al2

x86_64

512 MB

https://pkg.go.dev/github.com/nfnt/resize

### rust
1.72.1

provided.al2

x86_64

512 MB

https://crates.io/crates/image

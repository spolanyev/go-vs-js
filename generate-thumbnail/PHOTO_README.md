# About

The dataset consists of [18 different photos](https://github.com/spolanyev/go-vs-js/blob/main/generate-thumbnail/photo-statistics.txt).

For each image, I created thumbnails in Go and JavaScript using their libraries based on `libvips`.

I repeated the process 10 times with an interval of 1 hour to ensure statistical significance, resulting in a total of [360 data points](https://github.com/spolanyev/go-vs-js/blob/main/generate-thumbnail/photo-results.csv), and took the average (`50MP-1.jpg` is excluded from the summary because of the `VipsJpeg: Invalid SOS parameters for sequential JPEG` JavaScript problem).

This is done in a [Wildix](https://www.wildix.com/) R&D playground.


## Summary

### Go
Billing seconds 33.29

Billing MB 1088

### JavaScript
Billing seconds 39.82

Billing MB 1088

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

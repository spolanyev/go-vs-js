# About

A dataset consists of [100 different image files](https://github.com/spolanyev/go-vs-js/blob/main/generate-thumbnail/image-statistics.txt).

For each image, I implemented thumbnail generation using two different programming languages - Go and JavaScript with their respective image resizing libraries.

With an interval of 1 hour, I repeated this process 10 times for each image and programming language to ensure statistical significance, resulting in a total of 2,000 observations.

Several series were excluded from [the statistics](https://github.com/spolanyev/go-vs-js/blob/main/generate-thumbnail/results.csv) due to thumbnail generation errors.

This is done in a [Wildix](https://www.wildix.com/) R&D playground.


## Summary

### js
Billing seconds 116

Billing MB 6208

### go
Billing seconds 132.15

Billing MB 6208

## Technical Data
### js

v18.17.1

nodejs18.x

x86_64

256 MB

https://www.npmjs.com/package/sharp

### go
go1.21.0

provided.al2

x86_64

256 MB

https://pkg.go.dev/github.com/nfnt/resize

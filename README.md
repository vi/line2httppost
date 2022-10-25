# line2httppost

Simple tool to read lines from stdin and post each line as separate POST request to a specified URL (TCP connection is reused though).

Good together with [postsse](https://github.com/vi/postsse).

# Features

* Connects to http and https, both http1 and http2.
* Logging of errors during connection or failed response statuses.

# Limitations

* No advanced options like line delimiter
* It waits for reposnse from server before sending the next line, so RTT affects thoughput.
* Each line is fully buffered prior to starting the request, no streaming/chunking.

# Installation

Download pre-built executables from [Github releases](https://github.com/vi/line2httppost/releases/), install it from source code with `cargo install --path .` or from crates.io with `cargo install line2httppost`.

# Example

```
$ line2httppost http://example.com
123
ABC
```

```
POST / HTTP/1.1
content-type: text/plain
host: example.com
content-length: 3

123

POST / HTTP/1.1
content-type: text/plain
host: example.com
content-length: 3

ABC
```

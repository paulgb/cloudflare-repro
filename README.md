# Cloudflare workers-rs issue repro repo

## Reproducing

This repo reproduces an issue with Cloudflare workers.

To reproduce:

    npm i
    npm run dev

In Chrome, open up `about:blank` and open up the JavaScript console,
and run:

    new WebSocket("ws://127.0.0.1:8787/")

In the console, you will see:

    WebSocket connection to 'ws://127.0.0.1:8787/' failed: [snip]

And in the terminal with the running server, you will see:

    panicked at 'called `Result::unwrap()` on an `Err` value: JsValue(RangeError: Responses may only be constructed with status codes in the range 200 to 599, inclusive.
    RangeError: Responses may only be constructed with status codes in the range 200 to 599, inclusive.

## Verifying the regression

Now, stop the running server. In `Cargo.toml`, change `worker` to `0.0.13` and `worker-sys` to `=0.0.7`.

Start the server again:

    npm run dev

In the Chrome instance, run the same JavaScript again:

    new WebSocket("ws://127.0.0.1:8787/")

This time, you should not see an error.

## Commentary

The error message appears to come from
[this line](https://github.com/cloudflare/workerd/blob/ae612f0563d864c82adbfa4c2e5ed78b547aa0a1/src/workerd/api/http.c%2B%2B#L1091) of `workerd`. The
response object does indeed have a status code outside of the range (101). This check should be
bypassed because the response has a websocket connection, but it is not.

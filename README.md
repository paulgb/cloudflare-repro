# Cloudflare workers-rs issue repro repo

## Reproducing

This repo reproduces an issue with Cloudflare workers.

To reproduce:

    npm i
    npm run dev

While the server continues to run in another terminal:

    cd client
    npm i
    npx ts-node ./client.ts

Expected output:

    received: <Buffer 01 02 03>

Actual output:

    received: <Buffer 65 00 01>

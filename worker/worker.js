addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    try {
        const { wasm_main } = wasm_bindgen;

        // noinspection JSUnresolvedVariable
        await wasm_bindgen(wasm);

        const headers = {};
        for (let [key, value] of request.headers.entries()) {
            headers[key] = value;
        }

        // noinspection JSUnresolvedVariable
        const text = await request.text()
        console.log(text);
        const json = JSON.parse(text);
        console.log(json.type);
        const context = {
            request: {
                headers,
                body: text,
            },
            env: {
                PUBLIC_KEY,
                    UPSTASH_URI,
                UPSTASH_TOKEN
            },
            type: json.type
        };
        const { status, body } = await wasm_main(context);
        console.log(status, body);
        return new Response(body, {
            status,
            headers: {
                'Content-Type': 'application/json',
            },
        });
    } catch (e) {
        return new Response(e.toString(), {
            status: 500,
            headers: {
                'Content-Type': 'text/plain',
            },
        });
    }
}

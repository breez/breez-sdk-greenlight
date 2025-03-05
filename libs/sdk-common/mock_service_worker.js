let db = null;

async function getDb() {
    if (db) {
        return db;
    } else {
        return await new Promise((resolve, reject) => {
            const req = indexedDB.open("http_mock", 1);
            req.onsuccess = (event) => resolve(event.target.result);
            req.onerror = reject;
            req.onupgradeneeded = (event) => {
                db = event.target.result;
                if (!db.objectStoreNames.contains("mocks")) {
                    db.createObjectStore("mocks", { keyPath: "nonce" });
                }
            };
        })
    }
}

async function setMock(mock) {
    const db = await getDb();
    await new Promise((resolve, reject) => {
        const transaction = db.transaction("mocks", "readwrite");
        transaction.oncomplete = resolve;
        transaction.onerror = reject;
        const store = transaction.objectStore("mocks");
        store.put(mock);
    })
}

async function getMock(nonce) {
    const db = await getDb();
    return await new Promise((resolve, reject) => {
        const req = db.transaction("mocks")
            .objectStore("mocks")
            .get(nonce);
        req.onsuccess = (event) => resolve(event.target.result);
        req.onerror = reject;
    });
}

// Status codes are chosen to avoid being picked up as successes by tests expecting a 404 or 500.

self.addEventListener("fetch", (event) => {
    event.respondWith((async () => {
        try {
            const request = event.request;
            const url = new URL(request.url);
            if (url.host === "mock_configure") {
                const nonce = url.pathname.substring(1);
                const { method, path, status_code, body, content_type } = await request.json();
                const mock = await getMock(nonce) ?? { nonce, routes: [] };
                mock.routes.push({ method, path, status_code, body, content_type, hits: 0 });
                await setMock(mock);
                return new Response(null, { status: 204 });
            } else if (url.host === "mock_assert") {
                const nonce = url.pathname.substring(1);
                const mock = await getMock(nonce);
                if (mock === undefined) {
                    return new Response(`no such mock id ${nonce}`, { status: 421 });
                }
                const hitsMap = Object.fromEntries(mock.routes.map(route => [`${route.method} ${route.path}`, route.hits]));
                return new Response(JSON.stringify(hitsMap), { status: 200, headers: { 'Content-Type': 'application/json' } });
            } else {
                const nonce = url.host.split('_')[1];
                const mock = await getMock(nonce);
                if (mock === undefined) {
                    return new Response(`no such mock id ${nonce}`, { status: 421 });
                }
                for (const route of mock.routes) {
                    const partialPath = route.path.split('?')[0];
                    // Replace only -.!~*'()
                    const decodedPartialPath = route.path.replaceAll('%21', '!').replaceAll('%27', '\'').replaceAll('%28', '(').replaceAll('%29', ')').replaceAll('%2A', '*').replaceAll('%2D', '-').replaceAll('%2E', '.');
                    if (request.method === route.method && (url.pathname === route.path || url.pathname === partialPath || url.pathname === decodedPartialPath)) {
                        route.hits += 1;
                        await setMock(mock);
                        return new Response(Uint8Array.from(route.body), { status: route.status_code, headers: { 'Content-Type': route.content_type || 'application/json' }});
                    }
                }
                const possiblyMeant = mock.routes.find(route => route.path === url.pathname);
                if (possiblyMeant !== undefined) {
                    return new Response(`expected ${possiblyMeant.method}, got ${request.method}`, { status: 405 })
                } else {
                    return new Response(`expected ${mock.routes.map(route => route.path).join(' | ')}, got ${url.pathname}`, { status: 410 });
                }
            }
        } catch (e) {
            return new Response(e.toString(), { status: 503 });
        }
    })())
});

self.addEventListener("activate", (event) => {
    skipWaiting();
    event.waitUntil(clients.claim());
});
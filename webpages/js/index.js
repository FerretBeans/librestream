//Classes

// Simple way of getting datafiles bcs i couldnt think of any other way

const ip_address = document.URL.split("/");
const url = ip_address[0] + "//" + ip_address[2];

(async function () {
    const t = crypto.randomUUID();
    const e = new TextEncoder("utf-8").encode(t);
    const k = await crypto.subtle.digest("SHA-256", e);
    const h = new Uint8Array(k).toHex();

    if (!document.cookie) {
        document.cookie = "token=" + h + "; max-age=30"; // this is the web token for guest users
        await fetch(url + "/api/v1/token", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ "token": h }),
        });
    }
})();

function loggedin() {
    
}
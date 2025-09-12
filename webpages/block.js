if (document.URL.endsWith(".html")) {
    location.href = "/disallowed"
    setTimeout(() => {
        location.href = "/"
    }, 5000);
}
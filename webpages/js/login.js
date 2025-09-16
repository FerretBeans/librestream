// Simple way of getting datafiles bcs i couldnt think of any other way

const ip_address = document.URL.split("/");
const url = ip_address[0] + "//" + ip_address[2];


function sessiontokengen() {
    // Temp unless i change my mind
    const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-_";
    const charsetlength = charset.length;

    let result = '';

    for (let i = 0; i < 40; i++) {
        result += charset.charAt(Math.floor(Math.random() * charsetlength));
    }

    fetch(url + "/")
}

sessiontokengen();
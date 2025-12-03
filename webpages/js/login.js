// Classes :3

// Simple way of getting datafiles bcs i couldnt think of any other way

const ip_address = document.URL.split("/");
const url = ip_address[0] + "//" + ip_address[2];

function createuser() {
    // TODO : encyrpt upon sending
    // TODO : create a user token
    let usernamedata = document.getElementById("username");
    let passwordata = document.getElementById("password");
    
    let username = usernamedata.value;
    let password = passwordata.value;
    let token = ""

    if (!username || !password) {
        alert("missing username or password");
    } else {
        fetch(url + "/api/v1/usercreate", {
            method: "POST",
            headers: {
                "content-type": "application/json"
            },
            body: JSON.stringify({ "un": username, "pw": password, "token": token}),
        });
    }
}
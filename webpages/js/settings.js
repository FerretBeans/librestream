// Simple way of getting datafiles bcs i couldnt think of any other way

const ip_address = document.URL.split("/");
const url = ip_address[0] + "//" + ip_address[2];

var date = new Date();
let log = date.getFullYear() + "-" + (date.getMonth()+1).toString().padStart(2, 0) + "-" + date.getDate() + ".log";

fetch(url + "/logs/" + log)
    .then(response => response.text())
    .then(text => {
        document.getElementById("logs").innerHTML = text;
    });

const woah = document.getElementById("adminpassword");

woah.addEventListener("keypress", (event) => {
    if(event.key === "Enter") {
        fetch();
    }
});
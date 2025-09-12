fetch("./ip.json")
    .then((response) => response.json())
    .then((json) => { 
        fetch("../datafiles/settings.json")
            .then((response) => response.json())
            .then((json) => {
                const ip = json.ip;
                const port = json.port;

                const woah = document.getElementById("adminpassword");

                console.log(ip + ":" + port);

                woah.addEventListener("keypress", (event) => {
                    if(event.key === "Enter") {
                        fetch(ip + ":3000")
                    }
                });
            })
        })
        
                
                
                
                
                
                
                
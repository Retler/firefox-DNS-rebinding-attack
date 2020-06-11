# Description
This is an example of how one can circumnvent the same origin policy by controlling an authoritative DNS server for some site.

The effect of the attack is a scan of the victims internal network and logging of the responses at the attacker's server.

# Setup
The setup is a dockerized lab environment with a victim browser, the attacker site and DNS server and an internal service. For specifics check docker-compose.yml. The services are separated on two different networks to simulate victims internal network and the internet.

## Requirements

- A rust compiler + cargo
- docker

## How to run

1. cd into the dns folder and compile the code
2. In the root directory run `docker-compose up`
3. Go to your fav. web browser and navigate to localhost:5800
4. Visit malicious site at "http://kitties.com"

After visiting the malicious site, the advertisement on the site starts querying the DNS server to resolve "http://kitties.com". At some point, the DNS responses will contain the victims network internal IP'addresses, effectively initating an internal service scan and sending results back to the malicious server.

## Network setup

* The target-service is reachable on the 'internal' network at port 80
* For a more realistic setup, the ad-server and the dns-server are hosted on the 'external' network
* The victim browser is hosted internally as well

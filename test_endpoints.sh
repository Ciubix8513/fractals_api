#!/bin/bash

# Prompt the user for IP address
read -p "Enter the IP address: " ip

# Prompt the user for port number
read -p "Enter the port number: " port

read -p "Enter args: " args

# Perform the curl requests
echo "Performing curl requests on $ip:$port"
curl "http://$ip:$port/fractals/Mandelbrot?$args" --output mandelbrot.png
curl "http://$ip:$port/fractals/BurningShip?$args" --output burning_ship.png
curl "http://$ip:$port/fractals/Tricorn?$args" --output tricorn.png
curl "http://$ip:$port/fractals/Feather?$args" --output feather.png
curl "http://$ip:$port/fractals/Eye?$args" --output eye.png


# world_time is a cli program, which shows the current date, day of the week, and time in various cities around the world. 
# Features, which will be added to this program: 

# By default using ./world_time without options it will show current time in Sydney, Moscow, Vienna, and New York
# As an option world_time will take one of the supported cities, e.g. world_time London
# Planned support for the following cities: Sydney, Adelaide, Perth, Darwin, Tokyo, Singapore, Dubai, Doha, Moscow, Vienna, Budapest, Berlin, London, Paris, Madrid, Rome, Lissabon, New York, Chicago, Los Angeles
Programming an NTP request:
To make an NTP request programmatically, one would typically:
Create a UDP socket.
Construct an NTP request packet according to the NTP protocol specification.
Send the packet to the NTP server's IP address and port 123.
Receive the response packet.
Parse the response to extract the transmit timestamp, which represents the server's current time.
Convert the NTP time format to a standard time format.

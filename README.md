# basic-plotter
Just like the name suggests this is a basic plotter. It's designed to be more or less transparent. Any person with a basic idea of sockets and json can use this instead of having to learn to use a plotting library (not to mention the trouble you have to go through to install one in some languages).

This tool runs a socket server at `127.0.0.1:3333` [config files wll come later]. and you can connect to it using socket libraries in any language.
At a time you can send only information about one point. granted it is slow and really inefficient, but I think this saves time , if all you want to do is to  plot to get a really basic idea of what your data looks like.

### Sending data
Data is sent using json. One point at a time.
The schema is as follows
```json
 {
       "name":"YOURTHINGSNAME",
       "color":[u8COLOR_R,u8COLOR_G,u8COLOR_B],
       "points":[YOURPOINT_X,YOURPOINT_Y]
 }
```
`name` specifies the name of the plot you are plotting at the moment, so that the plotter can determine which plot it's plotting.

`color` refers to the color of the plot. If the first point in a plot named 'NAME' had a color `[R,G,B]` that color will be the color of all subsequent points with name 'NAME'. Even of you change the color parameter in the json file you send.

### Example
basic python example.

```python
import json
import socket
import time
# Whatever your data is
# for this example (18.0,18.0)
# The json schema for sending data is as follows
#
# {
#       "name":"YOURTHINGSNAME",
#       "color":[u8COLOR_R,u8COLOR_G,u8COLOR_B],
#       "points":[YOURPOINT_X,YOUTPOINT_Y]
# }
client  = socket.socket(socket.AF_INET,socket.SOCK_STREAM)
ip = socket.gethostbyname("127.0.0.1")
port = 3333             # replace with your port
address = (ip,port)
client.connect(address)

for m in range(-20,20):
    data = {'name':'cora','color':[0,2,200],'points':[m,m]}
    jdata = json.dumps(data)
    jdata = jdata+'\n'
    print(jdata)
    client.send(bytes(jdata,encoding='utf-8'))
    time.sleep(1)
client.close()
#thats it1

```

There are a few more things to add to make this usable. I intend to keep things as simple as possible.

### Note for linux/macOS users:
This was mostly built for people who use windows since package managers make installing libraries easier in linux. So you will have to go through the teeny weeny process of installing SDL2.

Ubuntu example:
> sudo apt-get install libsdl2-dev

Fedora example:
> sudo dnf install SDL2-devel

Arch example:
(Arch doesn't have separate regular and development packages, everything goes together.)
> sudo pacman -S sdl2
You might also need a C compiler (gcc).

to know more, go to
https://github.com/Rust-SDL2/rust-sdl2

I shall statically link it in another release.

## usage examples
The keybindings are as follows
```
W A S D for movement
Z X for zoom in/out
```
For a really basic plotter , these are all the controls you need. If you can think of something to add , that does not affect this tool's simplicity/transparency, submit a PR.

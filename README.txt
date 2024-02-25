oldmcskins
==========
An application to display Minecraft skins on old versions (<1.7.10)

Installation (on server)
========================
  $ git clone https://github.com/crptmem/oldmcskins.git && cd oldmcskins
  $ cargo build

Usage
=====
Server should listen on port 80.
On client, edit /etc/hosts on Linux and c:\windows\system32\drivers\etc\hosts
on Windows. Add following entry:
```
YOUR_SERVER_IP skins.minecraft.net
```

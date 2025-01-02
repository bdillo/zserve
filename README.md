# Zserve

Zserve is a really simple tool for hosting single files over a local network, discoverable via mDNS. I just wanted
something really, really simple for situations where I provision a VM or something locally and want to curl my
`authorized_keys` in an easy way. This setup is a bit simpler to use (assuming your client host can resolve mDNS),
you just run `zserve -f path/to/file` and the file is available at `https://zserve.local:8443` (hostname can be
changed with `-n` if needed). The file is served over a HTTPS with a self signed certificate (for ease of use), so it
can be accessed with curl like: `curl -k https://zserve.local:8443`. Be sure to verify the file's SHA256 hash (zserve
logs this upon start) on the client to be sure nothing funny is happening since we are using a self signed cert.

# jack-web-proxy

It's nothing serious like [NetJack](https://github.com/jackaudio/jackaudio.github.com/wiki/WalkThrough_User_NetJack2), but instead, it exposes a few port configuration APIs via http, so that some patchbay app like [Catia](https://github.com/falkTX/Catia) can be implemented on web.

And, that's just an execuse for starting to explore Jack ecosystem and Rust ecosystem.

```sh
# run server
cargo run

# GET /summary
curl localhost:3000/summary | jq
# {
#   "ports": [
#     {
#       "id": "system:capture_1",
#       "flags": [
#         "IS_OUTPUT",
#         "IS_PHYSICAL",
#         "IS_TERMINAL"
#       ]
#     },
#     ...
#   ],
#   "connections": [
#     {
#       "source": "system:capture_1",
#       "destinations": [
#         "PulseAudio JACK Source:front-left"
#       ]
#     },
#     ...
#   ]
# }

# GET /sse
curl localhost:3000/sse
# data:changed

# POST /api/connect (cf. https://jackaudio.org/api/group__PortFunctions.html)
curl localhost:3000/sse -d '{ source: "xxx", destination: "yyy" }'
```

## TODO

- [x] jack api
- [ ] web server
  - [x] summary endpoint
  - [x] event stream endpoint (SSE)
  - [ ] auto generate endpoint from jack api
- [ ] web client
- [ ] binary release

## references

- https://jackaudio.org/api
- https://github.com/RustAudio/rust-jack
- https://github.com/tokio-rs/axum

kinglet
=======
A modern asynchronous HTTP server for Rust.

Overview
--------
Kinglet is based on [rotor](https://github.com/tailhook/rotor) and
[mio](https://github.com/carllerche/mio) and is a proof-of-concept web server
that uses state machines to manage connections.

![digraph Client {
    // Simplified graph. Does not contain error cases.
    "Initial" -> "ReadHeaders" -> "Parsed" -> "KeepAlive" -> "ReadHeaders"
    "ReadHeaders" -> "ReadFixedSize" -> "ReadFixedSize" ->"Parsed"
    "ReadHeaders" -> "ReadChunked" -> "ReadChunked" -> "Parsed"
    "ReadChunked" -> "ReadTrailers" -> "Parsed"
    {rank = same; "Initial"; "KeepAlive";}
    {rank = same; "ReadFixedSize"; "ReadChunked";}
}
](https://cdn.rawgit.com/pyfisch/kinglet/a84733321ef6cc9e49a16422b04732dda684d0f4/connection.svg)

The software is still very incomplete and not yet usable.

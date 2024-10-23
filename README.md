# RustAPIHTTPSTutorial
Rust API tutorial where we also use HTTPS to protect against MITM attacks

## Reference
Reference : https://github.com/tokio-rs/axum/tree/main/examples/tls-rustls
Credits to Jonas Platte for much of the code

## Important notes
1- I generated my self-signed certificates using: 
```bash
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout key.pem -out cert.pem
```
On Windows (only recommended for development).

2- Self-signed certificates are good for development, but not for production. IMPORTANT: You will get warnings in the browser if you keep self-signed certificates. You get warnings, because it isn't guaranteed that you are protecting your certificates correctly (there could be a leak of information). The trusted providers (CAs) adhere to higher levels of security to keep your encryption keys from being stolen.

2.1- Use SSL certificates from trusted providers for projects going to production. Options:
    
- The 'Let's Encrypt' project offers SSL certificates for free. However, they expire after 90 days. Use their Certbot tool to automatically renew the keys.
- Digicert
- GlobalSign
- Namecheap
- Entrust

3- If you have errors with make (and have downloaded the tool), and it is of type:
```
failed to run custom build command for `aws-lc-sys v...`
```
I suggest looking here:
https://medium.com/@rrnazario/rust-how-to-fix-failed-to-run-custom-build-command-for-aws-lc-sys-on-windows-c3bd2405ac6f

Essentially, look at what stands below '---stderr' and it should give you indications on what to do. I have installed CMake and Visual Studio 2022, and have included 'Desktop development with C++' with my installation of Visual Studio. Now it works.

4- Assuming that you might want to update the dependencies of the project (like I did), go to crates.io to figure out which version of your dependency matches with which version of some other dependency. As an example, here is axum-server: https://crates.io/crates/axum-server/0.7.1/dependencies . If you look for axum, you see it relates to axum version 0.7. I had strange errors because of conflicting imports.
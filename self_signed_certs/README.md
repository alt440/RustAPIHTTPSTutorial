I generated my self-signed certificates using: 
```bash
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout key.pem -out cert.pem
```
On Windows (only recommended for development).

Generate them inside this folder. These keys will then be used from the main.rs file.
# Cryptography API
### Block Ciphers
AES_encrypt(String: message, [u32; 8] key) - Encrypts a string of hexadecimals using AES-256. \
AES_decrypt(String: message, [u32; 8] key) - Decrypts an AES-256 encrypted string into hexidecimal. 

### Padding
PKCS7_padding(text: String) - Pads a string using PKCS7. \
PKCS7_unpadding(text: String) - Unpads a PKCS7 padded string. 

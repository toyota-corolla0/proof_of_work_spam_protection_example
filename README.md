Simple proof of work spam protection example. This repo is for educational purposes. 

```bash
# cargo run --bin server create_challenge
enter number of leading zeros required in sha256 hash
24
challenge: 00000018f173c8c1c07f9d29979c632ee77db256e367d5393ba2f214137a3d9c12deb0dc

# cargo run --bin client solve
enter the challenge string
00000018f173c8c1c07f9d29979c632ee77db256e367d5393ba2f214137a3d9c12deb0dc
enter a message
my super secret password
Doing proof of work... need to find 24 leading zeros in sha256 hash
Good sha256 hash: 0000008e71e29fb8cb1fcdbd5d6e6901ce1cdabfd779bfb3688243a2d904f6e9
Found solution at nonce 3062806
Challenge response string: 00000000002ebc166d79207375706572207365637265742070617373776f7264

# cargo run --bin server verify
enter the challenge string
00000018f173c8c1c07f9d29979c632ee77db256e367d5393ba2f214137a3d9c12deb0dc
enter the challenge response string
00000000002ebc166d79207375706572207365637265742070617373776f7264
Good sha256 hash: 0000008e71e29fb8cb1fcdbd5d6e6901ce1cdabfd779bfb3688243a2d904f6e9
Verification was a success! Message: my super secret password

# cargo run --bin server verify
enter the challenge string
00000018f173c8c1c07f9d29979c632ee77db256e367d5393ba2f214137a3d9c12deb0dc
enter the challenge response string
00100000002ebc166d79207375706572207365637265742070617373776f7264
Verification failed!!!
```
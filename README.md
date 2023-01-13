# To reproduce the issue:

```shell
docker build -t testing-ssh:1 .
```
```shell
docker run -d -p 12345:22 -v <your local path to this repo>:/home/ubuntu/project testing-ssh:1
```

This is optional but makes IJ automatically pick up rust if we install it for this user, sorry i didn't automate it:
```shell
ssh -p 12345 test@localhost #password is test
```

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```shell
source "$HOME/.cargo/env"
```

- Run Gateway 223.8214.51
- Open project using host: localhost, username: test, port: 12345, password: test
- Open IJ (i use 223.8617.9), use `/home/ubuntu/project` as project directory
- Install Rust plugin on host (i use version 0.4.185.5086-223)
- Wait for project import and indexing
- Open `main.rs` and run

# Hotel Chain Backend

## Setup 

```bash
rustup default stable
```

---
**Install necessary dependencies for Rocket** 

1. Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate):
```bash
cargo install cargo-generate
```
2. Clone the Rocket application:
```bash
cargo generate --git https://github.com/k0pernicus/cargo-template-rocket-base --name yourprojectname
```
3. Override the default toolchain in `yourprojectname`:
```bash
cd yourprojectname && rustup override set nightly
```
4. Run the app:
```bash
cargo run
```


## Configuration
### Environment Variables:
- DATABASE_URL
  - example value: DATABASE_URL=postgres://monkey_user:monkey_pass@localhost/monkey_db
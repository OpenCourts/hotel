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
cargo generate --git https://github.com/k0pernicus/cargo-_template-rocket-base --name yourprojectname
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
- DATABASE_USER
- DATABASE_PASS
- DATABASE_HOST
- DATABASE_NAME
- SMTP_USERNAME
- SMTP_PASSWORD
- SMTP_SERVER
- SMTP_PORT (optional)
  - example value: DATABASE_USER=monkey_user;DATABASE_PASS=monkey_pass;DATABASE_HOST=localhost:5432;DATABASE_NAME=monkey_db;SMTP_USERNAME=example@mydomain.com;SMTP_PASSWORD=my_secret_password;SMTP_SERVER=smtp.mydomain.com;SMTP_PORT=587
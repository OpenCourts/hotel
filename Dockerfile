FROM rust:1.69

RUN apt install curl
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash -
RUN apt update
RUN apt install nodejs

RUN npm install -g @vue/cli

COPY . /distributed/

WORKDIR /distributed/frontend/app
RUN npm install -f
RUN npm run build

RUN mv /distributed/frontend/app/dist/ /distributed/backend/static

WORKDIR /distributed/backend
RUN cargo build --release --bin hotel

EXPOSE 8000
CMD ["/distributed/backend/target/release/hotel"]
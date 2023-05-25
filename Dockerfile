FROM node:15.5.1 as frontend
RUN npm install -g @vue/cli@5.0.8

COPY ./frontend/app /app

WORKDIR /app
RUN npm install -f
RUN npm run build

FROM rust:1.69 as backend
COPY ./backend /distributed

COPY --from=frontend /app/dist/ /distributed/static

WORKDIR /distributed
RUN cargo build --release --bin hotel

EXPOSE 8000
CMD ["/distributed/backend/target/release/hotel"]

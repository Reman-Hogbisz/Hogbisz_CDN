# Build frontend
FROM stappsworldreport/yew_builder:amd64 as frontend-builder

WORKDIR /app

ADD ./src ./src
ADD ./Cargo.toml .
ADD ./Cargo.lock .
ADD ./index.html .
ADD ./Trunk.toml .
ADD ./tailwind.config.js .

RUN trunk --config ./Trunk.toml build --release

FROM nginx:stable-alpine as frontend

COPY ./nginx/nginx.conf /etc/nginx/conf.d/default.conf

RUN mkdir /app

WORKDIR /app

COPY --from=frontend-builder /app/dist ./dist

RUN rm -rf /usr/share/nginx/html

RUN cp -r ./dist /usr/share/nginx/html

COPY ./entrypoint.sh ./entrypoint.sh

CMD ["/app/entrypoint.sh"]
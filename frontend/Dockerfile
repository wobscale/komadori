# node:10 @ 2018-10-02
FROM node@sha256:32d5d5435e5038028bd0c0871d256d5887b06c4315cb1a135bcf81e0735627c9 as build
RUN mkdir /usr/src/app
WORKDIR /usr/src/app
COPY . /usr/src/app
RUN npm install --silent
RUN npm run build-css && npm run build

# nginx:latest @ 2018-10-02
FROM nginx@sha256:11068617e2c38175c444c9e4b52294483dfebd4b8015a3ed815528687b2ab1ec

COPY --from=build /usr/src/app/build /usr/share/nginx/html

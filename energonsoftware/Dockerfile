# stage 1: build
FROM node:alpine AS builder

# build args
ARG configuration=prod

WORKDIR /app

# setup npm
COPY energonsoftware/package.json energonsoftware/package-lock.json ./
RUN npm install

# copy source
COPY energonsoftware/ ./

# build
RUN npm run build --configuration=$configuration

# stage 2: create image
FROM nginx:alpine

# install dependencies
RUN apk update

RUN rm -rf /usr/share/nginx/html/*

COPY energonsoftware/nginx.conf /etc/nginx/nginx.conf

COPY --from=builder /app/dist/energonsoftware /usr/share/nginx/html

EXPOSE 8080

ENTRYPOINT ["nginx", "-g", "daemon off;"]

# stage 1: build
FROM node:alpine AS builder

# build args
ARG configuration=prod

# update image
RUN apk update && apk upgrade

WORKDIR /app

# setup npm
COPY energonsoftware/package.json energonsoftware/package-lock.json ./
RUN npm install

# copy source
COPY energonsoftware/ ./

# build
RUN npm run build --configuration=$env

# stage 2: create image
FROM nginx:alpine

# update image
RUN apk update && apk upgrade

RUN rm -rf /usr/share/nginx/html/*

COPY energonsoftware/nginx.conf /etc/nginx/nginx.conf

COPY --from=builder /app/dist/energonsoftware /usr/share/nginx/html

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]

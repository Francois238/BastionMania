#stage 1
FROM node:lts as build
WORKDIR /app
COPY . .
RUN npm install && npm install -g @angular/cli

RUN ng build --build-optimizer --base-href=/
#stage 2
FROM nginx:stable-alpine

COPY nginx.conf /etc/nginx/nginx.conf

# copy artifact build from the 'build environment'
COPY --from=build /app/dist/front/ /usr/share/nginx/html

# Expose port 80
EXPOSE 80
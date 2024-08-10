# build stage
FROM node:lts-alpine

WORKDIR /var/www/html/app

ENV PATH /var/www/html/app/node_modules/.bin:$PATH

COPY package.json /var/www/html/app/package.json

RUN npm install

CMD ["npm", "run", "dev"]
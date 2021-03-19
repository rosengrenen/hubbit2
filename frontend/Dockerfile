FROM node:lts-alpine

WORKDIR /usr/src/app

ENV NODE_ENV=production

COPY . .

RUN yarn --frozen-lockfile --network-timeout 1000000000 --production=false
RUN yarn build
# Remove dev dependencies
RUN rm -rf node_modules
RUN yarn --frozen-lockfile --network-timeout 1000000000

CMD yarn start

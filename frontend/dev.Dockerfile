FROM node:lts

WORKDIR /usr/src/app

ENV NODE_ENV=development

CMD yarn && yarn dev

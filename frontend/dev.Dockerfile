FROM node:lts

WORKDIR /app

ENV NODE_ENV=development

CMD yarn && yarn dev

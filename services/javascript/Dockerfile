FROM node:22-alpine

WORKDIR /app
COPY package.json index.js ./
RUN npm install

CMD ["npm", "run", "start"]


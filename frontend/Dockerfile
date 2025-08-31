FROM node:16.18

RUN mkdir -p /cpp-frontend
WORKDIR /cpp-frontend

COPY . .
RUN yarn install && yarn generate
CMD ["yarn", "dev"] 

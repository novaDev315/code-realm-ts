FROM node:20-alpine
WORKDIR /app

# #Point npm at registry + increase timeouts
# RUN npm config set registry https://registry.npmjs.org/ && \
#     npm config set fetch-retry-maxtimeout 120000 && \
#     npm config set fetch-retries 5

# Install multi-language support: Python, Go, Rust, Java, C++
RUN apk add --no-cache \
    python3 \
    py3-pip \
    go \
    rust \
    cargo \
    openjdk17 \
    g++ \
    make

# Install global TS tools
RUN npm install -g ts-node typescript

# Copy package files first
COPY package*.json ./
RUN npm install

# Copy source code
COPY . .

# Run the app
CMD ["npm", "run", "dev"] 
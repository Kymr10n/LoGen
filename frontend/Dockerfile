FROM node:18

WORKDIR /app

# Copy package manifest first for fast npm install caching
COPY package.json package-lock.json* ./

RUN npm install --legacy-peer-deps

# Copy frontend source
COPY . .

EXPOSE 5173

# Run Vite dev server on all interfaces so host can reach it
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]

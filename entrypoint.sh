#!/bin/bash

# Start the backend in the background
/app/nebula-backend &

# Start the frontend (Next.js)
cd /app/frontend
npm start

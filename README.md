# Supabase Docker

This is a minimal Docker Compose setup for self-hosting Supabase. Follow the steps [here](https://supabase.com/docs/guides/hosting/docker) to get started.


make sure to copy your .env.example into a file denoted .env

To run this application, use the following command

 docker compose -f docker-compose.yml -f ./dev/docker-compose.dev.yml up
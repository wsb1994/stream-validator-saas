# Supabase Docker

A minimal Docker Compose setup for self-hosting Supabase.  
Official docs: [Self-hosting with Docker](https://supabase.com/docs/guides/hosting/docker).

---

## Run Locally

1. **Copy environment file**  
   ```bash
   cp .env.example .env
2. ***run the process***
 ```bash
 docker compose -f docker-compose.yml -f ./dev/docker-compose.dev.yml up -d
 ```
3.observe the production service via docker logs, submit any improvements via PR, or just enjoy the demo.

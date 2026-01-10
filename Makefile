.PHONY: dev up down build-backend build-frontend

dev: up

up:
	docker compose up --build

down:
	docker compose down

build-backend:
	docker build -f backend/Dockerfile -t logen-backend .

build-frontend:
	docker build -f frontend/Dockerfile -t logen-frontend ./frontend

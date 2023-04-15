default:
  @just --list

dev-up:
  docker-compose -f dev.docker-compose.yml up -d

dev-down:
  docker-compose -f dev.docker-compose.yml down

pub fn generate_docker_file() -> &'static str {
    r#"volumes:
  postgres_data:
    driver: local

services:
  postgresql:
    image: postgres:16
    shm_size: 1g
    restart: always
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - 5440:5432
    env_file:
      - ./.env
 "#
}

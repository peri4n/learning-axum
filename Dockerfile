FROM postgres:latest

ENV POSTGRES_USER=test
ENV POSTGRES_PASSWORD=password
ENV POSTGRES_DB=mydatabase

EXPOSE 5432

# COPY init.sql /docker-entrypoint-initdb.d/

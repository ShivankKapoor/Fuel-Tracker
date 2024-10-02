# Fuel Tracker Backend

This repository contains the backend code for the Fuel Tracker application.

## Building the Docker Image

To build the Docker image for the Fuel Tracker backend, use the following command:

```bash
sudo podman build -t fuel_tracker_backend .
```

This command builds a Docker image named `fuel_tracker_backend` based on the Dockerfile in the current directory.

## Running the Container

To run the container, use the following command:

```bash
sudo podman run -p 127.0.0.1:8081:8081 -e DATABASE_URL="postgres://postgres:<PASSWORD>@<DB_HOST>/<DB_NAME>" fuel_tracker_backend
```

Replace the following placeholders with your actual information:

- `<PASSWORD>`: Your PostgreSQL password
- `<DB_HOST>`: The IP address or hostname of your PostgreSQL server
- `<DB_NAME>`: The name of your database

This command does the following:

- Maps port `8081` on the container to port `8081` on the host machine
- Sets the `DATABASE_URL` environment variable for the container
- Runs the `fuel_tracker_backend` image

## Running Without Podman/Docker

If you're not using Podman/Docker, create a `.env` file in your project directory with the following format:

```
DATABASE_URL=postgres://postgres:hootie@10.0.0.6/GasTracker
```
# EphemeralGuard

Fast, secure system for storing encrypted secrets, whether temporary or persistent. It ensures sensitive data is protected, with options for automatic erasure or long-term storage, ideal for secure exchanges and token management.

Sure! Below is a section for your README file that explains how to build and run the Docker image for your project.


## Building and Running the Docker Image

This section guides you through the process of building and running the Docker image for the project.

### Building the Docker Image

1. **Navigate to the Project Directory**:
   Make sure you're in the root directory of the project where the `Dockerfile` is located.

2. **Build the Docker Image**:
Use the following command to build the Docker image. Replace `EphemeralGuard` with a name of your choice.

```bash
docker build -t EphemeralGuard .
```

- The `-t` flag allows you to tag your image with a name. The `.` at the end specifies the current directory as the build context.

- This command will create a Docker image based on the instructions in the `Dockerfile`. The image will include your compiled binary and all necessary dependencies.

### Running the Docker Container

Run the Docker Container: Once the image is built, you can run it using the following command. Replace EphemeralGuard with a name for your container.

```bash
docker run -p 1337:1337 --name EphemeralGuard EphemeralGuard
```

- The `-p 1337:1337` option maps port `1337` on your local machine to port `1337` inside the container. Adjust the port numbers as needed.

- The --name EphemeralGuard option gives your container a specific name, making it easier to manage.

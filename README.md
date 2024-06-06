# Microcred Project

## Overview

This project aims to build a secure and scalable microcredential system using Rust for the backend, MongoDB for data storage, and React Native for the mobile app. The project leverages the Linera SDK for microchain management and transaction handling.

## Features

- **Backend**: Built with Rust and Actix Web for high performance and safety.
- **Database**: Utilizes MongoDB for flexible and scalable data storage.
- **API Design**: RESTful API documented with OpenAPI (Swagger).
- **Linera SDK Integration**: Manages microchains and handles transactions.
- **Mobile App**: Built with React Native for cross-platform support.
- **Security**: Implements encryption, access control, and input validation.

## Getting Started

### Prerequisites

- **Rust**: Install from [rust-lang.org](https://www.rust-lang.org/).
- **MongoDB**: Follow the instructions at [mongodb.com](https://www.mongodb.com/).
- **Protobuf Compiler**: Install `protoc` from [protocolbuffers/protobuf](https://github.com/protocolbuffers/protobuf/releases).
- **Node.js**: Install from [nodejs.org](https://nodejs.org/).
- **React Native CLI**: Install using `npm install -g react-native-cli`.

### Installation

1. **Clone the Repository**

    ```sh
    git clone https://github.com/yourusername/microcred-project.git
    cd microcred-project
    ```

2. **Backend Setup**

    - Navigate to the backend directory:
      ```sh
      cd microcred-backend
      ```
    - Install Rust dependencies:
      ```sh
      cargo build
      ```
    - Start the MongoDB server:
      ```sh
      mongod --dbpath /path/to/your/db
      ```
    - Run the backend server:
      ```sh
      cargo run
      ```

3. **Frontend Setup**

    - Navigate to the frontend directory:
      ```sh
      cd microcred-frontend
      ```
    - Install Node.js dependencies:
      ```sh
      npm install
      ```
    - Start the React Native app:
      ```sh
      react-native run-android
      # or
      react-native run-ios
      ```

### Usage

- **API Endpoints**: Access the API at `http://localhost:8080/`. Refer to the OpenAPI documentation at `http://localhost:8080/swagger`.

#### API Endpoints

- **Create DID Credential**:
  ```sh
  POST /dids

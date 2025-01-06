# `piwallet`

Welcome to your new `piwallet` project and to the Internet Computer development community. This project includes a README and some template files to help you get started. Feel free to customize these files to suit your project's needs and accelerate your development process.

## Project Structure

- **Backend**: The backend canister is written in Rust and is located in the `src/piwallet_backend` directory.
- **Frontend**: The frontend is built using Vite and is located in the `src/piwallet_frontend` directory.

## Prerequisites

- **Node.js**: Ensure you have Node.js version 16 or higher installed.
- **DFX**: Install the DFINITY SDK to interact with the Internet Computer. For setup instructions, [click here](https://internetcomputer.org/docs/current/developer-docs/backend/rust/dev-env).

## Setup Instructions

1. **Clone the Repository**:

    ```bash
    git clone <repository-url>
    cd piwallet
    ```

2. **Start the DFINITY Server**:

    ```bash
    dfx start --clean --background
    ```

    This command starts the server locally in the background.

3. **Build the dApp**: (Optional) _The deploy command will automatically build the project as a prerequisite._

    ```bash
    dfx build
    ```

4. **Deploy the Canister**:
   Run the following command to set up and deploy your canisters:

    ```bash
    dfx deploy
    ```

    **Note:** After executing the deploy command, you will receive URLs for the frontend and backend. Navigate to these URLs to access the dApp.

![image](https://private-user-images.githubusercontent.com/90865757/399544512-dce14681-817f-4476-aaab-0d9bef883b12.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MzYxNDQ2NDksIm5iZiI6MTczNjE0NDM0OSwicGF0aCI6Ii85MDg2NTc1Ny8zOTk1NDQ1MTItZGNlMTQ2ODEtODE3Zi00NDc2LWFhYWItMGQ5YmVmODgzYjEyLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTAxMDYlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwMTA2VDA2MTkwOVomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTFmZTU1ZjUwZTE1MjA1MDE1NTc4OGQwYTU0Njc5NjJiZmQ3M2MxYjk4NTJhNWM4ZWY1YmNlNjIxOGUwM2UyNmImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.17Aoqmog0A2GzPdnnoUfkTkCk4TuOrHgtkyr3yYVWSI)

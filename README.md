# WEATHER BOYZ

[WeatherXM Hackathon](https://plgenesis.devspot.app/en?activeTab=overview&challenge=weather-xm)

## The One Sentence Description

A web3 application that allows users to buy weather insurance using weatherXM data to verify weather conditions and execute smart contracts.

## Linux Setup

**Requirements**

- [Docker](https://docs.docker.com/desktop/setup/install/linux/)
- [Rust + Cargo](https://www.rust-lang.org/tools/install)
- [Node.js + npm + nvm](https://nodejs.org/en/download)
- [MetaMask](https://chromewebstore.google.com/detail/metamask/nkbihfbeogaeaoehlefnkodbefgpgknn?hl=en&pli=1)

<br>

**Recommended**

- [Beekeeper Studio](https://www.beekeeperstudio.io/get)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
- [Prettier](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)

<br>

**VS Code Extensions**

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
- [solidity](https://marketplace.visualstudio.com/items?itemName=JuanBlanco.solidity)
- [Solidity](https://marketplace.visualstudio.com/items?itemName=NomicFoundation.hardhat-solidity)
- [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
- [YAML](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml)

### Backend Setup

**Set up database**

Start development database

```bash
docker compose up -d
```

Copy `.env` file in `weather-boyz/backend/` directory from the example file

```bash
cd backend
cp .env.example .env
```

Create database

```bash
sqlx database create
```

Run migrations

```bash
sqlx migrate run
```

Run the backend server

```bash
cargo run
```

### Frontend Setup

Copy `.env` file in `weather-boyz/frontend/` directory from the example file

```bash
cd frontend
cp .env.example .env
```

Install dependencies

```bash
npm install
```

Run the development server

```bash
npm run dev
```

### Blockchain Setup

copy `.env` file in `weather-boyz/blockchain/` directory from the example file

```bash
cd blockchain
cp .env.example .env
```

Install dependencies

```bash
npm install
```

Run the Hardhat node

```bash
npx hardhat node
```

When node is running, it will output a list of accounts with their private keys. Copy one of the private keys and add it to your MetaMask wallet. You will have to add the local network manually in MetaMask with the following settings:

- Network Name: `Hardhat Local`
- New RPC URL: `http://127.0.0:8545`
- Chain ID: `31337`
- Currency Symbol: `ETH`

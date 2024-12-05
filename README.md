# Solana Pool Wallet System

This project implements a Pool Wallet system on the Solana blockchain using Rust. It enables the creation of a Pool Wallet PDA (Program Derived Address) responsible for managing rent payments and other operations for user accounts. The system is designed with efficiency, security, and scalability in mind.

---

## Features

- **Pool Wallet Management**: Create and initialize a Pool Wallet PDA with a predefined balance.
- **User Wallet Creation**: Generate PDAs for user wallets, with rent payments covered by the Pool Wallet.
- **Borsh Serialization**: Efficient serialization of wallet data for storage and retrieval.
- **Secure PDA Operations**: Uses `invoke_signed` for secure transactions signed by the PDAs.

---

## Prerequisites

To work with this project, ensure you have the following:

- [Rust](https://www.rust-lang.org/) installed.
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) installed and configured.
- Basic knowledge of the Solana blockchain and Rust programming.

---

## Setup

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/your-repository.git
cd your-repository
```

### 2. Install Dependencies

Ensure you have Rust and Solana CLI installed. If not, follow these links:

- [Install Rust](https://www.rust-lang.org/tools/install)
- [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)

### 3. Build the Program

Run the following command to build the Solana program:

```bash
cargo build-bpf
```

---

## Usage

### 1. Deploy the Program

Deploy your program to the Solana devnet or mainnet using the Solana CLI:

```bash
solana program deploy target/deploy/your_program.so
```

### 2. Interact with the Program

Use the Solana CLI or custom scripts to create and manage the Pool Wallet and user wallets.

---

## Contributing

Contributions are welcome! Feel free to fork the repository and submit pull requests.

---

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

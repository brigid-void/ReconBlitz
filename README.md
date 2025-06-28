# ReconBlitz

ReconBlitz is a lightweight, Rust-based reconnaissance tool designed for security professionals, bug bounty hunters, and CTF players. It streamlines the process of running multiple scanning tools against a target and aggregates the results into a single report.

## Features

- **Profile-based Scanning**: Use predefined profiles (`fast`, `full`) to run different sets of tools.
- **Extensible**: Easily add new tools to the scanner.
- **HTML & JSON Reporting**: Generates easy-to-read HTML reports and machine-readable JSON output.

## Quickstart

1. **Build the tool**:
   ```bash
   cargo build --release
   ```

2. **Run a scan**:
   ```bash
   ./target/release/reconblitz --target example.com --profile fast
   ```

3. **View the report**:
   Check the generated `reconblitz_report_*.html` file in your current directory.

## Usage

To run a scan, use the following command:

```bash
./target/release/reconblitz --target <TARGET_DOMAIN> --profile <PROFILE_NAME> --format <html|json>
```

**Example:**

```bash
./target/release/reconblitz --target example.com --profile fast --format html
```

## Output

- **HTML**: A file named `report.html` will be created in the project's root directory.
- **JSON**: The output will be printed directly to the console.

## Building from Source

1.  Ensure you have Rust and Cargo installed.
2.  Clone the repository: `git clone <repo_url>`
3.  Build the project: `cargo build --release`
4.  The executable will be in `target/release/reconblitz`.
> Rust-based Network Reconnaissance Bundler for Bug Bounty, CTF, and Lab Environments

## Features
- **Tool Orchestration**: Bundles nmap, masscan, ffuf, amass, and subfinder
- **Parallel Execution**: Runs tools concurrently with configurable timeouts
- **Security Focus**: Input sanitization and safe command execution
- **Multiple Profiles**: Pre-configured scanning profiles (fast/deep/web)
- **Unified Reporting**: JSON/HTML output with critical findings summary

## Installation

### Prerequisites
- Rust toolchain: [Install Rust](https://www.rust-lang.org/tools/install)
- Docker (optional for containerized execution)

### From Source
```bash
git clone https://github.com/yourusername/ReconBlitz.git
cd ReconBlitz
cargo build --release
```

### Docker
```bash
docker build -t reconblitz .
```

## Usage

### Command Line
```bash
# Basic scan with fast profile
./target/release/reconblitz --target example.com --profile fast

# Deep scan with JSON output
./target/release/reconblitz --target example.com --profile deep -o report.json
```

### Docker
```bash
docker run -it reconblitz --target example.com --profile web
```

## Security Features
1. **Input Sanitization**: All targets are validated before scanning
2. **Timeouts**: Default 5-minute timeout per tool
3. **Containerization**: Optional Docker execution for isolation
4. **Error Handling**: Graceful failure for tool execution errors

## Cloud Deployment

ReconBlitz can be deployed to cloud environments for scalable distributed scanning.

### AWS ECS
1. Build Docker image: `docker build -t reconblitz .`
2. Create ECR repository
3. Push image: `docker tag reconblitz:latest <account-id>.dkr.ecr.<region>.amazonaws.com/reconblitz:latest`
   `docker push <account-id>.dkr.ecr.<region>.amazonaws.com/reconblitz:latest`
4. Create ECS task definition referencing the image

### Automated CI/CD
See [.github/workflows/deploy.yml](.github/workflows/deploy.yml) for GitHub Actions deployment to AWS ECS.

## Showcase

![ReconBlitz Demo](https://user-images.githubusercontent.com/.../reconblitz-demo.gif)

## Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit changes (`git commit -am 'Add some feature'`)
4. Push to branch (`git push origin feature/your-feature`)
5. Open a pull request

## License
MIT - See [LICENSE](LICENSE) for details.

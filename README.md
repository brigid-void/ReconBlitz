# ReconBlitz

ReconBlitz is a lightweight, Rust-based reconnaissance tool designed for security professionals, bug bounty hunters, and CTF players. It streamlines the process of running multiple scanning tools against a target and aggregates the results into a single report.

## Features

- **Tool Orchestration**: Bundles nmap, masscan, ffuf, amass, and subfinder
- **Parallel Execution**: Runs tools concurrently with configurable timeouts
- **Security Focus**: Input sanitization and safe command execution
- **Multiple Profiles**: Pre-configured scanning profiles (fast/deep/web)
- **Unified Reporting**: JSON/HTML output with critical findings summary
- **Hybrid Scanning**: Uses a hybrid scanning approach with RustScan and nmap for fast and accurate results. See [Scanning Flow](docs/scanning-flow.md) for more details.

## Security Enhancements
- **Input Validation**: All target inputs are validated as valid IPs or domain names
- **Least Privilege**: Runs as non-root user in Docker container
- **Dependency Scanning**: cargo-audit integrated for vulnerability detection
- **Secure Defaults**: Timeouts and resource limits enabled by default

## Robustness Features
- **Configurable Timeouts**: Set per-scan timeout with `-t` option (default: 300s)
- **Resource Management**: Control max concurrent scans with `-c` option (default: 5)
- **Enhanced Logging**: File-based logging via `-l` option

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

To create your own demo:
1. Install [asciinema](https://asciinema.org/)
2. Record your session: `asciinema rec`
3. Upload to asciinema.org

## Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit changes (`git commit -am 'Add some feature'`)
4. Push to branch (`git push origin feature/your-feature`)
5. Open a pull request

## License
MIT - See [LICENSE](LICENSE) for details.

## Development Note
This project is developed using [Windsurf](https://windsurf.codes) powered by DeepSeek R1, allowing for AI-assisted development on the go.
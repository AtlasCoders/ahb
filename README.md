# AHB (Atlas Http Benchmarker)
![discord server icon](/discord_icon.png)


AHB (Atlas Http Benchmarker) is a command-line interface (CLI) application developed by the Atlas Coders community. It is written in Rust and designed to benchmark HTTP servers by simulating various levels of traffic and measuring their performance.

## Features

- **HTTP Benchmarking**: AHB allows you to benchmark HTTP servers by sending a configurable number of concurrent requests and measuring response times and throughput.
- **Flexible Configuration**: You can customize various parameters such as the number of requests, concurrency level, request timeout, and more to suit your specific benchmarking needs.
- **Multiple Request Methods**: AHB supports various HTTP methods, including GET, POST, PUT, DELETE, and HEAD, enabling you to test different aspects of your server's performance.
- **Statistics and Metrics**: The application provides detailed statistics and metrics, such as response times, throughput, success rate, and error rate, allowing you to analyze and evaluate your server's performance under different loads.


## Getting Started

To get started with AHB, follow these steps:

1. **Installation**: Clone the AHB repository from the Atlas Coders GitHub organization and build the application using Cargo.
```
$ git clone https://github.com/AtlasCoders/ahb.git
$ cd ahb
$ cargo build --release
```
2. **Usage**: Run the compiled binary with the desired command-line arguments to perform benchmarking.
```
$ ./ahb --url http://example.com --concurrency 10 --requests 1000 --method GET
```
## Contributing

We welcome contributions from the community to enhance AHB and make it even better. To contribute, please follow these guidelines:

- Clone the repository and create a new branch for your contribution.
- Make your changes and ensure that the codebase adheres to the project's coding conventions and style guidelines.
- Write tests for any new features or modifications to ensure the stability of the application. (Please, otherwise I will have to do that myself :'()
- Submit a pull request, providing a clear explanation of your changes and the problem they solve.

## License

AHB is open source software released under the `MIT License`. You are free to use, modify, and distribute the application in accordance with the terms of the license.

## Contact

For any questions, suggestions, or discussions related to AHB, you can reach out to the Atlas Coders community on our Discord server or open an issue on the GitHub repository.

- Discord: [Atlas Coders](https://discord.gg/29eDVqUva2)
- GitHub: [Atlas Coders Organization](https://github.com/AtlasCoders/ahb)

We look forward to your involvement and contributions to AHB! Let's build a powerful HTTP benchmarking tool together.

# LoadVanguard
LoadVanguard is a command-line based load testing tool designed for performance evaluation of REST APIs. Built with Rust, this tool is ideal for developers and testers who need to assess the robustness and scalability of their web services under various load conditions.

## Getting Started
### Prerequisites
- Rust (latest stable version)
- Cargo (Rust's package manager)

### Installation
Clone the repository:
```shell
git clone https://github.com/joeljacobdev/load_vanguard.git
cd load_vanguard
``````
To run and build the project using Cargo, first create a src/.env file by copying from src/.env.template and update the variables.
Run the project directly using:
```sh
cargo run
```

You can build a standalone release executable using:
```sh
cargo build --release
```
The executable will be located at target/release/load_vanguard.

### Configuration
Configure your load tests in a YAML file. Here's a sample configuration:

```yml
base_url: https://jsonplaceholder.typicode.com
headers:
  app_type: loadtesting
scenarios:
- title: scene
  frequency: 5
  apis:
  - path: /todos/2
    identifier: get_todo_1
    method: GET
  - path: /todos/1
    identifier: get_todo_2
    method: GET
```

## Roadmap
- [ ] Add different metrics for detailed performance analysis.
- [ ] Integrate instance metrics from cloudwatch for AWS-hosted services.

## Contributing
Contributions to this project are welcome. If you want to contribute, please follow the standard open-source practices for raising issues/features and submitting pull requests.

## License
This project is licensed under the AGPLv3 License - see the [LICENSE](./LICENSE) file for details.

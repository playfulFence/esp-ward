# Contributing to `esp-ward`

I'm excited to have your contribute in `esp-ward` project! Whether you're looking to add support for new peripherals, improve connectivity features, or enhance display support, this guide will help you get started!

## Getting Started

Before contributing, please take a moment to read through the [README](README.md) and familiarize yourself with the project structure and goals. It's also helpful to read through any existing issue or Pull Requests to see if your contribution is already being worked on by someone else.

# Contributing Process

### Adding New Peripherals Support

To add support for new peripherals:

1.  Fork the repository and create a new branch for your contribution.
2.  Implement your new peripheral in the `src/peripherals` directory following the existing structure.
3.  In case your sensor is initialized in a way that makes it difficult or impossible to fit unifying traits - it's perfectly fine to do something like in the case of "ultrasonic_distance.rs", but respect the naming convention!
4.  If your peripheral gives rise to some new sensor type for this library - please feel free to create new traits for it as well (similar to the other existing "TemperatureSensor/HumiditySensor" and so on )
5.  Add or update examples to demonstrate the use of the new peripheral.
6.  Update the `index.html` if your changes introduce new functionality that should be documented.
7.  Add or update the documentation comments to reflect your changes.

### Adding New Displays

For new displays:

1.  Implement the new display driver in the `src/display` directory.
2.  Feel free to update existing display traits. There is no point in multiplying them uncontrollably, it is better to just leave empty functions if your new display cannot implement any of the functions
3.  Follow the existing naming conventions and modular structure.
3.  Add an example under the `examples` directory to showcase the new display.
4.  Document your changes in the code and update the `index.html` as needed.

### Enhancing Connectivity Features

To enhance connectivity features:

1.  Propose your changes by creating an issue.
3.  Once agreed upon, implement the changes in the `src/connectivity.rs` file.
4.  Update or create new examples to illustrate the use of enhanced connectivity features.
5.  Make sure to document your code and update the `index.html` file.

## Pull Request Process

1.  When you're ready to submit your changes, push changes to your fork and open a Pull Request against the origin `esp-ward` repository.
2.  In the Pull Request, provide a detailed description, reason of the changes. It will accelerate the reviewing process
3.  Your Pull Request will be reviewed by the maintainers and, possibly, other community members.
4.  Engage in the review process by responding to comments and making any necessary adjustments.
5.  Once approved, your Pull Request will be merged into the project.

## Best Practices

*   **Commit Messages**: Use meaningful commit messages that clearly describe the changes you've made.
*   **Testing**: Describe the testing process
*   **Documentation**: Keep the documentation up-to-date with your changes.
*   **Code Style**: Follow the existing code style (`rustfmt.toml`) and use the Rust formatter for consistency.

## PR merged!

That's it! I'm thrilled that you're interested in contributing to the `esp-ward` crate. Your work helps us create a robust and versatile environment that benefits a wide range of users, and most importantly, newcomers who are just at the beginning of their Rust journey!

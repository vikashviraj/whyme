# Contributing to WhyMe

Thank you for your interest in contributing to WhyMe! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Be patient with questions

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/vikashviraj/whyme/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Platform and version information
   - Relevant logs or error messages

### Suggesting Features

1. Check existing [Issues](https://github.com/vikashviraj/whyme/issues) and [Discussions](https://github.com/vikashviraj/whyme/discussions)
2. Open a new issue or discussion with:
   - Clear description of the feature
   - Use case and motivation
   - Potential implementation approach (if you have ideas)

### Submitting Code

1. **Fork the repository**
2. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/bug-description
   ```

3. **Make your changes**:
   - Write clear, documented code
   - Follow Rust conventions (`cargo fmt`, `cargo clippy`)
   - Add tests if applicable
   - Update documentation

4. **Test your changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

5. **Commit your changes**:
   ```bash
   git commit -m "Add: feature description"
   # Use prefixes: Add, Fix, Update, Remove, Refactor, Docs
   ```

6. **Push and create a Pull Request**:
   ```bash
   git push origin feature/your-feature-name
   ```

## Development Guidelines

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy -- -D warnings`
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and small

### Testing

- Add unit tests for new functionality
- Test on your platform before submitting
- If possible, test on multiple platforms
- Document any platform-specific behavior

### Documentation

- Update README.md if adding features
- Add inline comments for complex logic
- Update this file if changing contribution process

## Pull Request Process

1. Ensure your code compiles and tests pass
2. Update documentation as needed
3. Write a clear PR description:
   - What changes were made
   - Why the changes were needed
   - How to test the changes
4. Reference any related issues
5. Wait for review and address feedback

## Questions?

Feel free to:
- Open a discussion for questions
- Ask in an issue thread
- Reach out to maintainers

Thank you for contributing! 🎉


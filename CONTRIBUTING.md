# Contributing to EVM-Indexer

Thank you for your interest in contributing to EVM-Indexer! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a new branch for your feature/fix
4. Make your changes
5. Test your changes
6. Create a Pull Request

## Development Process

1. **Open an Issue First (Recommended)**
   - For new features
   - For bug fixes
   - For improvements
   - This helps discuss the changes before investing time in coding

2. **Code Changes**
   - Write clean, documented code
   - Follow Rust best practices
   - Keep commits atomic and well-described
   - Sign all your commits (required)

3. **Testing**
   - Add tests for new features
   - Ensure all existing tests pass
   - Test with different chains and configurations

4. **Pull Request Process**
   - Reference the related issue
   - Provide a clear description of changes
   - Include any relevant documentation updates
   - Ensure all tests pass
   - Sign your commits

## Commit Signing

All commits MUST be signed. To set up commit signing:

1. Generate a GPG key if you don't have one:
```bash
gpg --full-generate-key
```

2. Configure Git to use your GPG key:
```bash
git config --global user.signingkey <your-key-id>
git config --global commit.gpgsign true
```

3. Add your GPG key to your GitHub account:
   - Go to Settings > SSH and GPG keys
   - Add your GPG key

## Code Style

- Follow Rust standard formatting (use `rustfmt`)
- Document public APIs
- Use meaningful variable names
- Keep functions focused and concise

## Questions?

Feel free to open an issue or contact the maintainers if you have any questions.

## License

By contributing to this project, you agree to license your contributions under the [MIT License](LICENSE).

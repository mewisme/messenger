# Messenger

A desktop application wrapper for Facebook Messenger built with Tauri.

## Description

Messenger is a native desktop application that wraps the Messenger web interface (messenger.com) in a standalone window. It provides a desktop experience for Facebook Messenger with native features like notifications and clipboard management.

## Features

- Native desktop application for Windows, macOS, and Linux
- System notifications support
- Clipboard manager integration
- Window positioning and management

## Prerequisites

- Node.js (LTS version)
- pnpm (package manager)
- Rust (stable toolchain)
- Platform-specific build tools:
  - Windows: Microsoft Visual Studio C++ Build Tools
  - macOS: Xcode Command Line Tools
  
## Installation

### Development Setup

1. Clone the repository:
```
git clone https://github.com/mewisme/messenger.git
cd messenger
```

2. Install dependencies:
```
pnpm install
```

3. Run in development mode:
```
pnpm tauri dev
```

### Building

Build the application:
```
pnpm tauri build
```

The built application will be in `src-tauri/target/release/` (or `src-tauri/target/debug/` for debug builds).

## Project Structure

```
messenger/
├── src/                    # Frontend source files
│   └── main.ts            # Main TypeScript entry point
├── src-tauri/             # Tauri backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   └── lib.rs         # Main application logic
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── package.json           # Node.js dependencies
└── vite.config.ts         # Vite configuration
```

## Configuration

The application configuration is located in `src-tauri/tauri.conf.json`. Key settings include:

- Window size and minimum dimensions
- Application identifier
- Security policies

## Development

### Running in Development Mode

The development server runs on port 1420 and loads messenger.com directly:

```
pnpm tauri dev
```

### Building for Production

Build for all platforms:
```
pnpm tauri build
```

Build for a specific platform:
```
pnpm tauri build --target x86_64-pc-windows-msvc    # Windows
pnpm tauri build --target x86_64-apple-darwin       # macOS Intel
pnpm tauri build --target aarch64-apple-darwin      # macOS Apple Silicon
pnpm tauri build --target x86_64-unknown-linux-gnu # Linux
```

## Technologies

- Tauri 2: Framework for building desktop applications
- Rust: Backend language
- TypeScript: Frontend language
- Vite: Build tool and development server
- pnpm: Package manager

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Mew

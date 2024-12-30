# CanvasCode

A web-based infinite canvas code editor that allows you to visualize and edit your codebase in a spatial environment. Built with Rust backend and Monaco Editor frontend.

## Features

- 🎨 Infinite canvas workspace for code editing
- 📝 Multiple editor instances on the canvas
- 🔍 File explorer with easy file access
- 🖱️ Drag and drop editor windows
- 🔄 Zoom and pan controls
- 💻 Monaco Editor integration with syntax highlighting
- 🚀 Fast Rust backend server
- 🐳 Docker support for easy deployment

## Prerequisites

- Docker and Docker Compose
- Rust (for local development)
- Node.js (for frontend development)

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/yourusername/CanvasCode.git
cd CanvasCode
```

2. Start the application using Docker Compose:
```bash
docker-compose up --build
```

3. Open your browser and navigate to:
```
http://localhost:8080
```

## Development Setup

### Backend (Rust)
```bash
cd backend
cargo run
```

### Frontend
The frontend is served statically through the Rust backend server.

## Project Structure

- `/backend` - Rust backend server
- `/frontend` - Static frontend files
- `Dockerfile` - Docker configuration for the application
- `docker-compose.yaml` - Docker Compose configuration

## License

MIT License

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request



╔═════════════════════════════════════════════════════════════════╗
║                                                                 ║
║   ██████╗ ██╗  ██╗ ██████╗ ███╗   ██╗███████╗ █████╗ ███████╗   ║
║   ██╔══██╗██║  ██║██╔═══██╗████╗  ██║██╔════╝██╔══██╗██╔════╝   ║
║   ██████╔╝███████║██║   ██║██╔██╗ ██║█████╗  ███████║███████╗   ║
║   ██╔═══╝ ██╔══██║██║   ██║██║╚██╗██║██╔══╝  ██╔══██║╚════██║   ║
║   ██║     ██║  ██║╚██████╔╝██║ ╚████║███████╗██║  ██║███████║   ║
║   ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝╚══════╝   ║
║                                                                 ║
╚═════════════════════════════════════════════════════════════════╝

All generated using Cursor - The AI-first Code Editor
"Turn your phone into a mic, let your voice be heard through the bytes."

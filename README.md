<div align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/75442e77-0211-4fb5-9a17-1a0bed89426f">
  <img width="500" alt="OminiX-MLX" src="[https://github.com/user-attachments/assets/b168cf1c-8e2f-4969-bffa-b57ee33950c0](https://github.com/user-attachments/assets/063e750e-ac4b-48e6-ba20-f9b4ac5bbe04)" />
</picture>

# OminiX Studio

</div>

A native desktop AI application for interacting with local and cloud-based models, built with Rust and the [Makepad](https://github.com/makepad/makepad) UI framework.

OminiX Studio provides a unified interface for chatting with LLMs, generating images, transcribing speech, and managing models — all running natively on Apple Silicon with no Python runtime required.


## The OminiX Ecosystem

OminiX Studio is the user-facing layer of a three-part stack, all written in Rust:

```
┌─────────────────────────────────────────────┐
│            OminiX Studio (this repo)        │  Desktop UI
│         Chat · Models · Voice · Settings    │
└──────────────────────┬──────────────────────┘
                       │ OpenAI-compatible API
┌──────────────────────▼──────────────────────┐
│               OminiX-API                    │  Local API server
│    LLM · Image · ASR · TTS endpoints        │
└──────────────────────┬──────────────────────┘
                       │ Rust bindings
┌──────────────────────▼──────────────────────┐
│               OminiX-MLX                    │  Inference engine
│   Metal-accelerated ML on Apple Silicon     │
└─────────────────────────────────────────────┘
```

- [**OminiX-MLX**](https://github.com/OminiX-ai/OminiX-MLX) — Pure-Rust inference engine built on Apple's MLX framework. Leverages Metal GPU acceleration and unified memory for high-throughput inference (e.g. ~45 tok/s for LLMs, 18x real-time for ASR on M3 Max). Supports LLMs (Qwen, GLM, Mistral), image generation (FLUX, Z-Image), speech recognition (Paraformer), and voice cloning (GPT-SoVITS).

- [**OminiX-API**](https://github.com/OminiX-ai/OminiX-API) — OpenAI-compatible HTTP/WebSocket server that wraps OminiX-MLX. Provides chat completions, image generation, transcription, and TTS endpoints. Supports dynamic model loading and switching at runtime without server restarts.

- **OminiX Studio** (this repo) — The desktop application. Connects to OminiX-API for local inference and also supports cloud providers (OpenAI, Anthropic, Google Gemini, DeepSeek, OpenRouter, and more).

All three projects are available at [github.com/OminiX-ai](https://github.com/OminiX-ai).

## Features

- **Multi-provider chat** — Talk to local models via OminiX-API/Ollama, or cloud models from OpenAI, Anthropic, Gemini, DeepSeek, OpenRouter, and SiliconFlow
- **Local model management** — Download, import, and run models directly on Apple Silicon
- **Image generation** — Generate images through local or cloud endpoints
- **Voice input/output** — Speech-to-text and text-to-speech support
- **MCP support** — Model Context Protocol integration for tool use (desktop)
- **Chat history** — Persistent, searchable conversation history
- **Dark mode** — Full light/dark theme support

## Project Structure

```
OminiX-Studio/
├── moly-shell/          # Main application binary (ominix-studio)
├── moly-data/           # Shared state, persistence, API clients
├── moly-widgets/        # Reusable UI components and theming
└── apps/
    ├── moly-chat/       # Chat interface
    ├── moly-models/     # Model discovery and downloads
    ├── moly-settings/   # Provider and API key configuration
    ├── moly-local-models/ # Local model management (MLX)
    ├── moly-mcp/        # MCP server configuration
    └── moly-voice/      # Voice I/O
```

## Requirements

- macOS 14.0+ (Sonoma) on Apple Silicon (M1/M2/M3/M4)
- Rust 1.82+

## Getting Started

```bash
# Clone the repository
git clone https://github.com/OminiX-ai/OminiX-Studio.git
cd OminiX-Studio

# Build and run
cargo run -p moly-shell
```

To use local model inference, you'll also need to set up [OminiX-API](https://github.com/OminiX-ai/OminiX-API) — see its README for instructions.

For cloud providers, open Settings in the app and configure your API keys.

## License

[Apache 2.0](LICENSE)

[English version](docs/CROSS_PLATFORM.md) | [Versão em Português Brasileiro](docs/CROSS_PLATFORM.pt-BR.md)

> A dor que você já conhece: uma CLI que roda em um SO e finge ser portável.

# Suporte Multiplataforma

## A Dor Que Você Já Conhece

Você quer entregar uma skill de transcrição aos seus usuários.
Metade tem macOS Apple Silicon, a outra metade tem Macs Intel,
máquinas Windows ou servidores Linux. Você não pode entregar um
único binário que funcione em todos os lugares porque o backend
Metal do whisper.cpp é exclusivo para macOS.

## Por Quê

- A aceleração GPU do whisper.cpp requer Apple Metal
- O fallback CPU-only em Linux ou Windows é 50x mais lento
- Compilação cruzada de binários Metal não é suportada pela Apple
- Forçar portabilidade sacrifica a qualidade de vida que o projeto
  foi construído para entregar

## Soberania

- macOS Apple Silicon é o alvo primário
- Outras plataformas são explicitamente excluídas, não quebradas por
  acidente
- Usuários em outras plataformas são roteados para whisper.cpp upstream

## Matriz de Suporte

| Target                    | Tier   | Status         |
|---------------------------|--------|----------------|
| aarch64-apple-darwin      | Tier 1 | Suporte total  |
| x86_64-apple-darwin       | Tier 2 | Compila apenas |
| x86_64-unknown-linux-gnu  | Nenhum | Não suportado  |
| aarch64-unknown-linux-gnu | Nenhum | Não suportado  |
| x86_64-pc-windows-msvc   | Nenhum | Não suportado  |

## Notas sobre macOS

- Apple Silicon é necessário porque o backend Metal do whisper.cpp
  é o único caminho de aceleração GPU
- macOS 13 (Ventura) ou superior é necessário para Metal 3
- Xcode Command Line Tools fornece o compilador Metal
- O modelo padrão carrega em memória unificada

## Notas sobre Linux

- Não suportado
- O backend GPU Metal não tem implementação Linux
- whisper.cpp CPU-only é 50x mais lento que Metal
- Usuários em Linux devem usar whisper.cpp upstream ou faster-whisper

## Notas sobre Windows

- Não suportado
- Mesma razão do Linux
- Builds Windows exigiriam um fork separado do whisper.cpp
- Usuários em Windows devem usar WSL com whisper.cpp Linux

## Notas sobre Containers

- Não publicado como container
- Containers macOS não existem para uso em produção
- Use um host macOS com uma toolchain Rust

## Suporte de Shells

A CLI é testada em:

- bash 5.x
- zsh 5.x
- fish 3.x
- nushell 0.80+

## Paths de Arquivos e XDG

O diretório de cache do modelo é:

- macOS: `~/Library/Application Support/whisper-macos-cli/models/`
- Linux: `~/.local/share/whisper-macos-cli/models/` (se suportado)
- Windows: `%APPDATA%\whisper-macos-cli\models\` (se suportado)

## Performance por Target

- aarch64-apple-darwin (M1): 1x tempo real
- aarch64-apple-darwin (M2 Pro): 0.5x tempo real
- aarch64-apple-darwin (M3 Max): 0.3x tempo real
- x86_64-apple-darwin (Intel): 5x tempo real (CPU apenas)

## Agentes Validados por Plataforma

- macOS Apple Silicon: Claude Code, OpenCode, Codex CLI, Gemini CLI,
  Cline, Cursor, Windsurf, Aider, Continue, Cody, Tabnine, Replit Agent
- Todas as outras plataformas: não validados, não suportados

> Transcreva qualquer áudio localmente em Apple Silicon em segundos, não em minutos.

# whisper-macos-cli

[![Crates.io](https://img.shields.io/crates/v/whisper-macos-cli.svg)](https://crates.io/crates/whisper-macos-cli)
[![Documentation](https://docs.rs/whisper-macos-cli/badge.svg)](https://docs.rs/whisper-macos-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/daniloaguiarbr/whisper-macos-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/daniloaguiarbr/whisper-macos-cli/actions)
[![codecov](https://codecov.io/gh/daniloaguiarbr/whisper-macos-cli/branch/main/graph/badge.svg)](https://codecov.io/gh/daniloaguiarbr/whisper-macos-cli)
[![Audit](https://img.shields.io/badge/audit-cargo%20audit-blue)](https://github.com/rustsec/rustsec)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-blue)](https://github.com/rust-lang/rust/releases)

[English version](README.md)

## O que é

- CLI de transcrição local de áudio para macOS Apple Silicon
- Alimentada por whisper.cpp com aceleração Metal GPU
- Contrato JSON estrito stdin/stdout para agentes de IA
- Zero telemetria, zero chamadas em nuvem, zero configuração além de `cargo install`

## Por quê

- Transcrição de áudio como serviço tranca seus dados com terceiros
- Modelos Whisper em Python são 10x mais lentos e 5x mais pesados que whisper.cpp
- A maioria das CLIs trata stdout como lixeira; tratamos como contrato

## Superpoderes

- Descobrível: `whisper-macos-cli commands` emite a árvore completa
- Auto-descritiva: `whisper-macos-cli schema` retorna o JSON Schema completo
- Rastreável: cada saída carrega um UUID v7 em `correlation_id`
- Versionada: cada saída carrega um `schema_version` para evolução segura
- Resiliente: SIGINT e SIGTERM disparam shutdown limpo; duplo Ctrl+C força saída
- Segura: downloads de modelo são verificados por SHA256 e TLS
- Componível: comporta-se como qualquer ferramenta Unix — pipes, NDJSON, jaq, xargs

## Início Rápido

```bash
cargo install whisper-macos-cli
whisper-macos-cli models download
whisper-macos-cli transcribe voz.ogg
```

A primeira transcrição é mais lenta porque o modelo carrega em memória
unificada. Transcrições subsequentes reusam o contexto em cache.

## Instalação

### Pré-requisitos

- macOS 13 ou superior
- Apple Silicon (M1, M2, M3, M4)
- Xcode Command Line Tools: `xcode-select --install`
- cmake: `brew install cmake`
- Rust 1.88 ou superior: `rustup install stable`

### Do crates.io

```bash
cargo install whisper-macos-cli
```

### Do código-fonte

```bash
git clone https://github.com/daniloaguiarbr/whisper-macos-cli
cd whisper-macos-cli
cargo build --release
./target/release/whisper-macos-cli --version
```

### Binários pré-compilados

Baixe o binário apropriado para sua arquitetura na página
[GitHub Releases](https://github.com/daniloaguiarbr/whisper-macos-cli/releases).
Verifique o hash SHA256 contra `SHA256SUMS` antes de instalar.

## Uso

```bash
# Arquivo único
whisper-macos-cli transcribe gravacao.ogg

# Via stdin
cat audio.mp3 | whisper-macos-cli transcribe

# Lote como NDJSON
whisper-macos-cli transcribe *.ogg --ndjson --concurrency 4

# Forçar idioma
whisper-macos-cli transcribe --language pt audio.wav

# Modelo menor para velocidade
whisper-macos-cli transcribe --model small audio.wav

# Obter JSON Schema para validação
whisper-macos-cli schema > schema.json
whisper-macos-cli transcribe audio.ogg | jsonschema -i schema.json
```

## Comandos

| Subcomando  | Propósito                                |
|-------------|------------------------------------------|
| transcribe  | Transcreve um ou mais arquivos de áudio  |
| models      | Baixa, lista, localiza ou remove modelos |
| doctor      | Diagnostica ambiente e dependências     |
| schema      | Emite o envelope JSON Schema completo    |
| config      | Emite configuração efetiva atual         |
| commands    | Emite a árvore de comandos em JSON       |
| init        | Gera scaffold SKILL.md e AGENTS.md       |
| licenses    | Imite atribuição de licenças de terceiros|
| completions | Gera completions de shell                |
| resume      | Retoma lote anterior (v0.1: sem efeito)  |

Execute `whisper-macos-cli commands --format json` para ver a árvore completa.

## Variáveis de Ambiente

| Variável          | Efeito                                          |
|-------------------|-------------------------------------------------|
| WHISPER_MODEL     | Sobrescreve modelo padrão                      |
| WHISPER_LANGUAGE  | Sobrescreve idioma padrão                      |
| NO_COLOR          | Desabilita saída colorida                       |
| CI                | Desabilita prompts interativos (1, true, yes)   |
| RUST_LOG          | Sobrescreve filtro de nível de log              |
| SOURCE_DATE_EPOCH | Timestamp Unix para builds reproduzíveis       |
| NO_INPUT          | Sobrescreve flag --no-input                     |
| QUIET             | Sobrescreve flag --quiet                        |

## Padrões de Integração

```bash
# Pipe para jaq para extração seletiva
whisper-macos-cli transcribe audio.ogg | jaq -r '.text'

# Lote via fd e xargs
fd -e ogg . /path/to/audios/ \
  | xargs whisper-macos-cli transcribe --ndjson --concurrency 4

# Stream a partir de HTTP
xh -d https://example.com/audio.ogg | whisper-macos-cli transcribe

# Validar contra schema em CI
whisper-macos-cli transcribe audio.ogg \
  | jaq -e "has(\"correlation_id\") and has(\"schema_version\")"
```

## Performance

- Primeira transcrição (cold start, large-v3): 2-5 segundos de warmup
- Transcrições subsequentes: aproximadamente tempo real em M2 Pro
- Memória: large-v3 requer ~3 GB de memória unificada durante inferência
- Concorrência: escala linearmente até `--concurrency 8` em M1 Pro

## Requisitos de Memória

| Modelo   | Pico de Memória |
|----------|-----------------|
| tiny     | ~300 MB         |
| base     | ~500 MB         |
| small    | ~1 GB           |
| medium   | ~3 GB           |
| large-v3 | ~3.5 GB         |

Whisper.cpp descarrega o modelo quando o processo termina.

## FAQ de Solução de Problemas

Veja [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) para o guia
completo, incluindo:

- código de saída 64 (sem entrada)
- código de saída 65 (áudio inválido)
- código de saída 66 (arquivo não encontrado)
- código de saída 69 (download falhou)
- código de saída 70 (inferência falhou)
- código de saída 74 (erro de I/O)
- código de saída 78 (modelo não encontrado)

## Documentação

- [AGENTS.md](AGENTS.md) — Guia de integração para agentes
- [CHANGELOG.md](CHANGELOG.md) — Histórico de versões
- [CONTRIBUTING.md](CONTRIBUTING.md) — Como contribuir
- [SECURITY.md](SECURITY.md) — Reportar vulnerabilidades
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — Padrões da comunidade
- [PRIVACY.md](PRIVACY.md) — Política de manuseio de dados
- [INTEGRATIONS.md](INTEGRATIONS.md) — Agentes e plataformas suportados
- [llms.txt](llms.txt) — Resumo para LLMs
- [llms-full.txt](llms-full.txt) — Referência completa para LLMs
- [docs/HOW_TO_USE.md](docs/HOW_TO_USE.md) — Receitas avançadas
- [docs/AGENTS.md](docs/AGENTS.md) — Guia para integradores
- [docs/COOKBOOK.md](docs/COOKBOOK.md) — Vinte exemplos práticos
- [docs/CROSS_PLATFORM.md](docs/CROSS_PLATFORM.md) — Matriz de plataformas
- [docs/MIGRATION.md](docs/MIGRATION.md) — Migração de versões
- [docs/TESTING.md](docs/TESTING.md) — Guia de execução de testes
- [docs/schemas/](docs/schemas/README.md) — Schemas legíveis por máquina
- [skill/](skill/) — Descritores de skill para agentes

## Contribuindo

Veja [CONTRIBUTING.md](CONTRIBUTING.md) para o fluxo. Cada pull
request deve passar no checklist de 8 itens antes do merge.

## Segurança

Reporte vulnerabilidades via GitHub Security Advisories em
https://github.com/daniloaguiarbr/whisper-macos-cli/security/advisories/new
— não como issues públicas. SLA é de 72 horas para triagem inicial.

## Changelog

Veja [CHANGELOG.md](CHANGELOG.md) para o histórico completo. A versão
em desenvolvimento atual está documentada sob `## [Unreleased]`.

## Licença

MIT — veja [LICENSE](LICENSE). Avisos de terceiros em
[NOTICE](NOTICE) e via `whisper-macos-cli licenses`.

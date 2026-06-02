[English version](docs/HOW_TO_USE.md) | [Versão em Português Brasileiro](docs/HOW_TO_USE.pt-BR.md)

> Domine a CLI em 60 segundos, do zero ao pipeline de agente em produção.

# Como Usar

## A Dor

A maioria das ferramentas de transcrição vaza seu áudio para
terceiros. whisper-macos-cli roda inteiramente na sua máquina e
expõe um contrato JSON previsível. Este guia leva você do install à
integração de agente em produção em menos de 60 segundos.

## Por Quê

- Wrappers Whisper em Python adicionam 200 MB e 5 segundos de cold start
- APIs de nuvem trancam seus dados sob ToS que você não pode auditar
- A maioria das CLIs trata stdout como lixeira; tratamos como contrato

## Economia

- Um script de instalação de 200 linhas substitui 20 minutos de setup
  de modelo
- Uma chamada `whisper-macos-cli transcribe` substitui 50 linhas de
  Python
- Uma flag `--ndjson` substitui um parser de streaming customizado

## Soberania

- Seu áudio nunca sai do dispositivo
- Suas transcrições nunca saem do dispositivo
- Sem telemetria, sem analytics, sem phone-home
- O modelo é verificado por SHA256 antes do primeiro uso

## Pré-requisitos

- macOS 13 ou superior
- Apple Silicon (M1, M2, M3, M4)
- Xcode Command Line Tools: `xcode-select --install`
- cmake: `brew install cmake`
- Rust 1.88 ou superior

## Primeiro Comando em 60 Segundos

```bash
cargo install whisper-macos-cli
whisper-macos-cli models download
whisper-macos-cli transcribe ~/Desktop/audio-nota.ogg
```

A saída é um único objeto JSON no stdout. O modelo fica em cache
para todas as invocações futuras.

## Comandos Centrais

### Transcrever um arquivo único

```bash
whisper-macos-cli transcribe audio.ogg
```

### Transcrever via stdin

```bash
cat gravacao.mp3 | whisper-macos-cli transcribe
```

### Lote com NDJSON

```bash
whisper-macos-cli transcribe *.ogg --ndjson --concurrency 4
```

Cada arquivo emite um objeto JSON por linha. Uma linha final de
sumário reporta os totais.

### Forçar um idioma

```bash
whisper-macos-cli transcribe --language pt audio.wav
```

### Usar modelo menor para velocidade

```bash
whisper-macos-cli models download base
whisper-macos-cli transcribe --model base arquivo-grande.wav
```

### Obter o JSON Schema para validação downstream

```bash
whisper-macos-cli schema > schema.json
whisper-macos-cli transcribe audio.ogg | jaq -r .text
```

## Padrões Avançados

### Pipe a partir de HTTP

```bash
xh -d https://example.com/audio.ogg | whisper-macos-cli transcribe --quiet
```

### Extrair apenas o texto via jaq

```bash
whisper-macos-cli transcribe audio.ogg --quiet | jaq -r '.text'
```

### Dry run para validação em CI

```bash
whisper-macos-cli transcribe --dry-run --language pt audio.ogg
```

### Forçar saída JSON em modo agente

```bash
CI=true whisper-macos-cli transcribe --no-input --quiet audio.ogg
```

### Transcrição air-gapped

```bash
# Pré-baixar em uma máquina conectada
whisper-macos-cli models download large-v3

# Copiar para a máquina air-gapped
scp -r ~/Library/Application\ Support/whisper-macos-cli/ user@airgapped:

# Rodar com --offline para pular verificações de rede
whisper-macos-cli --offline transcribe --model large-v3 audio.ogg
```

## Configuração

Leia a configuração efetiva a qualquer momento:

```bash
whisper-macos-cli config
```

Sobrescreva padrões via variáveis de ambiente:

```bash
export WHISPER_MODEL=base
export WHISPER_LANGUAGE=en
export RUST_LOG=info
whisper-macos-cli transcribe audio.ogg
```

## Referência de Subcomandos Não Cobertos Acima

| Subcomando   | Propósito                                |
|--------------|------------------------------------------|
| models list  | Mostra modelos instalados e disponíveis  |
| models path  | Imprime o caminho do arquivo do modelo   |
| models remove| Deleta um modelo baixado                 |
| doctor       | Diagnostica ambiente e dependências     |
| commands     | Imprime árvore de comandos em JSON       |
| init         | Gera scaffold de skill                   |
| licenses     | Imprime atribuição de licenças           |
| resume       | Retoma lote anterior                     |

## Integração Com Agentes de IA

Veja [docs/AGENTS.md](docs/AGENTS.md) para o guia completo de autor
de agente incluindo matriz de compatibilidade, detalhes de contrato
e operações estilo CRUD.

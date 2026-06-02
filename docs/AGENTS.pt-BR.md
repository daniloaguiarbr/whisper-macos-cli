[English version](docs/AGENTS.md) | [Versão em Português Brasileiro](docs/AGENTS.pt-BR.md)

> Coloque em produção uma skill de transcrição em uma tarde, não em uma semana. Grátis, local, previsível.

# AGENTS — Guia do Autor

Este documento é escrito para engenheiros que constroem agentes de IA
que integram whisper-macos-cli como serviço de transcrição
caixa-preta.

## Agentes e Orquestradores Compatíveis

- Claude Code (Anthropic)
- OpenCode
- Codex CLI (OpenAI)
- Gemini CLI (Google)
- Cline
- Cursor
- Windsurf
- Aider
- Continue
- Cody (Sourcegraph)
- Tabnine
- Replit Agent

## Detalhes da Integração de Agente

Todo agente integra invocando o binário via subprocess e fazendo
parse do envelope JSON no stdout. O contrato é estável e versionado
via `schema_version`.

```python
import subprocess
import json

result = subprocess.run(
    ["whisper-macos-cli", "transcribe", "--quiet", audio_path],
    capture_output=True, text=True, check=True
)
output = json.loads(result.stdout)
text = output["text"]
correlation_id = output["correlation_id"]
```

## Integrações de Crates

A CLI é publicada como um único binário no crates.io. As crates do
ecossistema Rust que compõem o binário estão documentadas em
[docs/COOKBOOK.md](docs/COOKBOOK.md) e em
`whisper-macos-cli licenses`.

## Contrato Técnico (Estilo CRUD)

### Read (transcribe)

- Entrada: caminho de arquivo ou stdin
- Saída: envelope JSON no stdout
- Efeitos colaterais: pode escrever no diretório de cache de modelo
- Idempotente: sim (mesma entrada, mesmo modelo, mesma saída)
- Checkpointable: não

### Discovery

- Entrada: nenhuma
- Saída: árvore de comandos, JSON Schema, ou configuração JSON
- Efeitos colaterais: nenhum
- Idempotente: sim

### List (models)

- Entrada: nome de modelo opcional
- Saída: array de modelos com tamanho, descrição, flag downloaded
- Efeitos colaterais: nenhum
- Idempotente: sim

### Mutate (models download ou remove)

- Entrada: nome do modelo, flag dry-run opcional
- Saída: envelope de status com action, path, etag opcional
- Efeitos colaterais: escreve ou deleta arquivo no diretório de
  cache de modelo
- Idempotente: sim (download é no-op se já em cache)

## Padrão de Consumo JSON

O envelope é a unidade de consumo. Cada linha do stdout é um objeto
JSON completo e parseável. Streams NDJSON terminam com uma linha de
sumário contendo `"summary": true`.

```bash
whisper-macos-cli transcribe *.ogg --ndjson \
  | jaq -c 'select(.summary | not) | {file, text}'
```

## Padrão de Tratamento de Erros

Quando `error: true` é definido no envelope, o agente DEVE tratar o
resultado como falha e pode usar a flag `retryable` mais
`retry_after_ms` para decidir se deve retentar.

```python
output = json.loads(result.stdout)
if output.get("error"):
    if output["retryable"]:
        time.sleep(output["retry_after_ms"] / 1000)
        # retry
    else:
        # log and surface error to user
        pass
```

## Padrão Auto-Descritivo

A CLI pode fazer scaffold de seu próprio descritor de skill em
qualquer diretório alvo:

```bash
whisper-macos-cli init --target /caminho/do/workspace
```

Isso escreve `SKILL.md` e `AGENTS.md` no diretório alvo com o
contrato, exemplos e códigos de saída.

## Referência de Códigos de Saída

| Código | Significado             | Retentável |
|--------|-------------------------|------------|
| 0      | Sucesso                 | n/a        |
| 2      | Erro de uso             | não        |
| 64     | Nenhuma entrada         | não        |
| 65     | Áudio inválido          | não        |
| 66     | Arquivo não encontrado  | não        |
| 69     | Serviço indisponível    | sim        |
| 70     | Erro de inferência      | não        |
| 74     | Erro de I/O             | não        |
| 78     | Erro de configuração    | não        |
| 130    | SIGINT (Ctrl+C)         | não        |
| 141    | Pipe quebrado           | não        |
| 143    | SIGTERM                 | não        |

## Composição com Ferramentas Unix

- `xh` para downloads HTTP
- `fd` para descoberta de arquivos
- `bat` para preview com syntax highlighting
- `jaq` para query de JSON
- `ripgrep` para busca de texto
- `xargs` para dispatch paralelo
- `timeout` para execução com limite
- `procs` para inspeção de processos

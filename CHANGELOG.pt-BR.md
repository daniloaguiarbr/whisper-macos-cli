[English version](CHANGELOG.md) | [Versão em Português Brasileiro](CHANGELOG.pt-BR.md)

# Changelog

Todas as mudanças notáveis neste projeto são documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/),
e este projeto adota [Semantic Versioning](https://semver.org/lang/pt-BR/spec/v2.0.0.html).

## [Não Lançado]

### Alterado

- Licença alterada de MIT-only para dual MIT OR Apache-2.0
- `LICENSE-MIT` e `LICENSE-APACHE` substituem o arquivo único `LICENSE`
- Campo `license` em `Cargo.toml` agora é `MIT OR Apache-2.0`

### Adicionado

- Verificação de integridade SHA256 no download de modelos
- Identificação via User-Agent em requisições HTTP
- Retry com backoff exponencial para erros transitórios de download
- Validação de magic bytes antes do decode de áudio
- Descarte automático de pre-skip OGG/Opus (3840 samples a 48kHz)
- Limite máximo de duração de áudio de 24h (proteção DoS)
- Limite máximo de tamanho de stdin de 2 GB (proteção OOM)
- Normalização NFC do texto transcrito
- `correlation_id` (UUID v7) em toda saída JSON
- Campo `schema_version` em toda saída JSON
- Campo `docs_url` no envelope de erro
- Campo `retry_after_ms` no envelope de erro
- Linha de sumário NDJSON ao final de operações em lote
- Handler SIGINT com cleanup (não chama mais `process::exit`)
- Handler SIGTERM com graceful shutdown
- Duplo Ctrl+C força saída imediata
- Novos subcomandos: `commands`, `init`, `licenses`, `config`, `resume`
- Novas flags globais: `--print-config`, `--print-schema`, `--no-input`
- Novas flags de transcribe: `--dry-run`, `--timeout`, `--retry-count`,
  `--retry-max-elapsed`, `--offline`, `--resume`
- Suporte a variáveis de ambiente `WHISPER_MODEL` e `WHISPER_LANGUAGE`
- Variável de ambiente `CI=true` honrada
- `docs_url` por categoria de erro
- Detecção de `air-gapped` no subcomando `doctor`
- Verificação de espaço em disco no subcomando `doctor`
- Sonda de conectividade de rede no subcomando `doctor`
- `THIRD-PARTY-LICENSES.md` e subcomando `licenses`
- `CONTRIBUTING.md` com checklist de 8 itens
- `SECURITY.md` com SLA de 72h e política de divulgação coordenada
- `CODE_OF_CONDUCT.md` (Contributor Covenant v2.1)
- `PRIVACY.md` documentando manuseio de dados
- `llms-full.txt` para consumo abrangente por LLMs
- `README.pt-BR.md` tradução para português brasileiro
- `AGENTS.pt-BR.md` guia de integração em português
- Documentos `docs/*.pt-BR.md` espelhados
- Documentos `CHANGELOG.pt-BR.md`, `CONTRIBUTING.pt-BR.md`,
  `CODE_OF_CONDUCT.pt-BR.md`, `SECURITY.pt-BR.md`, `PRIVACY.pt-BR.md`
  espelhados
- `INTEGRATIONS.md` na raiz (movido de `docs/`)
- `docs/MIGRATION.md` e `docs/TESTING.md` e pares pt-BR
- `docs/schemas/` com 7 schemas JSON versionados e README bilíngue
- Subpastas `skill/whisper-macos-cli-en/` e `skill/whisper-macos-cli-pt/`
- `llms.pt-BR.txt` espelhado
- `deny.toml` para verificação de licenças e advisories
- `.cargo/audit.toml` para configuração de cargo-audit
- `.cargo/config.toml` para reprodutibilidade de build
- `.editorconfig` para consistência cross-editor
- `.gitattributes` para normalização de line endings
- `.github/workflows/ci.yml` com matrix, audit, deny, doc, coverage
- `.github/workflows/release.yml` para builds cross-platform
- `.github/dependabot.yml` para atualizações automatizadas
- `.github/ISSUE_TEMPLATE/bug.md` para reports estruturados
- `.github/PULL_REQUEST_TEMPLATE.md` com checklist de 12 itens
- `proptest` e `insta` como dev-dependencies
- Scaffolding de benchmark `criterion`
- `wiremock` para mock de HTTP em testes
- `serial_test` para execução serial de testes

### Alterado

- Registry de modelos agora armazena `min_size_bytes` para rejeição
  de downloads parciais
- `error::Error::to_json` agora requer parâmetro `correlation_id`
- Todos os comandos propagam `correlation_id` pela call stack
- `output::write_error` agora requer parâmetro `correlation_id`
- `signal::install_ctrlc_handler` renomeada para `install_handlers`
  e adiciona suporte a SIGTERM
- `eprintln!` substituído por `tracing::info!` em `transcribe.rs` e
  `models.rs`
- Texto de transcrição normalizado para Unicode NFC antes de serializar
- Build requer Rust 1.88 MSRV
- Cargo.toml usa `exclude` (allowlist invertida) ao invés de `include`
- Estrutura `skill/` organizada em subpastas por idioma
- Cada documento público abre com link cruzado para o idioma oposto

### Segurança

- Documentação `# Safety` adicionada a todos os blocos `unsafe`
- Verificação SHA256 no download de modelos
- Identificação via User-Agent em todas as requisições HTTP
- Verificação de `min_size_bytes` para rejeitar downloads parciais
- `cleanup_partial_downloads` para remover temp files em sinal
- Classificação explícita de retry (5xx e 429 são transientes)
- Proteção DoS de 24h na duração de áudio
- Proteção OOM de 2 GB no tamanho de stdin

## [0.1.0] - 2026-06-01

### Adicionado

- Lançamento inicial
- Transcrição de áudio via whisper.cpp com aceleração Metal GPU
- Suporte a MP3, WAV, FLAC, AAC, OGG/Vorbis, OGG/Opus (WhatsApp)
- 5 tamanhos de modelo: tiny, base, small, medium, large-v3 (padrão)
- VAD (Voice Activity Detection) via Silero para prevenir alucinações
- Detecção automática de idioma a partir do locale do macOS
- Modo `--language auto` para detecção nativa do whisper.cpp
- Modos de saída JSON e NDJSON para integração com agentes
- Transcrição paralela via `--concurrency`
- Subcomando `doctor` para diagnóstico de ambiente
- Subcomando `schema` para introspecção de JSON Schema
- Subcomando `completions` para geração de completions de shell
- Flag global `--print-schema`
- Flag `--color` com modos auto/always/never
- Erro JSON estruturado no stdout com campos category e retryable
- Download de modelos atômico com barra de progresso
- Decoding BeamSearch com beam size configurável (padrão 8)
- Filtragem de alucinações e colapso de repetições consecutivas
- AGENTS.md, SKILL.md, llms.txt para descoberta por agentes

[Não Lançado]: https://github.com/daniloaguiarbr/whisper-macos-cli/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/daniloaguiarbr/whisper-macos-cli/releases/tag/v0.1.0

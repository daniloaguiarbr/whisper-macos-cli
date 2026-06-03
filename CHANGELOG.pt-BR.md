[English version](CHANGELOG.md) | [Versão em Português Brasileiro](CHANGELOG.pt-BR.md)

# Changelog

Todas as mudanças notáveis neste projeto são documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/),
e este projeto adota [Semantic Versioning](https://semver.org/lang/pt-BR/spec/v2.0.0.html).

## [Não Lançado]

### Documentação

- Documentação bilíngue completa revisada e validada contra o
  framework `rules_rust_documentation_framework`
- Adicionado `THIRD-PARTY-LICENSES.md` para publicação no crates.io
- Adicionado `MIGRATION.md` cobrindo a transição 0.1.0 → 0.1.2
- Adicionados Q&A de vídeo e ffmpeg no `FAQ.md`
- Adicionada receita de vídeo e ffmpeg no `COOKBOOK.md`
- Adicionadas seções de subprocesso ffmpeg em `SECURITY.md` e
  `PRIVACY.md`
- Adicionadas seções para as novas variantes de erro
  `VideoExtractionFailed`, `FfmpegNotFound`, `UnsupportedVideoFormat`
  em `TROUBLESHOOTING.md`
- Schemas JSON bumpados para `$id` v0.1.2 com novos campos
  `ffmpeg_binary` e `no_ffmpeg_fallback` em `transcribe-input`
- `Cargo.toml` migrado de `include` para `exclude` (allowlist
  invertida conforme o framework)
- Badge do Contributor Covenant adicionado em `CODE_OF_CONDUCT.md`
- Todas as referências a MCP removidas conforme política do projeto
- `llms.txt` e `llms-full.txt` atualizados com links para
  `PRIVACY.md` e `docs/VIDEO-EXTRACTION.md`
- Estrutura AIDA híbrida adicionada em `docs/AGENTS.md`
  (Why/Economy/Sovereignty)

## [0.1.2] - 2026-06-02

### Adicionado

- Suporte a containers de vídeo: MP4, MOV, M4V, MKV, WebM, AVI, M4A
- Extração automática da trilha de áudio via subprocesso ffmpeg
- `Error::VideoExtractionFailed` (saída 65) quando ffmpeg falha
- `Error::FfmpegNotFound` (saída 69) quando binário ffmpeg ausente
- `Error::UnsupportedVideoFormat` (saída 65) quando
  `--no-ffmpeg-fallback` está ativo
- Flag `--ffmpeg-binary <PATH>` (env: `WHISPER_FFMPEG_BINARY`)
- Flag `--no-ffmpeg-fallback` (env: `WHISPER_NO_FFMPEG_FALLBACK`)
- `docs/VIDEO-EXTRACTION.md` (inglês) e `docs/VIDEO-EXTRACTION.pt-BR.md`
- Wrapper de subprocesso ffmpeg com trait `FfmpegRunner`,
  `RealFfmpeg` e `MockFfmpeg`
- 17 testes unitários em `src/video/mod.rs` para detecção de magic
  bytes
- 23 testes unitários em `src/video/ffmpeg.rs` para endurecimento
  do subprocesso
- 12 testes de integração em `tests/video_extraction.rs`

### Corrigido

- Falha de decode OGG/Opus para mensagens de voz do WhatsApp
  (symphonia Issue #8): fallback transparente para ffmpeg quando
  o decode nativo falha, com captura completa de erro e timeout
  limitado

### Alterado

- `decode_file` agora aceita runner ffmpeg opcional e flag de
  auto-fallback
- Comportamento padrão: fallback ffmpeg fica ativo mas só dispara
  em falha real de decode
- Enum `Error` marcado como `#[non_exhaustive]` para evolução estável
- Campo `Error::VideoExtractionFailed` renomeado de `source` para
  `path` para evitar conflito com semântica `source` do thiserror

### Segurança

- Subprocesso ffmpeg roda com `env_clear()` mais allowlist mínima
  (`PATH`, `HOME`, `TMPDIR`, `LANG`, `LC_ALL`) para prevenir vazamento
  de segredos
- Processo filho envolvido em `SafeChild` com semântica kill-on-drop;
  sem processos zumbi ffmpeg em panic
- Unix: filho roda em grupo de processo próprio via `setsid()` para
  isolar SIGINT do pai
- Windows: `CREATE_NEW_PROCESS_GROUP` para o mesmo isolamento
- Arquivos WAV temporários limpos via guard `Drop` mesmo em panic
- Magic bytes validados ANTES de invocar ffmpeg para recusar
  arquivos não-vídeo renomeados
- Timeout limitado (180s padrão) previne travamentos infinitos

## [0.1.1] - 2026-06-02

### Alterado

- Licença alterada de MIT-only para dual MIT OR Apache-2.0
- `LICENSE-MIT` e `LICENSE-APACHE` substituem o arquivo único `LICENSE`
- Campo `license` em `Cargo.toml` agora é `MIT OR Apache-2.0`

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

[Não Lançado]: https://github.com/daniloaguiarbr/whisper-macos-cli/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/daniloaguiarbr/whisper-macos-cli/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/daniloaguiarbr/whisper-macos-cli/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/daniloaguiarbr/whisper-macos-cli/releases/tag/v0.1.0

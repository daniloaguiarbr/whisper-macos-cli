[English version](docs/FAQ.md) | [Versão em Português Brasileiro](docs/FAQ.pt-BR.md)

# Perguntas Frequentes

## O que é whisper-macos-cli?

Uma CLI Rust que transcreve arquivos de áudio localmente em macOS
Apple Silicon usando whisper.cpp com aceleração Metal GPU. É
projetada para integração com agentes de IA e pipelines Unix via
um contrato estrito stdin/stdout JSON.

## Por que exclusiva para macOS?

O backend GPU Metal do whisper.cpp requer o framework Metal da Apple,
que está disponível apenas no macOS. Suporte multiplataforma não é
um objetivo. Para transcrição multiplataforma, use os projetos
originais [whisper.cpp](https://github.com/ggml-org/whisper.cpp) ou
[faster-whisper](https://github.com/SYSTRAN/faster-whisper).

## Por que o modelo padrão é large-v3?

Qualidade. O modelo `large-v3` produz as transcrições mais precisas,
especialmente para idiomas não-inglês. O primeiro download é ~3 GB;
execuções subsequentes usam o arquivo em cache.

## Posso usar isso para mensagens de voz do WhatsApp?

Sim. Mensagens de voz do WhatsApp são codificadas como OGG/Opus. A
CLI lida com elas nativamente e descarta o pre-skip de 80 ms
automaticamente.

## Funciona offline?

Sim, após o modelo ser baixado. Use `--offline` para pular
verificações de rede.

## Faz phone home?

Não. A única atividade de rede é o download do modelo do
huggingface.co. Veja `PRIVACY.md` para a política completa.

## Por que JSON no stdout?

JSON é a língua franca dos agentes de IA. Ao emitir JSON estruturado
com um esquema estável e `correlation_id`, agentes podem fazer
parse de resultados confiavelmente e rastrear requisições entre
serviços.

## Como atualizo o modelo?

```bash
whisper-macos-cli models remove large-v3
whisper-macos-cli models download large-v3
```

## Posso rodar múltiplos modelos em paralelo?

A CLI carrega um único modelo por processo. Rode múltiplas instâncias
da CLI em paralelo para fluxos de trabalho multi-modelo. A flag
`--concurrency` controla transcrições paralelas dentro de um único
modelo.

## Quão preciso é?

Para Português (pt-BR) e Inglês, a acurácia é comparável ao Whisper
large-v3 da OpenAI com WER tipicamente abaixo de 5% em áudio limpo.

## E a privacidade do meu áudio?

Áudio é processado inteiramente na sua máquina local. Nada é
transmitido para nenhum serviço externo. Veja `PRIVACY.pt-BR.md`.

## Como reporto um bug?

Abra uma issue em
https://github.com/daniloaguiarbr/whisper-macos-cli/issues usando
o template de bug report.

## Onde reporto uma vulnerabilidade de segurança?

Através de GitHub Security Advisories em
https://github.com/daniloaguiarbr/whisper-macos-cli/security/advisories/new
— NÃO como issue pública.

## Posso transcrever arquivos de vídeo?

Sim, desde a v0.1.2. O whisper-macos-cli suporta containers
MP4, MOV, M4V, MKV, WebM, AVI, FLV e WMV/WMA. A trilha de áudio
é extraída via subprocesso ffmpeg antes de ser alimentada ao
pipeline regular do whisper.cpp. Veja
[VIDEO-EXTRACTION.pt-BR.md](VIDEO-EXTRACTION.pt-BR.md) para
detalhes.

## Preciso do ffmpeg?

Para arquivos somente de áudio (MP3, WAV, FLAC, OGG/Vorbis,
OGG/Opus, AAC), não. Para arquivos de vídeo, sim — instale via
`brew install ffmpeg`. Para mensagens de voz OGG/Opus do
WhatsApp, o ffmpeg é opcional: a CLI tenta primeiro o decoder
nativo do symphonia e só cai para o ffmpeg se o decode nativo
falhar (bug upstream Issue #8 do symphonia).

## Como instalar o ffmpeg?

- macOS: `brew install ffmpeg`
- Ubuntu/Debian: `sudo apt-get install ffmpeg`
- Windows (Chocolatey): `choco install ffmpeg`
- Windows (winget): `winget install Gyan.FFmpeg`

## O que acontece se o ffmpeg não estiver instalado?

A CLI retorna código de saída 69 com mensagem de erro clara e
dica para instalar o ffmpeg. Para arquivos somente de áudio, o
ffmpeg não é necessário a menos que o decoder nativo OGG/Opus
falhe.

## Por que meu arquivo OGG/Opus falha com código de saída 65?

Este era um bug conhecido em v0.1.0 e v0.1.1 causado pelo codec
Opus do symphonia estar incompleto upstream (Issue #8, status
"In work"). Desde a v0.1.2, a CLI automaticamente cai para o
ffmpeg quando o decode nativo falha. Se você ainda vir este
erro, garanta que o ffmpeg está instalado e não está bloqueado
por `--no-ffmpeg-fallback`.

## Posso desabilitar o fallback do ffmpeg?

Sim, passe `--no-ffmpeg-fallback` ao subcomando `transcribe`.
Isso é útil para reproduzir bugs do decoder nativo. A flag
também está disponível como variável de ambiente
`WHISPER_NO_FFMPEG_FALLBACK`.

## Posso usar um caminho customizado para o ffmpeg?

Sim, passe `--ffmpeg-binary <PATH>` ou defina a variável de
ambiente `WHISPER_FFMPEG_BINARY`.


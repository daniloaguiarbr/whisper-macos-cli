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

[English version](docs/TROUBLESHOOTING.md) | [Versão em Português Brasileiro](docs/TROUBLESHOOTING.pt-BR.md)

# Solução de Problemas

## código de saída 64 — sem entrada

Você não passou nenhum arquivo e stdin é um TTY.

Correção: passe um argumento de arquivo ou faça pipe de stdin.

```bash
whisper-macos-cli transcribe audio.ogg
cat audio.ogg | whisper-macos-cli transcribe
```

## código de saída 65 — dados de áudio inválidos

O arquivo de áudio está corrompido, criptografado ou usa codec não
suportado.

Correção: verifique se o arquivo toca em um player de mídia, depois
re-exporte como WAV descompactado ou OGG/Opus padrão.

## código de saída 66 — arquivo de entrada não encontrado

O caminho que você forneceu não existe ou não é legível.

Correção: verifique o caminho. Use `ls` para confirmar que o arquivo
está presente.

## código de saída 69 — serviço indisponível

O download do modelo falhou ou você está em uma plataforma não
suportada.

Correção:

1. Rode `whisper-macos-cli doctor` para ver o que está errado
2. Verifique sua conexão de rede
3. Confirme que está em macOS com Apple Silicon

## código de saída 70 — inferência do whisper falhou

O modelo Whisper encontrou um erro interno.

Correção: tente um modelo menor com `--model base`.

## código de saída 74 — erro de I/O

Falha de I/O de baixo nível ocorreu.

Correção: verifique espaço em disco, permissões de arquivo e que
nenhum outro processo mantém lock exclusivo no arquivo alvo.

## código de saída 78 — erro de configuração

O modelo não está baixado ou a configuração é inválida.

Correção: rode `whisper-macos-cli models download` para instalar o
modelo.

## Decode de Áudio

Se você ver `audio decode failed: probe failed`, o arquivo pode
estar criptografado (DRM) ou usar um codec que o decoder não
reconhece. Rode `whisper-macos-cli doctor` e verifique a lista de
formatos de áudio.

## Download de Modelo

Se um download de modelo for interrompido, o temp file (`.bin.tmp`)
é automaticamente limpo. Rode `whisper-macos-cli models download`
para retentar. Até 3 tentativas com backoff exponencial são
executadas.

## Latência de Inferência

O modelo padrão `large-v3` requer aproximadamente 2 GB de memória
unificada. Em M1, a primeira inferência leva 2-5 segundos para
warmup; inferências subsequentes são mais rápidas. Use
`--model small` para 5x speedup ao custo de acurácia.

## Modo Air-Gapped

Quando o acesso à rede está indisponível, pré-baixar modelos e rodar
com `--offline` para pular a verificação de rede. O comando doctor
reportará rede como `fail` mas a CLI ainda funcionará para
transcrição local.

## SIGINT Durante Transcrição Longa

Pressionar Ctrl+C duas vezes em 1.5 segundos força saída imediata.
O primeiro Ctrl+C permite que trabalho em voo complete de forma
limpa.

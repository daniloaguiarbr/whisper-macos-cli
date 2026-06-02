[English version](PRIVACY.md) | [Versão em Português Brasileiro](PRIVACY.pt-BR.md)

# Política de Privacidade

## Resumo

whisper-macos-cli é uma CLI EXCLUSIVAMENTE LOCAL. Não coleta,
transmite nem armazena dados pessoais em servidores remotos.

## Quais Dados São Processados

A CLI processa os seguintes dados apenas localmente:

- Arquivos de áudio fornecidos como argumentos ou via stdin
- Texto transcrito gerado pelo modelo Whisper local
- Metadados de diagnóstico (nome do modelo, duração, idioma, tempo
  de processamento) incluídos no JSON de saída

## Quais Dados São Armazenados

- Modelos Whisper baixados ficam em
  `~/Library/Application Support/whisper-macos-cli/models/`
- Gravações de áudio não são retidas após a transcrição completar
- Transcrições não são retidas a menos que o usuário redirecione
  stdout para um arquivo

## Atividade de Rede

A CLI conecta a `huggingface.co` APENAS para baixar modelos
Whisper no primeiro uso. Cada requisição inclui:

- User-Agent identificando whisper-macos-cli e sua versão
- Cabeçalhos HTTPS padrão
- URL alvo do arquivo de modelo

Sem cookies, sem pixels de rastreamento, sem analytics, sem
telemetria.

## O Que NÃO É Coletado

- Sem analytics de uso
- Sem relatórios de crash
- Sem informações pessoalmente identificáveis
- Sem conteúdo de áudio
- Sem conteúdo de transcrição
- Sem endereços IP armazenados ou logados

## Controle do Usuário

Você pode verificar o comportamento de rede usando o subcomando
`doctor`, que sonda conectividade ao Hugging Face e reporta o
resultado.

Você pode rodar a CLI em modo air-gapped pré-baixando modelos e
usando `--offline` para pular verificações de rede.

## Pesos do Modelo

Os pesos do modelo Whisper são baixados de
`huggingface.co/ggerganov/whisper.cpp`. Esses pesos são licenciados
por seus autores originais (OpenAI). A CLI não modifica nem
reempacota os pesos.

## LGPD

Esta CLI não processa dados pessoais de residentes do Brasil porque
não transmite nenhum dado para servidores remotos. Se você
transcrever áudio contendo dados pessoais, você é responsável pelo
fundamento legal do processamento sob a lei aplicável.

## Contato

Para questões relacionadas a privacidade, abra uma issue em
https://github.com/daniloaguiarbr/whisper-macos-cli/issues.

## Mudanças nesta Política

Mudanças nesta política serão rastreadas em `CHANGELOG.pt-BR.md` e
anunciadas nas release notes.
